use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut state = fjord::eval::State::new_root();

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
