// TODO(chromiumoxide): store the CDP handler JoinHandle and cancellation
// token here when the real session runtime lands.

pub async fn execute(
    command: crate::browser::BrowserCommand,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    use crate::browser::BrowserCommand as Command;
    match command {
        Command::Health => err(),
        Command::Start(request) => todo_err(request),
        Command::Stop => err(),
        Command::Snapshot => err(),
        Command::Console => err(),
        Command::Goto(request) => todo_err(request),
        Command::Back => err(),
        Command::Reload => err(),
        Command::Wait(request) => todo_err(request),
        Command::Click(request) => todo_err(request),
        Command::Fill(request) => todo_err(request),
        Command::Type(request) => todo_err(request),
        Command::Press(request) => todo_err(request),
        Command::Text(request) => todo_err(request),
        Command::Html(request) => todo_err(request),
        Command::Eval(request) => todo_err(request),
        Command::ConsoleEval(request) => todo_err(request),
        Command::ClickText(request) => todo_err(request),
        Command::FillNative(request) => todo_err(request),
        Command::Toggle(request) => todo_err(request),
        Command::Screenshot(request) => todo_err(request),
        Command::MouseClick(request) => todo_err(request),
        Command::KeyboardType(request) => todo_err(request),
        Command::KeyboardPress(request) => todo_err(request),
        Command::Tabs => err(),
        Command::TabsSelect(request) => todo_err(request),
        Command::TabsNew(request) => todo_err(request),
        Command::TabsClose(request) => todo_err(request),
    }
}

fn err() -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    Err(crate::browser::BrowserError::NotImplemented)
}

fn todo_err<T>(request: T) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    let _ = request;
    err()
}
