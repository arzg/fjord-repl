use std::{
    io::{self, Write},
    path::Path,
};

const PATH: [&str; 5] = ["/usr/local/bin", "/usr/bin", "/bin", "/usr/sbin", "/sbin"];

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let commands = fjord::Commands::default();
    commands.rescan(PATH.iter().map(Path::new))?;

    let mut state = fjord::eval::State::new_root(&commands);

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        let mut s = String::new();
        stdin.read_line(&mut s)?;

        let eval_result = fjord::eval(s.trim(), &mut state);

        match eval_result {
            Ok(output) => writeln!(stdout, "{}", output)?,
            Err(e) => writeln!(stderr, "Error: {:?}", anyhow::Error::new(e))?,
        }
    }
}
