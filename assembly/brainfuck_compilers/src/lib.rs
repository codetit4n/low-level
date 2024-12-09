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

impl InstKind {
    fn set_jmp_idx(&mut self, jmp_idx: usize) {
        match self {
            InstKind::LoopStart { end_idx } => *end_idx = jmp_idx,
            InstKind::LoopEnd { start_idx } => *start_idx = jmp_idx,
            _ => panic!("trying to set jmp_idx {} on {:?}", jmp_idx, self),
        }
    }
}

/// This struct represents a single Brainfuck instruction after being parsed.
#[derive(Debug, Eq, PartialEq)]
pub struct Inst {
    /// This is the index of the instruction in the parsed list of instructions.
    pub idx: usize,
    /// Describes the type of instruction.
    pub kind: InstKind,
    /// Represents how many times this instruction should be executed in a row.
    /// For instance, ++++ becomes a single instruction with kind: InstKind::IncByte and times: 4.
    pub times: usize,
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

pub fn parse(src: &str) -> Result<Vec<Inst>, Error> {
    let mut loop_depth: usize = 0;
    let mut insts: Vec<Inst> = Vec::new();

    // 1st pass, convert chars to instructions
    for c in src.chars().filter(|c| INSTS.contains(c)) {
        if c == LOOP_END {
            if loop_depth == 0 {
                return Err(Error::UnbalancedBrackets);
            }
            loop_depth -= 1;
        } else if c == LOOP_START {
            loop_depth += 1;
        }

        let curr_inst = Inst::new(insts.len(), c);
        if let Some(prev_inst) = insts.last_mut() {
            if prev_inst.kind == curr_inst.kind {
                prev_inst.increment();
            } else {
                insts.push(curr_inst);
            }
        } else {
            insts.push(curr_inst);
        }
    }

    if loop_depth > 0 {
        return Err(Error::UnbalancedBrackets);
    }

    // 2nd pass, link loops together by setting their jmp idxs
    for i in 0..insts.len() {
        let mut update_jmp_idx: Option<usize> = None;
        let Inst { kind, times, .. } = &insts[i];
        match kind {
            // found open bracket / LoopStart
            InstKind::LoopStart { .. } => {
                let mut loop_starts = *times;

                // match outermost matching close bracket / LoopEnd
                for j in i + 1..insts.len() {
                    let Inst { kind, times, .. } = &insts[j];
                    match kind {
                        InstKind::LoopEnd { .. } => {
                            let loop_ends = *times;
                            loop_starts = loop_starts.saturating_sub(loop_ends);
                            if loop_starts == 0 {
                                update_jmp_idx = Some(j + 1);
                                break;
                            }
                        }
                        InstKind::LoopStart { .. } => {
                            let nested_loop_starts = *times;
                            loop_starts += nested_loop_starts;
                        }
                        _ => (),
                    }
                }
            }

            // found close bracket / LoopEnd
            InstKind::LoopEnd { .. } => {
                let mut loop_ends = 1_usize;

                // match innermost open bracket / LoopStart
                for j in (0..i).rev() {
                    let Inst { kind, times, .. } = &insts[j];
                    match kind {
                        InstKind::LoopStart { .. } => {
                            let loop_starts = *times;
                            loop_ends = loop_ends.saturating_sub(loop_starts);
                            if loop_ends == 0 {
                                if i == j + 1 {
                                    return Err(Error::InfiniteLoop);
                                }
                                update_jmp_idx = Some(j + 1);
                                break;
                            }
                        }
                        InstKind::LoopEnd { .. } => {
                            let nested_loop_ends = *times;
                            loop_ends += nested_loop_ends;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }

        if let Some(jmp_idx) = update_jmp_idx {
            insts[i].kind.set_jmp_idx(jmp_idx);
        }
    }

    Ok(insts)
}
