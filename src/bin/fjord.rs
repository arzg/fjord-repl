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

        let parser = Parser::new(&input);
        let parse_output = parser.parse();

        println!("{}", parse_output.debug_tree());

        if let Some(parse_output) = parse_output.clone().into_no_errors() {
            println!("No errors were found. Evaluating...");

            let eval_result = parse_output.eval(&mut env);
            dbg!(&eval_result);
        } else {
            println!("Errors:");

            for error in parse_output.errors() {
                dbg!(error);
            }

            println!("Syntax errors were found. Skipping evaluation.");
        }
    }
}
