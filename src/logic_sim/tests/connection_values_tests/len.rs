use super::*;
#[test]
fn test_single() {
    assert_eq!(ConnectionValues::Single(true).len(), 1);
    assert_eq!(ConnectionValues::Single(false).len(), 1);
}
#[test]
fn test_half_byte() {
    assert_eq!(
        ConnectionValues::HalfByte(true, false, true, false).len(),
        4
    );
}
#[test]
fn test_byte() {
    assert_eq!(ConnectionValues::Byte(INPUT as u8).len(), 8);
}

#[test]
fn test_xx() {
    assert_eq!(ConnectionValues::X16(INPUT as u16).len(), 16);
    assert_eq!(ConnectionValues::X32(INPUT as u32).len(), 32);
    assert_eq!(ConnectionValues::X64(INPUT as u64).len(), 64);
    assert_eq!(ConnectionValues::X128(INPUT).len(), 128);
    assert_eq!(ConnectionValues::X256(INPUT, INPUT).len(), 256);
}
