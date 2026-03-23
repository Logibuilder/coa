use std::fmt;
fn main() {
    let i1 = Instruction::Add(12, 13);
    let i2 = Instruction::FAdd(12.0, 13.1);

    println!("{}", i1);
    println!("{}", i2);
}

/// An enumeration that represents an instruction to execute
#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    /// Integer addition
    Add(i32,i32),
    /// Integer multiplication
    Mul(i32, i32),
    /// Floating point addition
    FAdd(f32, f32),
    /// Floating point multiplication
    FMul(f32, f32),
}

impl fmt::Display for Instruction {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add(a, b) => write!(f, "{}, {}", a, b),
            Instruction::Mul(a, b) => write!(f, "{}, {}", a, b),
            Instruction::FAdd(a, b) => write!(f, "{}, {}", a, b),
            Instruction::FMul(a, b) => write!(f, "{}, {}", a, b),
        }
    }
}
