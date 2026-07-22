const AGENT = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/131.0.0.0 Safari/537.36";

export async function scrape(cookie) {
  const response = await fetch("https://gemini.google.com", {
    redirect: "follow", headers: { Cookie: cookie, "User-Agent": AGENT },
  });
  const html = await response.text();
  const token = match(html, /"thykhd":"([^"]+)"/) ?? match(html, /"SNlM0e":"([^"]+)"/);
  if (!token) throw new Error("Gemini anti-CSRF token was not found");
  return {
    at: token,
    bl: match(html, /"cfb2h":"([^"]+)"/) ?? "",
    sid: match(html, /"FdrFJe":"([^"]+)"/) ?? "",
    prefix: new URL(response.url).pathname.match(/(\/u\/\d+)\//)?.[1] ?? "",
    agent: AGENT,
  };
}

function match(text, pattern) {
  return text.match(pattern)?.[1];
}
