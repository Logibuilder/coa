use std::convert::TryFrom;
use std::fmt;


fn main() {
    let bytes1: &[u8] = &[0, 1, 0, 0, 0, 1, 0, 0, 0];
    let bytes2: &[u8] = &[1, 0, 0, 0, 0, 4, 0, 0, 0];

    let i1 = Instruction::try_from(bytes1).unwrap();
    let i2 = Instruction::try_from(bytes2).unwrap();

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
            Instruction::Add(a, b) | Instruction::Mul(a, b) => write!(f, "{}, {}", a, b),
            Instruction::FAdd(a, b) | Instruction::FMul(a, b) => write!(f, "{}, {}", a, b)
        }
    }
}


impl TryFrom<&[u8]> for Instruction {
    type Error = ParseInstructionError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        if value.len() != 9 {
            return Err(ParseInstructionError::InvalidFormedError);
        }

        let opcode = value[0];

        
        match opcode {
            0 =>  {
                let a = i32::from_le_bytes(value[1..5].try_into().map_err(|_| ParseInstructionError::MalFormedError)?);
                let b = i32::from_le_bytes(value[5..9].try_into().map_err(|_| ParseInstructionError::MalFormedError)?);
                Ok(Instruction::Add(a, b))
            },
            1 => {
                let a = i32::from_le_bytes(value[1..5].try_into().map_err(|_| ParseInstructionError::MalFormedError)?);
                let b = i32::from_le_bytes(value[5..9].try_into().map_err(|_| ParseInstructionError::MalFormedError)?);
                Ok(Instruction::Mul(a, b))
            },
            2 => {
                let a = f32::from_bits(u32::from_le_bytes(value[1..5].try_into().map_err(|_| ParseInstructionError::MalFormedError)?));
                let b = f32::from_bits(u32::from_le_bytes(value[5..9].try_into().map_err(|_| ParseInstructionError::MalFormedError)?));
                Ok(Instruction::FAdd(a, b))
            },
            3 => {
                let a = f32::from_bits(u32::from_le_bytes(value[1..5].try_into().map_err(|_| ParseInstructionError::MalFormedError)?));
                let b = f32::from_bits(u32::from_le_bytes(value[5..9].try_into().map_err(|_| ParseInstructionError::MalFormedError)?));
                Ok(Instruction::FMul(a, b))
            },
            _ => Err(ParseInstructionError::UnknownInstructionError)
        }
    }
}




#[derive(Debug)]
enum ParseInstructionError {
    InvalidFormedError,
    MalFormedError,
    ParsError,
    UnknownInstructionError
}

