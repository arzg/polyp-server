use jsonl::Connection;
use polyp::{ProcessletMsg, ServerMsg, Ui, UserInput};
use std::process::{Command, Stdio};

fn main() -> anyhow::Result<()> {
    let mut kon = Command::new("kon-polyp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut client_connection = Connection::new_from_stdio();
    let mut kon_connection = Connection::new_from_child(&mut kon).unwrap();

    loop {
        let server_msg: ServerMsg = client_connection.read()?;
        eprintln!("polyp-server: got server message {:?}\r", server_msg);

        let processlet_msg = match server_msg {
            ServerMsg::UserInput(UserInput::PressedKey(key)) => {
                ProcessletMsg::UserInput(UserInput::PressedKey(key))
            }
            ServerMsg::Shutdown => {
                eprintln!("polyp-server: shutting down...\r");
                kon_connection.write(&ProcessletMsg::Shutdown)?;
                kon.wait()?;

                return Ok(());
            }
        };

        kon_connection.write(&processlet_msg)?;
        eprintln!("polyp-server: forwarded user input\r");

        let ui: Ui = kon_connection.read()?;
        eprintln!("polyp-server: received UI {:?}\r", ui);

        client_connection.write(&ui)?;
        eprintln!("polyp-server: forwarded UI\r");
    }
}
