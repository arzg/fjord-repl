use fjord::env::Env;
use fjord::parser::Parser;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut env = Env::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        process_chunk(&input, &mut env, &mut stdout)?;
    }
}

fn process_chunk(text: &str, env: &mut Env, stdout: &mut io::Stdout) -> io::Result<()> {
    let parser = Parser::new(text);
    let parse_output = parser.parse();

    println!("{}", parse_output.debug_tree());

    if let Some(parse_output) = parse_output.clone().into_no_errors() {
        writeln!(stdout, "No errors were found. Evaluating...")?;

        let eval_result = parse_output.eval(env);
        dbg!(&eval_result);
    } else {
        writeln!(stdout, "Errors:")?;

        for error in parse_output.errors() {
            dbg!(error);
        }

        writeln!(stdout, "Syntax errors were found. Skipping evaluation.")?;
    }

    Ok(())
}
