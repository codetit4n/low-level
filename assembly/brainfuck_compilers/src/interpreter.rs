use brainfuck_compilers::{parse, Error, Inst, InstKind, ARRAY_LEN, EOF};

use std::env;
use std::io;
use std::io::{Bytes, Read, Write};

fn run<R: Read, W: Write>(
    insts: &[Inst],
    mut input: Bytes<R>,
    output: &mut W,
) -> Result<(), Error> {
    let mut memory = [0_u8; ARRAY_LEN];
    let mut ptr = 0;
    let mut inst_ptr = 0;

    while inst_ptr < insts.len() {
        let Inst { kind, times, .. } = &insts[inst_ptr];
        match kind {
            InstKind::IncPtr => {
                ptr += times;
                if ptr >= ARRAY_LEN {
                    return Err(Error::PtrAboveLimit);
                }
                inst_ptr += 1;
            }
            InstKind::DecPtr => {
                if *times > ptr {
                    return Err(Error::PtrBelowZero);
                }
                ptr -= times;
                inst_ptr += 1;
            }
            InstKind::IncByte => {
                memory[ptr] = memory[ptr].wrapping_add(*times as u8);
                inst_ptr += 1;
            }
            InstKind::DecByte => {
                memory[ptr] = memory[ptr].wrapping_sub(*times as u8);
                inst_ptr += 1;
            }
            InstKind::WriteByte => {
                for _ in 0..*times {
                    let result = output.write(&[memory[ptr]]);
                    match result {
                        Ok(bytes_written) if bytes_written < 1 => {
                            panic!("failed to write byte {} to output", memory[ptr]);
                        }
                        Err(error) => {
                            panic!("failed to write byte to put with error {}", error);
                        }
                        Ok(_) => (), // everything went fine
                    }
                }
                inst_ptr += 1;
            }
            InstKind::ReadByte => {
                // before asking user for some input we have to make
                // sure they've seen our prompt / output
                if let Err(error) = output.flush() {
                    panic!("error while flushing to output: {}", error);
                }
                for _ in 0..*times {
                    let maybe_byte = input.next();
                    match maybe_byte {
                        Some(Ok(byte)) => {
                            memory[ptr] = byte;
                        }
                        None => {
                            // received EOF
                            memory[ptr] = EOF;
                        }
                        Some(Err(error)) => {
                            panic!("error while trying to read byte from input {}", error);
                        }
                    }
                }
                inst_ptr += 1;
            }
            InstKind::LoopStart { end_idx } => {
                if memory[ptr] == 0 {
                    inst_ptr = *end_idx;
                } else {
                    inst_ptr += 1;
                }
            }
            InstKind::LoopEnd { start_idx } => {
                if memory[ptr] != 0 {
                    inst_ptr = *start_idx;
                } else {
                    inst_ptr += 1;
                }
            }
        }
    }

    Ok(())
}

fn parse_and_run<R: Read, W: Write>(
    src: &str,
    input: Bytes<R>,
    output: &mut W,
) -> Result<(), Error> {
    let insts = parse(src)?;
    run(&insts, input, output)
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

    parse_and_run(&src, input, &mut output)?;
    output.flush()?;

    Ok(())
}
