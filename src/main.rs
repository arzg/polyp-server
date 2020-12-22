use polyp::{Ui, UserInput};
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() -> anyhow::Result<()> {
    let kon = Command::new("kon-polyp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut kon_stdin = kon.stdin.unwrap();
    let mut kon_stdout = BufReader::new(kon.stdout.unwrap());

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let user_input: UserInput = serde_json::from_str(&line)?;
        eprintln!("polyp-server: got user input {:?}\r", user_input);

        serde_json::to_writer(&kon_stdin, &user_input)?;
        kon_stdin.write_all(b"\n")?;
        eprintln!("polyp-server: forwarded user input\r");

        let mut line = String::new();
        kon_stdout.read_line(&mut line)?;
        let ui: Ui = serde_json::from_str(&line)?;
        eprintln!("polyp-server: received UI {:?}\r", ui);

        io::stdout().write_all(&serde_json::to_vec(&ui)?)?;
        io::stdout().write_all(b"\n")?;
        eprintln!("polyp-server: forwarded UI\r");
    }
}
