//! tester relay role prompt and capabilities.
use std::path::Path;
pub(super) fn profile(working_dir: &Path) -> (String, String, Vec<String>) {
    let wd = working_dir.display();
    (
        "auto-tester".to_string(),
        format!(
            "You are @auto-tester.\n\
                     Specialty: Test writing and verification.\n\
                     Mission: Write tests for the implemented changes and ensure they pass.\n\
                     Working directory: {wd}\n\n\
                     Review the implementation, write appropriate tests, run them with `bash`.\n\
                     Report test results and coverage gaps."
        ),
        vec!["testing".into(), "relay".into()],
    )
}
