use polyp::{ProcessletMsg, ServerMsg};
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();

    let mut ls_processlet = Command::new("ls-polyp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let ls_stdin = ls_processlet.stdin.as_mut().unwrap();
    ls_stdin.write_all(b"/Users/aramis")?;
    ls_stdin.flush()?;

    let ls_stdout = ls_processlet.wait_with_output()?.stdout;

    let ls_processlet_msg: ProcessletMsg = serde_json::from_slice(&ls_stdout)?;

    let server_msg = match ls_processlet_msg {
        ProcessletMsg::NewOutput(new_output) => ServerMsg::NewText(format!("{:#?}", new_output)),
    };

    let serialized_server_msg = serde_json::to_string(&server_msg)?;
    stdout.write_all(serialized_server_msg.as_bytes())?;

    stdout.flush()?;

    Ok(())
}
