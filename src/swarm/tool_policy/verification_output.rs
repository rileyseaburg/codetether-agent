//! Calibration of unsupported behavioral-preservation summaries.

use super::VerificationContract;

pub(crate) fn for_instruction(instruction: &str, output: &str) -> String {
    calibrate(VerificationContract::from_instruction(instruction), output)
}

pub(crate) fn calibrate(contract: VerificationContract, output: &str) -> String {
    if contract != VerificationContract::StaticOnly
        || !super::verification_language::preserves_behavior(&output.to_ascii_lowercase())
    {
        return output.to_string();
    }
    output
        .lines()
        .map(calibrate_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn calibrate_line(line: &str) -> &str {
    if super::verification_language::preserves_behavior(&line.to_ascii_lowercase()) {
        "Static/local: source inspection only; behavioral preservation was not verified because execution was prohibited."
    } else {
        line
    }
}
