use brainfuck_compilers::Error;

use std::env;
use std::io;
use std::io::{Bytes, Read, Write};

fn parse_and_run<R: Read, W: Write>(
    src: &str,
    input: Bytes<R>,
    output: &mut W,
) -> Result<(), Error> {
    //let insts = parse(src)?;
    //run(&insts, input, output)
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: <path to brainfuck file>");
        std::process::exit(1);
    }

    let src = std::fs::read_to_string(&args[1])?;
    let stdin = io::stdin();
    let input = stdin.lock().bytes();
    let stdout = io::stdout();
    let mut output = stdout.lock();

    Ok(())
}
