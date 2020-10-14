use anyhow::{anyhow, Context};
use polyp::{ServerMsg, Ui, UserInput};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::process::Command;
use tungstenite::Message;
use url::Url;

fn main() -> anyhow::Result<()> {
    let (mut client_websocket, _) = {
        let client_socket_addr = env::args()
            .nth(1)
            .ok_or_else(|| anyhow!("expected argument of client socket address"))?;

        let client_websocket_url = Url::parse(&format!("ws://{}", client_socket_addr))?;

        tungstenite::client::connect(client_websocket_url).context("server")?
    };

    println!("polyp-server: connected to client\r");

    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 16180);

    Command::new("kon-polyp")
        .arg(socket_addr.to_string())
        .spawn()?;

    let mut processlet_websocket = {
        let listener = TcpListener::bind(socket_addr)?;
        let stream = listener.incoming().next().unwrap()?;

        tungstenite::server::accept(stream)?
    };
    println!("polyp-server: connected to processlet\r");

    loop {
        let user_input = match client_websocket.read_message()? {
            Message::Binary(json) => serde_json::from_slice(&json)?,
            Message::Close(_) => {
                processlet_websocket.close(None)?;
                return Ok(());
            }
            _ => unreachable!(),
        };

        let UserInput::PressedKey(c) = user_input;
        println!("polyp-server: client says user pressed ‘{:?}’\r", c);

        processlet_websocket.write_message(Message::Binary(serde_json::to_vec(
            &UserInput::PressedKey(c),
        )?))?;
        println!("polyp-server: forwarded on user input to processlet\r");

        let processlet_msg = processlet_websocket.read_message()?;
        println!("polyp-server: read UI from processlet\r");

        if let Message::Binary(json) = processlet_msg {
            let processlet_ui: Ui = serde_json::from_slice(&json)?;

            let server_msg = match processlet_ui {
                Ui::Value(value) => ServerMsg::NewText(format!("{:#?}", value)),
                Ui::TextField { current_text } => ServerMsg::NewText(current_text),
            };

            let serialized_server_msg = serde_json::to_vec(&server_msg)?;

            client_websocket.write_message(Message::Binary(serialized_server_msg))?;
            println!("polyp-server: sent new text to client\r");
        } else {
            unreachable!()
        }
    }
}
