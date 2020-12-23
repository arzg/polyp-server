use polyp::protocol::Connection;
use polyp::{ProcessletMsg, ServerMsg, Ui, UserInput};
use std::process::{Command, Stdio};

fn main() -> anyhow::Result<()> {
    let kon = Command::new("kon-polyp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut client_connection = Connection::new_from_current_process();
    let mut kon_connection = Connection::new_from_child(kon).unwrap();

    loop {
        let server_msg: ServerMsg = client_connection.recv_message()?;
        eprintln!("polyp-server: got server message {:?}\r", server_msg);

        let processlet_msg = match server_msg {
            ServerMsg::UserInput(UserInput::PressedKey(key)) => {
                ProcessletMsg::UserInput(UserInput::PressedKey(key))
            }
            ServerMsg::Shutdown => {
                eprintln!("polyp-server: shutting down...\r");
                kon_connection.send_message(&ProcessletMsg::Shutdown)?;
                return Ok(());
            }
        };

        kon_connection.send_message(&processlet_msg)?;
        eprintln!("polyp-server: forwarded user input\r");

        let ui: Ui = kon_connection.recv_message()?;
        eprintln!("polyp-server: received UI {:?}\r", ui);

        client_connection.send_message(&ui)?;
        eprintln!("polyp-server: forwarded UI\r");
    }
}
