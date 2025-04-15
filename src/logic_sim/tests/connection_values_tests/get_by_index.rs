use super::*;
fn check_get_by_index(val: ConnectionValues) {
    assert_eq!(val.get_by_index(val.len()), false); // Out of bounds
    assert_eq!(val.get_by_index(usize::MAX), false); // Out of bounds

    for i in 0..val.len() {
        if i >= 128 {
            assert_eq!(val.get_by_index(i), (INPUT & (1 << (i - 128))) != 0);
        } else {
            assert_eq!(val.get_by_index(i), (INPUT & (1 << i)) != 0);
        }
    }
}
#[test]
fn test_single() {
    let val_true = ConnectionValues::Single(true);
    let val_false = ConnectionValues::Single(false);

    assert_eq!(val_true.get_by_index(0), true);
    assert_eq!(val_false.get_by_index(0), false);

    assert_eq!(val_true.get_by_index(1), false); // Out of bounds
    assert_eq!(val_false.get_by_index(1), false);
}

#[test]
fn test_half_byte() {
    let val = ConnectionValues::HalfByte(true, false, true, false);
    check_get_by_index(val);

    assert_eq!(val.get_by_index(0), true);
    assert_eq!(val.get_by_index(1), false);
    assert_eq!(val.get_by_index(2), true);
    assert_eq!(val.get_by_index(3), false);

    assert_eq!(val.get_by_index(4), false); // Out of bounds
}

#[test]
fn test_byte() {
    let val = ConnectionValues::Byte(INPUT as u8); // 1000_0101
    check_get_by_index(val);
    assert_eq!(val.get_by_index(0), true);
    assert_eq!(val.get_by_index(1), false);
    assert_eq!(val.get_by_index(2), true);
    assert_eq!(val.get_by_index(3), false);

    assert_eq!(val.get_by_index(4), false);
    assert_eq!(val.get_by_index(5), false);
    assert_eq!(val.get_by_index(6), false);
    assert_eq!(val.get_by_index(7), true);

    assert_eq!(val.get_by_index(8), false); // Out of bounds
}

#[test]
fn test_x16() {
    let val = ConnectionValues::X16(INPUT as u16); // 1001_0110_1000_0101
    check_get_by_index(val);

    assert_eq!(val.get_by_index(0), true);
    assert_eq!(val.get_by_index(1), false);
    assert_eq!(val.get_by_index(2), true);
    assert_eq!(val.get_by_index(3), false);

    assert_eq!(val.get_by_index(4), false);
    assert_eq!(val.get_by_index(5), false);
    assert_eq!(val.get_by_index(6), false);
    assert_eq!(val.get_by_index(7), true);

    assert_eq!(val.get_by_index(8), false);
    assert_eq!(val.get_by_index(9), true);
    assert_eq!(val.get_by_index(10), true);
    assert_eq!(val.get_by_index(11), false);

    assert_eq!(val.get_by_index(12), true);
    assert_eq!(val.get_by_index(13), false);
    assert_eq!(val.get_by_index(14), false);
    assert_eq!(val.get_by_index(15), true);

    assert_eq!(val.get_by_index(16), false); // Out of bounds
}

#[test]
fn test_x32() {
    let val = ConnectionValues::X32(INPUT as u32);
    assert_eq!(val.get_by_index(0), true);
    assert_eq!(val.get_by_index(1), false);
    assert_eq!(val.get_by_index(2), true);
    assert_eq!(val.get_by_index(3), false);

    assert_eq!(val.get_by_index(4), false);
    assert_eq!(val.get_by_index(5), false);
    assert_eq!(val.get_by_index(6), false);
    assert_eq!(val.get_by_index(7), true);

    assert_eq!(val.get_by_index(8), false);
    assert_eq!(val.get_by_index(9), true);
    assert_eq!(val.get_by_index(10), true);
    assert_eq!(val.get_by_index(11), false);

    assert_eq!(val.get_by_index(12), true);
    assert_eq!(val.get_by_index(13), false);
    assert_eq!(val.get_by_index(14), false);
    assert_eq!(val.get_by_index(15), true);

    assert_eq!(val.get_by_index(16), false);
    assert_eq!(val.get_by_index(31), true);
    assert_eq!(val.get_by_index(32), false); // Out of bounds
    check_get_by_index(val);
}

#[test]
fn test_x64() {
    let val = ConnectionValues::X64(INPUT as u64);
    assert_eq!(val.get_by_index(0), true);
    assert_eq!(val.get_by_index(1), false);
    assert_eq!(val.get_by_index(2), true);
    assert_eq!(val.get_by_index(3), false);

    assert_eq!(val.get_by_index(4), false);
    assert_eq!(val.get_by_index(5), false);
    assert_eq!(val.get_by_index(6), false);
    assert_eq!(val.get_by_index(7), true);

    assert_eq!(val.get_by_index(8), false);
    assert_eq!(val.get_by_index(9), true);
    assert_eq!(val.get_by_index(10), true);
    assert_eq!(val.get_by_index(11), false);

    assert_eq!(val.get_by_index(12), true);
    assert_eq!(val.get_by_index(13), false);
    assert_eq!(val.get_by_index(14), false);
    assert_eq!(val.get_by_index(15), true);

    assert_eq!(val.get_by_index(16), false);

    assert_eq!(val.get_by_index(31), true);

    assert_eq!(val.get_by_index(63), true);

    assert_eq!(val.get_by_index(64), false); // Out of bounds
    check_get_by_index(val);
}

#[test]
fn test_x128() {
    let val = ConnectionValues::X128(INPUT);
    assert_eq!(val.get_by_index(0), true);
    assert_eq!(val.get_by_index(1), false);
    assert_eq!(val.get_by_index(2), true);
    assert_eq!(val.get_by_index(3), false);

    assert_eq!(val.get_by_index(4), false);
    assert_eq!(val.get_by_index(5), false);
    assert_eq!(val.get_by_index(6), false);
    assert_eq!(val.get_by_index(7), true);

    assert_eq!(val.get_by_index(8), false);
    assert_eq!(val.get_by_index(9), true);
    assert_eq!(val.get_by_index(10), true);
    assert_eq!(val.get_by_index(11), false);

    assert_eq!(val.get_by_index(12), true);
    assert_eq!(val.get_by_index(13), false);
    assert_eq!(val.get_by_index(14), false);
    assert_eq!(val.get_by_index(15), true);

    assert_eq!(val.get_by_index(16), false);

    assert_eq!(val.get_by_index(31), true);

    assert_eq!(val.get_by_index(63), true);

    assert_eq!(val.get_by_index(64), true);
    assert_eq!(val.get_by_index(68), false);
    assert_eq!(val.get_by_index(72), false);
    assert_eq!(val.get_by_index(76), true);
    assert_eq!(val.get_by_index(80), false);
    assert_eq!(val.get_by_index(84), false);
    assert_eq!(val.get_by_index(88), false);
    assert_eq!(val.get_by_index(92), false);
    assert_eq!(val.get_by_index(96), true);
    assert_eq!(val.get_by_index(127), true);
    assert_eq!(val.get_by_index(128), false); // Out of bounds
    check_get_by_index(val);
}

#[test]
fn test_x256() {
    let val = ConnectionValues::X256(INPUT, INPUT);
    assert_eq!(val.get_by_index(0), true);
    assert_eq!(val.get_by_index(1), false);
    assert_eq!(val.get_by_index(2), true);
    assert_eq!(val.get_by_index(3), false);

    assert_eq!(val.get_by_index(4), false);
    assert_eq!(val.get_by_index(5), false);
    assert_eq!(val.get_by_index(6), false);
    assert_eq!(val.get_by_index(7), true);

    assert_eq!(val.get_by_index(8), false);
    assert_eq!(val.get_by_index(9), true);
    assert_eq!(val.get_by_index(10), true);
    assert_eq!(val.get_by_index(11), false);

    assert_eq!(val.get_by_index(12), true);
    assert_eq!(val.get_by_index(13), false);
    assert_eq!(val.get_by_index(14), false);
    assert_eq!(val.get_by_index(15), true);

    assert_eq!(val.get_by_index(16), false);

    assert_eq!(val.get_by_index(31), true);

    assert_eq!(val.get_by_index(63), true);

    assert_eq!(val.get_by_index(64), true);
    assert_eq!(val.get_by_index(68), false);
    assert_eq!(val.get_by_index(72), false);
    assert_eq!(val.get_by_index(76), true);
    assert_eq!(val.get_by_index(80), false);
    assert_eq!(val.get_by_index(84), false);
    assert_eq!(val.get_by_index(88), false);
    assert_eq!(val.get_by_index(92), false);
    assert_eq!(val.get_by_index(96), true);
    assert_eq!(val.get_by_index(127), true);

    assert_eq!(val.get_by_index(128 + 0), true);
    assert_eq!(val.get_by_index(128 + 1), false);
    assert_eq!(val.get_by_index(128 + 2), true);
    assert_eq!(val.get_by_index(128 + 3), false);
    assert_eq!(val.get_by_index(128 + 4), false);
    assert_eq!(val.get_by_index(128 + 5), false);
    assert_eq!(val.get_by_index(128 + 6), false);
    assert_eq!(val.get_by_index(128 + 7), true);
    assert_eq!(val.get_by_index(128 + 8), false);
    assert_eq!(val.get_by_index(128 + 9), true);
    assert_eq!(val.get_by_index(128 + 10), true);
    assert_eq!(val.get_by_index(128 + 11), false);
    assert_eq!(val.get_by_index(128 + 12), true);
    assert_eq!(val.get_by_index(128 + 13), false);
    assert_eq!(val.get_by_index(128 + 14), false);
    assert_eq!(val.get_by_index(128 + 15), true);
    assert_eq!(val.get_by_index(128 + 16), false);
    assert_eq!(val.get_by_index(128 + 31), true);
    assert_eq!(val.get_by_index(128 + 63), true);
    assert_eq!(val.get_by_index(128 + 64), true);
    assert_eq!(val.get_by_index(128 + 68), false);
    assert_eq!(val.get_by_index(128 + 72), false);
    assert_eq!(val.get_by_index(128 + 76), true);
    assert_eq!(val.get_by_index(128 + 80), false);
    assert_eq!(val.get_by_index(128 + 84), false);
    assert_eq!(val.get_by_index(128 + 88), false);
    assert_eq!(val.get_by_index(128 + 92), false);
    assert_eq!(val.get_by_index(128 + 96), true);
    assert_eq!(val.get_by_index(128 + 127), true);

    assert_eq!(val.get_by_index(255), true);
    assert_eq!(val.get_by_index(256), false); // Out of bounds
    check_get_by_index(val);
}
