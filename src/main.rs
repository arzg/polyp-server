use polyp::protocol::Connection;
use polyp::{Ui, UserInput};
use std::process::{Command, Stdio};

fn main() -> anyhow::Result<()> {
    let kon = Command::new("kon-polyp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut client_connection = Connection::new_from_current_process();
    let mut kon_connection = Connection::new_from_child(kon).unwrap();

    loop {
        let user_input: UserInput = client_connection.recv_message()?;
        eprintln!("polyp-server: got user input {:?}\r", user_input);

        kon_connection.send_message(&user_input)?;
        eprintln!("polyp-server: forwarded user input\r");

        let ui: Ui = kon_connection.recv_message()?;
        eprintln!("polyp-server: received UI {:?}\r", ui);

        client_connection.send_message(&ui)?;
        eprintln!("polyp-server: forwarded UI\r");
    }
}
