use std::str::FromStr;
use std::fmt;


fn main() {
    let a = Instruction::Add(2, 3);
    let b = a;
    let c = a;

    assert!(matches!(b, Instruction::Add(2, 3)));
    assert!(matches!(c, Instruction::Add(2, 3)));

    let bytes: [u8; 9] = a.into();
    println!("{:?}", bytes);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Add(i32, i32),
    Mul(i32, i32),
    FAdd(f32, f32),
    FMul(f32, f32)
}

impl From<Instruction> for [u8; 9] {
    fn from(value : Instruction) -> Self {
        let mut bytes = [0u8; 9];
        match value {
            Instruction::Add(a, b) => {
                bytes[0] = 0;
                bytes[1..5].copy_from_slice(&a.to_le_bytes());
                bytes[5..9].copy_from_slice(&b.to_le_bytes());
            },
            Instruction::Mul(a, b) => {
                bytes[0] = 1;
                bytes[1..5].copy_from_slice(&a.to_le_bytes());
                bytes[5..9].copy_from_slice(&b.to_le_bytes());
            },
            Instruction::FAdd(a, b) => {
                bytes[0] = 2;
                bytes[1..5].copy_from_slice(&a.to_bits().to_le_bytes());
                bytes[5..9].copy_from_slice(&b.to_bits().to_le_bytes());
            },
            Instruction::FMul(a, b) => {
                bytes[0] = 3;
                bytes[1..5].copy_from_slice(&a.to_bits().to_le_bytes());
                bytes[5..9].copy_from_slice(&b.to_bits().to_le_bytes());
            }
        }
        bytes
    }
}


impl FromStr for Instruction {

    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(ParseInstructionError::InvalidFormedError);
        }

        let op = parts[1];
        let left = parts[0];
        let right = parts[2];
        let is_float = left.contains('.') || right.contains('.');

        match op {
            "+" => {
                if is_float {
                    let a = left.parse::<f32>().map_err(|_| ParseInstructionError::ParsError)?;
                    let b = right.parse::<f32>().map_err(|_| ParseInstructionError::ParsError)?;
                    Ok(Instruction::FAdd(a, b))
                } else {
                    let a = left.parse::<i32>().map_err(|_| ParseInstructionError::ParsError)?;
                    let b = right.parse::<i32>().map_err(|_| ParseInstructionError::ParsError)?;
                    Ok(Instruction::Add(a, b))
                }
            },
            "*" => {
                if is_float {
                    let a = left.parse::<f32>().map_err(|_| ParseInstructionError::ParsError)?;
                    let b = right.parse::<f32>().map_err(|_| ParseInstructionError::ParsError)?;
                    Ok(Instruction::FMul(a, b))
                } else {
                    let a = left.parse::<i32>().map_err(|_| ParseInstructionError::ParsError)?;
                    let b = right.parse::<i32>().map_err(|_| ParseInstructionError::ParsError)?;
                    Ok(Instruction::Mul(a, b))
                }
            },
             _ => Err(ParseInstructionError::UnknownInstructionError),
        }
    } 
}

#[derive(Debug)]
enum ParseInstructionError {
    UnknownInstructionError,
    InvalidFormedError,
    MalFormedError,
    ParsError,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add(a, b)=> write!(f, "{} + {}", a, b),
            Instruction::Mul(a, b) => write!(f, "{} * {}", a, b),
            Instruction::FAdd(a, b)=> write!(f, "{} + {}", a, b),
            Instruction::FMul(a, b) => write!(f, "{} * {}", a, b)
        }
    }
}


//use super::*;

#[test]
fn clone() {
    let a = Instruction::Add(2, 3);
    let b = a.clone();

    assert!(matches!(b, Instruction::Add(2, 3)));
}

#[test]
fn copy() {
    let a = Instruction::Add(2, 3);
    let b = a;
    let c = a;

    assert!(matches!(b, Instruction::Add(2, 3)));
    assert!(matches!(c, Instruction::Add(2, 3)));
}

#[test]
fn debug() {
    let a = Instruction::Mul(3, 4);
    let s = format!("{a:?}");

    assert_eq!(s, "Mul(3, 4)");
}

#[test]
fn partialeq() {
    let a = Instruction::FMul(3.0, 4.0);
    let b = Instruction::FMul(3.0, 4.0);

    assert_eq!(a, b);
}
#[test]
fn display() {
    let a = Instruction::Add(1, 2);
    let s = format!("{a}");

    assert_eq!(s, "1 + 2");

    let a = Instruction::Mul(1, 2);
    let s = format!("{a}");

    assert_eq!(s, "1 * 2");

    let a = Instruction::FAdd(1.0, 2.1);
    let s = format!("{a}");

    assert_eq!(s, "1 + 2.1");

    let a = Instruction::FMul(1.1, 2.0);
    let s = format!("{a}");

    assert_eq!(s, "1.1 * 2");
}

#[test]
fn from_str() {
    let a: Instruction = "1 + 2".parse().unwrap();
    assert_eq!(a, Instruction::Add(1, 2));

    let a: Instruction = "1.0 + 2".parse().unwrap();
    assert_eq!(a, Instruction::FAdd(1.0, 2.0));

    let a: Instruction = "1 * 2".parse().unwrap();
    assert_eq!(a, Instruction::Mul(1, 2));

    let a: Instruction = "1 * 2.3".parse().unwrap();
    assert_eq!(a, Instruction::FMul(1.0, 2.3));
}

#[test]
fn try_from_u8() {
    let data = [0u8, 0x45, 0x05, 00, 00, 0x6e, 0xdd, 0x0d, 00];
    let a: Instruction = data[..].try_into().unwrap();
    assert_eq!(a, Instruction::Add(1349, 908654));

    let data = [1u8, 0x45, 0x05, 00, 00, 0x6e, 0xdd, 0x0d, 00];
    let a: Instruction = data[..].try_into().unwrap();
    assert_eq!(a, Instruction::Mul(1349, 908654));

    let data = [2u8, 0, 0, 0, 64, 0, 0, 64, 64];
    let a: Instruction = data[..].try_into().unwrap();
    assert_eq!(a, Instruction::FAdd(2.0, 3.0));

    let data = [3u8, 0, 0, 0, 64, 0, 0, 64, 64];
    let a: Instruction = data[..].try_into().unwrap();
    assert_eq!(a, Instruction::FMul(2.0, 3.0));

    // If slice is too long, it should not be a problem
    let data = [0u8, 0x45, 0x05, 00, 00, 0x6e, 0xdd, 0x0d, 00, 10, 12, 13, 14];
    let a: Instruction = data[..].try_into().unwrap();
    assert_eq!(a, Instruction::Add(1349, 908654));

}

#[test]
#[should_panic]
fn try_from_small_slice_should_panic() {
    let data = [0u8, 0x45, 0x05, 00, 00, 0x6e, 0xdd, 0x0d];
    let _: Instruction = data[..].try_into().unwrap();
}

#[test]
#[should_panic]
fn try_from_unknown_opcode_should_panic() {
    let data = [5u8, 0x45, 0x05, 00, 00, 0x6e, 0xdd, 0x0d];
    let _: Instruction = data[..].try_into().unwrap();
}


#[test]
fn from_intruction() {
    let a = Instruction::Add(1349, 908654);
    let b: [u8; 9] = a.into();
    assert_eq!(b, [0u8, 0x45, 0x05, 00, 00, 0x6e, 0xdd, 0x0d, 00]);

    let a = Instruction::Mul(1349, 908654);
    let b: [u8; 9] = a.into();
    assert_eq!(b, [1u8, 0x45, 0x05, 00, 00, 0x6e, 0xdd, 0x0d, 00]);

    let a = Instruction::FAdd(2.0, 3.0);
    let b: [u8; 9] = a.into();
    assert_eq!(b, [2u8, 0, 0, 0, 64, 0, 0, 64, 64]);

    let a = Instruction::FMul(2.0, 3.0);
    let b: [u8; 9] = a.into();
    assert_eq!(b, [3u8, 0, 0, 0, 64, 0, 0, 64, 64]);
}

impl TryFrom<&[u8]> for Instruction {
    type Error = ParseInstructionError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        if value.len() < 9 {
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