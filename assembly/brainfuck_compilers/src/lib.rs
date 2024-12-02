#![forbid(unsafe_code)]

use std::fmt;

const INC_PTR: char = '>';
const DEC_PTR: char = '<';
const INC_BYTE: char = '+';
const DEC_BYTE: char = '-';
const WRITE_BYTE: char = '.';
const READ_BYTE: char = ',';
const LOOP_START: char = '[';
const LOOP_END: char = ']';

const INSTS: &[char; 8] = &[
    INC_PTR, DEC_PTR, INC_BYTE, DEC_BYTE, WRITE_BYTE, READ_BYTE, LOOP_START, LOOP_END,
];

/// Length of the array used by the Brainfuck interpreter.
pub const ARRAY_LEN: usize = 30_000;
/// Newline character.
pub const NEWLINE: u8 = 10;
/// End of file character.
pub const EOF: u8 = 0;

/// This enum represents the type of Brainfuck instruction. Each variant
/// corresponds to a Brainfuck command, and additional variants handle loops
/// efficiently.
#[derive(Debug, Eq, PartialEq)]
pub enum InstKind {
    /// Brainfuck command `>` | Move the pointer to the right.
    IncPtr,
    /// Brainfuck command `<` | Move the pointer to the left.
    DecPtr,
    /// Brainfuck command `+` | Increment the byte at the pointer.
    IncByte,
    /// Brainfuck command `-` | Decrement the byte at the pointer.
    DecByte,
    /// Brainfuck command `.` | Output the byte at the pointer as a character.
    WriteByte,
    /// Brainfuck command `,` | Read a character from input and store it at the pointer.
    ReadByte,
    /// Brainfuck command `[` | Start of a loop.
    LoopStart {
        /// The index of the matching `LoopEnd` instruction.
        /// This makes it easy to jump to the end of the loop when the condition is false.
        end_idx: usize,
    },
    /// Brainfuck command `]` | End of a loop.
    LoopEnd {
        /// The index of the matching `LoopStart` instruction.
        /// This allows the interpreter to efficiently jump back to the start of the loop.
        start_idx: usize,
    },
}

/// This struct represents a single Brainfuck instruction after being parsed.
struct Inst {
    /// This is the index of the instruction in the parsed list of instructions.
    idx: usize,
    /// Describes the type of instruction.
    kind: InstKind,
    /// Represents how many times this instruction should be executed in a row.
    /// For instance, ++++ becomes a single instruction with kind: InstKind::IncByte and times: 4.
    times: usize,
}

impl Inst {
    fn new(idx: usize, c: char) -> Self {
        let kind = match c {
            INC_PTR => InstKind::IncPtr,
            DEC_PTR => InstKind::DecPtr,
            INC_BYTE => InstKind::IncByte,
            DEC_BYTE => InstKind::DecByte,
            READ_BYTE => InstKind::ReadByte,
            WRITE_BYTE => InstKind::WriteByte,
            LOOP_START => InstKind::LoopStart { end_idx: 0 },
            LOOP_END => InstKind::LoopEnd { start_idx: 0 },
            _ => panic!("unrecognized bf command {}", c),
        };

        Self {
            idx,
            kind,
            times: 1,
        }
    }

    fn increment(&mut self) {
        self.times += 1;
    }
}

/// This enum represents the possible errors that can occur during parsing or running Brainfuck
/// code.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    PtrBelowZero,
    PtrAboveLimit,
    UnbalancedBrackets,
    InfiniteLoop,
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            PtrBelowZero => write!(f, "error: mem pointer went below zero"),
            PtrAboveLimit => write!(f, "error: mem pointer went above limit {}", ARRAY_LEN),
            UnbalancedBrackets => write!(f, "error: unbalanced brackets in bf source"),
            InfiniteLoop => write!(f, "error: potential infinite loop in bf source"),
        }
    }
}

pub fn parse(std: &str) -> Result<Vec<Inst>, Error> {
    let mut loop_dept: usize = 0;
    let mut insts: Vec<Inst> = Vec::new();
}
