struct Inst {
    idx: usize,     // index of instruction
    kind: InstKind, // kind of instruction
    times: usize,   // run-length encoding of instruction
}

#[derive(Debug, Eq, PartialEq)]
pub enum InstKind {
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    WriteByte,
    ReadByte,
    LoopStart { end_idx: usize },
    LoopEnd { start_idx: usize },
}
