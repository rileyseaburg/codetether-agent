//! Minimal blocking SMTP client with DKIM signing.
//!
//! Exposes `smtp_send(host, port, from, to, subject, body)` to TetherScript scripts.
//! Messages are DKIM-signed by default using signer settings from the host
//! environment:
//! - `TETHERSCRIPT_DKIM_SELECTOR` (required; `tetherscript_DKIM_SELECTOR` still works)
//! - `TETHERSCRIPT_DKIM_PRIVATE_KEY_PEM` or `TETHERSCRIPT_DKIM_PRIVATE_KEY_FILE`
//!   (required; legacy `tetherscript_DKIM_*` names still work)
//! - `TETHERSCRIPT_DKIM_DOMAIN` (optional; defaults to the domain in `from`)
//!
//! This is intentionally narrow:
//! - plain TCP SMTP only
//! - no TLS / STARTTLS
//! - no AUTH
//! - enough for localhost relays and test harnesses

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::rc::Rc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::system::{base64_decode_bytes, base64_encode_bytes, sha256};
use crate::value::Value;

const IO_TIMEOUT: Duration = Duration::from_secs(30);

pub fn send(
    host: &Value,
    port: &Value,
    from: &Value,
    to: &Value,
    subject: &Value,
    body: &Value,
) -> Result<Value, String> {
    let host = expect_str("host", host)?;
    let port = expect_port(port)?;
    let from = expect_header_line("from", from)?;
    let recipients = expect_recipients(to)?;
    let subject = expect_header_line("subject", subject)?;
    let body = expect_body(body)?;
    let dkim = load_dkim_config(&from)?;
    let date = format_rfc2822_utc(SystemTime::now());
    let message_id = make_message_id(&dkim.domain);
    let dkim_header = dkim_signature(
        &dkim,
        &from,
        &recipients,
        &subject,
        &date,
        &message_id,
        &body,
    )?;

    let mut stream = TcpStream::connect((host.as_str(), port))
        .map_err(|e| format!("smtp_send: connect to {}:{} failed: {}", host, port, e))?;
    stream
        .set_read_timeout(Some(IO_TIMEOUT))
        .map_err(|e| format!("smtp_send: set read timeout failed: {}", e))?;
    stream
        .set_write_timeout(Some(IO_TIMEOUT))
        .map_err(|e| format!("smtp_send: set write timeout failed: {}", e))?;
    let mut reader = BufReader::new(
        stream
            .try_clone()
            .map_err(|e| format!("smtp_send: clone stream failed: {}", e))?,
    );

    expect_reply("greeting", read_reply(&mut reader)?, &[220])?;

    let hello = send_command(&mut stream, &mut reader, "EHLO tetherscript")?;
    if hello.code != 250 {
        if matches!(hello.code, 500 | 502 | 504) {
            expect_reply(
                "HELO",
                send_command(&mut stream, &mut reader, "HELO tetherscript")?,
                &[250],
            )?;
        } else {
            return Err(format!(
                "smtp_send: EHLO failed: {} {}",
                hello.code, hello.message
            ));
        }
    }

    expect_reply(
        "MAIL FROM",
        send_command(&mut stream, &mut reader, &format!("MAIL FROM:<{}>", from))?,
        &[250],
    )?;

    for rcpt in &recipients {
        expect_reply(
            "RCPT TO",
            send_command(&mut stream, &mut reader, &format!("RCPT TO:<{}>", rcpt))?,
            &[250, 251],
        )?;
    }

    expect_reply(
        "DATA",
        send_command(&mut stream, &mut reader, "DATA")?,
        &[354],
    )?;
    write_message(
        &mut stream,
        &from,
        &recipients,
        &subject,
        &date,
        &message_id,
        &dkim_header,
        &body,
    )?;
    let queued = expect_reply("message body", read_reply(&mut reader)?, &[250])?;

    let _ = send_command(&mut stream, &mut reader, "QUIT");
    Ok(reply_value(queued))
}

struct Reply {
    code: u16,
    message: String,
}

struct DkimConfig {
    domain: String,
    selector: String,
    key: RsaPrivateKey,
}

fn expect_str(name: &str, value: &Value) -> Result<String, String> {
    match value {
        Value::Str(s) => Ok((**s).clone()),
        other => Err(format!(
            "smtp_send: {} must be str, got {}",
            name,
            other.type_name()
        )),
    }
}

fn expect_port(value: &Value) -> Result<u16, String> {
    match value {
        Value::Int(n) if *n > 0 && *n <= 65535 => Ok(*n as u16),
        Value::Int(n) => Err(format!("smtp_send: port {} out of range", n)),
        other => Err(format!(
            "smtp_send: port must be int, got {}",
            other.type_name()
        )),
    }
}

fn expect_header_line(name: &str, value: &Value) -> Result<String, String> {
    let s = expect_str(name, value)?;
    if s.contains('\r') || s.contains('\n') {
        return Err(format!("smtp_send: {} must not contain CR or LF", name));
    }
    if s.is_empty() {
        return Err(format!("smtp_send: {} must not be empty", name));
    }
    Ok(s)
}

fn expect_body(value: &Value) -> Result<String, String> {
    match value {
        Value::Str(s) => Ok((**s).clone()),
        other => Err(format!(
            "smtp_send: body must be str, got {}",
            other.type_name()
        )),
    }
}

fn expect_recipients(value: &Value) -> Result<Vec<String>, String> {
    match value {
        Value::Str(_) => Ok(vec![expect_header_line("to", value)?]),
        Value::List(xs) => {
            let xs = xs.borrow();
            if xs.is_empty() {
                return Err("smtp_send: to list must not be empty".into());
            }
            xs.iter().map(|v| expect_header_line("to", v)).collect()
        }
        other => Err(format!(
            "smtp_send: to must be str or list, got {}",
            other.type_name()
        )),
    }
}

fn load_dkim_config(from: &str) -> Result<DkimConfig, String> {
    let selector =
        dkim_env("TETHERSCRIPT_DKIM_SELECTOR", "tetherscript_DKIM_SELECTOR").map_err(|_| {
            "smtp_send: missing TETHERSCRIPT_DKIM_SELECTOR or tetherscript_DKIM_SELECTOR"
                .to_string()
        })?;
    let domain = match dkim_env("TETHERSCRIPT_DKIM_DOMAIN", "tetherscript_DKIM_DOMAIN") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => from_domain(from)?,
    };

    let key_pem = match dkim_env(
        "TETHERSCRIPT_DKIM_PRIVATE_KEY_PEM",
        "tetherscript_DKIM_PRIVATE_KEY_PEM",
    ) {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            let path = dkim_env(
                "TETHERSCRIPT_DKIM_PRIVATE_KEY_FILE",
                "tetherscript_DKIM_PRIVATE_KEY_FILE",
            )
                .map_err(|_| {
                    "smtp_send: missing DKIM key; set TETHERSCRIPT_DKIM_PRIVATE_KEY_PEM, TETHERSCRIPT_DKIM_PRIVATE_KEY_FILE, or legacy tetherscript_DKIM_* equivalents".to_string()
                })?;
            fs::read_to_string(&path)
                .map_err(|e| format!("smtp_send: read DKIM key {} failed: {}", path, e))?
        }
    };

    let key = RsaPrivateKey::from_pem(&key_pem)
        .map_err(|e| format!("smtp_send: parse DKIM private key failed: {}", e))?;

    Ok(DkimConfig {
        domain,
        selector,
        key,
    })
}

fn dkim_env(primary: &str, legacy: &str) -> Result<String, env::VarError> {
    env::var(primary).or_else(|_| env::var(legacy))
}

fn from_domain(from: &str) -> Result<String, String> {
    let domain = from
        .rsplit_once('@')
        .map(|(_, domain)| domain.trim())
        .filter(|domain| !domain.is_empty())
        .ok_or_else(|| "smtp_send: could not infer DKIM domain from from-address".to_string())?;
    Ok(domain.to_string())
}

fn dkim_signature(
    dkim: &DkimConfig,
    from: &str,
    recipients: &[String],
    subject: &str,
    date: &str,
    message_id: &str,
    body: &str,
) -> Result<String, String> {
    let body_hash = base64_encode_bytes(&sha256(canonicalize_body_relaxed(body).as_bytes()));
    let signed_headers = "from:to:subject:date:message-id";
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("smtp_send: clock error: {}", e))?
        .as_secs();
    let dkim_value = format!(
        "v=1; a=rsa-sha256; c=relaxed/relaxed; d={}; s={}; q=dns/txt; t={}; h={}; bh={}; b=",
        dkim.domain, dkim.selector, timestamp, signed_headers, body_hash
    );

    let mut input = String::new();
    input.push_str(&canonicalize_header_relaxed("From", from));
    input.push_str(&canonicalize_header_relaxed("To", &recipients.join(", ")));
    input.push_str(&canonicalize_header_relaxed("Subject", subject));
    input.push_str(&canonicalize_header_relaxed("Date", date));
    input.push_str(&canonicalize_header_relaxed("Message-ID", message_id));
    input.push_str(&canonicalize_header_relaxed("DKIM-Signature", &dkim_value));

    let digest = sha256(input.as_bytes());
    let sig = dkim.key.sign_sha256_pkcs1v15(&digest)?;
    Ok(format!("{}{}", dkim_value, base64_encode_bytes(&sig)))
}

fn canonicalize_header_relaxed(name: &str, value: &str) -> String {
    format!(
        "{}:{}\r\n",
        name.to_ascii_lowercase(),
        compress_wsp(value.trim())
    )
}

fn canonicalize_body_relaxed(body: &str) -> String {
    let normalized = body.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines: Vec<String> = normalized
        .split('\n')
        .map(|line| compress_wsp(line.trim_end_matches([' ', '\t'])))
        .collect();

    while matches!(lines.last(), Some(last) if last.is_empty()) {
        lines.pop();
    }

    if lines.is_empty() {
        "\r\n".to_string()
    } else {
        format!("{}\r\n", lines.join("\r\n"))
    }
}

fn compress_wsp(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_wsp = false;
    for ch in s.chars() {
        if ch == ' ' || ch == '\t' {
            if !in_wsp {
                out.push(' ');
                in_wsp = true;
            }
        } else {
            out.push(ch);
            in_wsp = false;
        }
    }
    out
}

fn make_message_id(domain: &str) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("<{}.{}@{}>", now.as_secs(), now.subsec_nanos(), domain)
}

fn format_rfc2822_utc(now: SystemTime) -> String {
    const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let seconds = duration.as_secs() as i64;
    let days = seconds.div_euclid(86_400);
    let second_of_day = seconds.rem_euclid(86_400);
    let hour = second_of_day / 3_600;
    let minute = (second_of_day % 3_600) / 60;
    let second = second_of_day % 60;
    let weekday = (days + 4).rem_euclid(7) as usize;
    let (year, month, day) = civil_from_days(days);
    format!(
        "{}, {:02} {} {:04} {:02}:{:02}:{:02} +0000",
        WEEKDAYS[weekday],
        day,
        MONTHS[(month - 1) as usize],
        year,
        hour,
        minute,
        second
    )
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

struct RsaPrivateKey {
    n: BigUint,
    d: BigUint,
    size: usize,
}

impl RsaPrivateKey {
    fn from_pem(pem: &str) -> Result<Self, String> {
        let der = pem_to_der(pem)?;
        parse_pkcs1_private_key(&der).or_else(|_| parse_pkcs8_private_key(&der))
    }

    fn sign_sha256_pkcs1v15(&self, digest: &[u8; 32]) -> Result<Vec<u8>, String> {
        const SHA256_DIGEST_INFO_PREFIX: [u8; 19] = [
            0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
            0x01, 0x05, 0x00, 0x04, 0x20,
        ];
        let digest_info_len = SHA256_DIGEST_INFO_PREFIX.len() + digest.len();
        if self.size < digest_info_len + 11 {
            return Err("smtp_send: RSA key is too small for SHA-256 DKIM signature".into());
        }

        let mut encoded = Vec::with_capacity(self.size);
        encoded.push(0x00);
        encoded.push(0x01);
        encoded.extend(std::iter::repeat_n(0xff, self.size - digest_info_len - 3));
        encoded.push(0x00);
        encoded.extend_from_slice(&SHA256_DIGEST_INFO_PREFIX);
        encoded.extend_from_slice(digest);

        let message = BigUint::from_be_bytes(&encoded);
        if message.cmp(&self.n).is_ge() {
            return Err("smtp_send: encoded DKIM signature is larger than RSA modulus".into());
        }
        let signature = message.mod_pow(&self.d, &self.n);
        Ok(signature.to_be_bytes_len(self.size))
    }
}

fn pem_to_der(pem: &str) -> Result<Vec<u8>, String> {
    let mut b64 = String::new();
    for line in pem.lines() {
        let line = line.trim();
        if !line.starts_with("-----") {
            b64.push_str(line);
        }
    }
    if b64.is_empty() {
        return Err("missing PEM body".into());
    }
    base64_decode_bytes(&b64)
}

fn parse_pkcs8_private_key(der: &[u8]) -> Result<RsaPrivateKey, String> {
    let mut seq = DerReader::new(der).read_sequence()?;
    let _version = seq.read_integer_bytes()?;
    let _algorithm = seq.read_tlv(0x30)?;
    let key = seq.read_tlv(0x04)?;
    parse_pkcs1_private_key(key)
}

fn parse_pkcs1_private_key(der: &[u8]) -> Result<RsaPrivateKey, String> {
    let mut seq = DerReader::new(der).read_sequence()?;
    let _version = seq.read_integer_bytes()?;
    let n = BigUint::from_be_bytes(&seq.read_integer_bytes()?);
    let _e = seq.read_integer_bytes()?;
    let d = BigUint::from_be_bytes(&seq.read_integer_bytes()?);
    let size = n.to_be_bytes().len();
    if n.is_zero() || d.is_zero() {
        return Err("empty RSA modulus or exponent".into());
    }
    Ok(RsaPrivateKey { n, d, size })
}

struct DerReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> DerReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn read_sequence(&mut self) -> Result<DerReader<'a>, String> {
        Ok(DerReader::new(self.read_tlv(0x30)?))
    }

    fn read_integer_bytes(&mut self) -> Result<Vec<u8>, String> {
        let bytes = self.read_tlv(0x02)?;
        let mut start = 0;
        while start + 1 < bytes.len() && bytes[start] == 0 {
            start += 1;
        }
        Ok(bytes[start..].to_vec())
    }

    fn read_tlv(&mut self, expected_tag: u8) -> Result<&'a [u8], String> {
        let tag = self
            .next()
            .ok_or_else(|| "unexpected end of DER".to_string())?;
        if tag != expected_tag {
            return Err(format!(
                "unexpected DER tag 0x{tag:02x}, expected 0x{expected_tag:02x}"
            ));
        }
        let len = self.read_len()?;
        if self.pos + len > self.data.len() {
            return Err("DER length exceeds input".into());
        }
        let out = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Ok(out)
    }

    fn read_len(&mut self) -> Result<usize, String> {
        let first = self
            .next()
            .ok_or_else(|| "unexpected end of DER length".to_string())?;
        if first & 0x80 == 0 {
            return Ok(first as usize);
        }
        let count = (first & 0x7f) as usize;
        if count == 0 || count > std::mem::size_of::<usize>() {
            return Err("unsupported DER length".into());
        }
        let mut len = 0usize;
        for _ in 0..count {
            len = (len << 8)
                | self
                    .next()
                    .ok_or_else(|| "unexpected end of DER length".to_string())?
                    as usize;
        }
        Ok(len)
    }

    fn next(&mut self) -> Option<u8> {
        let byte = self.data.get(self.pos).copied()?;
        self.pos += 1;
        Some(byte)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BigUint {
    words: Vec<u32>,
}

impl BigUint {
    fn zero() -> Self {
        Self { words: Vec::new() }
    }

    fn one() -> Self {
        Self { words: vec![1] }
    }

    fn from_be_bytes(bytes: &[u8]) -> Self {
        let mut words = Vec::new();
        let mut current = 0u32;
        let mut shift = 0;
        for byte in bytes.iter().rev() {
            current |= (*byte as u32) << shift;
            shift += 8;
            if shift == 32 {
                words.push(current);
                current = 0;
                shift = 0;
            }
        }
        if shift != 0 {
            words.push(current);
        }
        let mut out = Self { words };
        out.normalize();
        out
    }

    fn to_be_bytes(&self) -> Vec<u8> {
        if self.words.is_empty() {
            return vec![0];
        }
        let mut bytes = Vec::with_capacity(self.words.len() * 4);
        for word in self.words.iter().rev() {
            bytes.extend_from_slice(&word.to_be_bytes());
        }
        while bytes.len() > 1 && bytes[0] == 0 {
            bytes.remove(0);
        }
        bytes
    }

    fn to_be_bytes_len(&self, len: usize) -> Vec<u8> {
        let bytes = self.to_be_bytes();
        if bytes.len() >= len {
            return bytes[bytes.len() - len..].to_vec();
        }
        let mut out = vec![0; len - bytes.len()];
        out.extend(bytes);
        out
    }

    fn is_zero(&self) -> bool {
        self.words.is_empty()
    }

    fn bit_len(&self) -> usize {
        match self.words.last() {
            Some(last) => (self.words.len() - 1) * 32 + (32 - last.leading_zeros() as usize),
            None => 0,
        }
    }

    fn bit(&self, index: usize) -> bool {
        self.words
            .get(index / 32)
            .map(|word| (word & (1u32 << (index % 32))) != 0)
            .unwrap_or(false)
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.words.len() != other.words.len() {
            return self.words.len().cmp(&other.words.len());
        }
        for (a, b) in self.words.iter().rev().zip(other.words.iter().rev()) {
            if a != b {
                return a.cmp(b);
            }
        }
        std::cmp::Ordering::Equal
    }

    fn add(&self, other: &Self) -> Self {
        let max = self.words.len().max(other.words.len());
        let mut words = Vec::with_capacity(max + 1);
        let mut carry = 0u64;
        for index in 0..max {
            let a = self.words.get(index).copied().unwrap_or(0) as u64;
            let b = other.words.get(index).copied().unwrap_or(0) as u64;
            let sum = a + b + carry;
            words.push(sum as u32);
            carry = sum >> 32;
        }
        if carry != 0 {
            words.push(carry as u32);
        }
        let mut out = Self { words };
        out.normalize();
        out
    }

    fn sub(&self, other: &Self) -> Self {
        debug_assert!(self.cmp(other).is_ge());
        let mut words = Vec::with_capacity(self.words.len());
        let mut borrow = 0i64;
        for index in 0..self.words.len() {
            let a = self.words[index] as i64 - borrow;
            let b = other.words.get(index).copied().unwrap_or(0) as i64;
            if a >= b {
                words.push((a - b) as u32);
                borrow = 0;
            } else {
                words.push(((1i64 << 32) + a - b) as u32);
                borrow = 1;
            }
        }
        let mut out = Self { words };
        out.normalize();
        out
    }

    fn add_mod(&self, other: &Self, modulus: &Self) -> Self {
        let sum = self.add(other);
        if sum.cmp(modulus).is_ge() {
            sum.sub(modulus)
        } else {
            sum
        }
    }

    fn mod_mul(&self, other: &Self, modulus: &Self) -> Self {
        let mut result = BigUint::zero();
        let mut addend = self.clone();
        for index in 0..other.bit_len() {
            if other.bit(index) {
                result = result.add_mod(&addend, modulus);
            }
            addend = addend.add_mod(&addend, modulus);
        }
        result
    }

    fn mod_pow(&self, exponent: &Self, modulus: &Self) -> Self {
        let mut result = BigUint::one();
        let mut base = self.clone();
        for index in 0..exponent.bit_len() {
            if exponent.bit(index) {
                result = result.mod_mul(&base, modulus);
            }
            base = base.mod_mul(&base, modulus);
        }
        result
    }

    fn normalize(&mut self) {
        while self.words.last() == Some(&0) {
            self.words.pop();
        }
    }
}

fn send_command(
    stream: &mut TcpStream,
    reader: &mut BufReader<TcpStream>,
    cmd: &str,
) -> Result<Reply, String> {
    write!(stream, "{}\r\n", cmd).map_err(|e| format!("smtp_send: write command failed: {}", e))?;
    stream
        .flush()
        .map_err(|e| format!("smtp_send: flush command failed: {}", e))?;
    read_reply(reader)
}

#[allow(clippy::too_many_arguments)]
fn write_message(
    stream: &mut TcpStream,
    from: &str,
    recipients: &[String],
    subject: &str,
    date: &str,
    message_id: &str,
    dkim_header: &str,
    body: &str,
) -> Result<(), String> {
    write!(stream, "From: {}\r\n", from)
        .map_err(|e| format!("smtp_send: write From failed: {}", e))?;
    write!(stream, "To: {}\r\n", recipients.join(", "))
        .map_err(|e| format!("smtp_send: write To failed: {}", e))?;
    write!(stream, "Subject: {}\r\n", subject)
        .map_err(|e| format!("smtp_send: write Subject failed: {}", e))?;
    write!(stream, "Date: {}\r\n", date)
        .map_err(|e| format!("smtp_send: write Date failed: {}", e))?;
    write!(stream, "Message-ID: {}\r\n", message_id)
        .map_err(|e| format!("smtp_send: write Message-ID failed: {}", e))?;
    write!(stream, "DKIM-Signature: {}\r\n", dkim_header)
        .map_err(|e| format!("smtp_send: write DKIM-Signature failed: {}", e))?;
    write!(stream, "MIME-Version: 1.0\r\n")
        .map_err(|e| format!("smtp_send: write MIME-Version failed: {}", e))?;
    write!(stream, "Content-Type: text/plain; charset=utf-8\r\n")
        .map_err(|e| format!("smtp_send: write Content-Type failed: {}", e))?;
    write!(stream, "\r\n")
        .map_err(|e| format!("smtp_send: write header/body separator failed: {}", e))?;

    let normalized = body.replace("\r\n", "\n").replace('\r', "\n");
    for line in normalized.split('\n') {
        if let Some(rest) = line.strip_prefix('.') {
            write!(stream, "..{}\r\n", rest)
                .map_err(|e| format!("smtp_send: write dot-stuffed body failed: {}", e))?;
        } else {
            write!(stream, "{}\r\n", line)
                .map_err(|e| format!("smtp_send: write body failed: {}", e))?;
        }
    }
    write!(stream, ".\r\n")
        .map_err(|e| format!("smtp_send: write DATA terminator failed: {}", e))?;
    stream
        .flush()
        .map_err(|e| format!("smtp_send: flush message failed: {}", e))
}

fn read_reply(reader: &mut BufReader<TcpStream>) -> Result<Reply, String> {
    let mut code = None;
    let mut lines = Vec::new();

    loop {
        let mut line = String::new();
        let n = reader
            .read_line(&mut line)
            .map_err(|e| format!("smtp_send: read reply failed: {}", e))?;
        if n == 0 {
            return Err("smtp_send: unexpected EOF from SMTP server".into());
        }

        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.len() < 4 {
            return Err(format!("smtp_send: malformed SMTP reply: {}", trimmed));
        }

        let current = trimmed[..3]
            .parse::<u16>()
            .map_err(|_| format!("smtp_send: malformed SMTP status code: {}", trimmed))?;
        let sep = trimmed.as_bytes()[3];
        if sep != b' ' && sep != b'-' {
            return Err(format!(
                "smtp_send: malformed SMTP reply separator: {}",
                trimmed
            ));
        }

        if let Some(expected) = code {
            if expected != current {
                return Err(format!(
                    "smtp_send: mismatched multiline SMTP reply codes: {} then {}",
                    expected, current
                ));
            }
        } else {
            code = Some(current);
        }

        lines.push(trimmed[4..].to_string());
        if sep == b' ' {
            return Ok(Reply {
                code: current,
                message: lines.join("\n"),
            });
        }
    }
}

fn expect_reply(stage: &str, reply: Reply, allowed: &[u16]) -> Result<Reply, String> {
    if allowed.contains(&reply.code) {
        Ok(reply)
    } else {
        Err(format!(
            "smtp_send: {} failed: {} {}",
            stage, reply.code, reply.message
        ))
    }
}

fn reply_value(reply: Reply) -> Value {
    let mut map = HashMap::new();
    map.insert("code".into(), Value::Int(reply.code as i64));
    map.insert("message".into(), Value::Str(Rc::new(reply.message)));
    Value::Map(Rc::new(RefCell::new(map)))
}
