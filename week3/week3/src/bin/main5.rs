
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