use std::fmt;
 use std::str::FromStr;
fn main() {
    let i1 = Instruction::Add(12, 13);
    let i2 = Instruction::FAdd(12.0, 13.1);

    println!("{}", i1.to_string());
    println!("{}", i2.to_string());
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


impl FromStr for Instruction {

    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(ParseInstructionError::InvaliMalFormedError);
        }

        match parts[0] {
            "add" => {
                let a = parts[1].parse().map_err(|_| ParseInstructionError::ParsError)?;
                let b = parts[2].parse().map_err(|_| ParseInstructionError::ParsError)?;
                Ok(Instruction::Add(a, b))
            },
            "mul" => {
                let a = parts[1].parse().map_err(|_| ParseInstructionError::ParsError)?;
                let b = parts[2].parse().map_err(|_| ParseInstructionError::ParsError)?;
                Ok(Instruction::Mul(a, b))
            },
            "fadd" => {
                let a = parts[1].parse().map_err(|_| ParseInstructionError::ParsError)?;
                let b = parts[2].parse().map_err(|_| ParseInstructionError::ParsError)?;
                Ok(Instruction::FAdd(a, b))
            },
            "fmul" => {
                let a = parts[1].parse().map_err(|_| ParseInstructionError::ParsError)?;
                let b = parts[2].parse().map_err(|_| ParseInstructionError::ParsError)?;
                Ok(Instruction::FMul(a, b))
            },
            _ => Err(ParseInstructionError::UnknownInstruction),
        }
    } 
}

#[derive(Debug)]
enum ParseInstructionError {
    UnknownInstruction,
    ParsError,
    InvaliMalFormedError,
}