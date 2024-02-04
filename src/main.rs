const fn const_itoa<const NUM: usize>() -> &'static str {
    trait ConstToString<const CONST_NUM: usize> {
        const CONST_STR: &'static [u8; 20];
    }
    impl<const N: usize> ConstToString<N> for () {
        const CONST_STR: &'static [u8; 20] = &[
            b'0' + (((N / (10usize).pow(20 - 1)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 2)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 3)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 4)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 5)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 6)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 7)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 8)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 9)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 10)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 11)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 12)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 13)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 14)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 15)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 16)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 17)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 18)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 19)) % 10) as u8),
            b'0' + (((N / (10usize).pow(20 - 20)) % 10) as u8),
        ];
    }
    let len = if NUM == 0 {
        1
    } else {
        NUM.ilog10() as usize + 1
    };
    unsafe {
        let bytes = <() as ConstToString<NUM>>::CONST_STR;
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(
            std::mem::transmute(bytes.as_ptr().offset((bytes.len() - len) as isize)),
            len,
        ))
    }
}

fn main() {
    const TEXT: &'static str = const_itoa::<{ usize::MAX }>();
    let number = usize::MAX;
    println!("{TEXT:?} == {number}");
}

#[test]
fn test() {
    const TEXT_ZERO: &'static str = const_itoa::<0>();
    let number_zero = 0;
    assert_eq!(TEXT_ZERO, number_zero.to_string());

    const TEXT_MAX: &'static str = const_itoa::<{ usize::MAX }>();
    let number_max = usize::MAX;
    assert_eq!(TEXT_MAX, number_max.to_string());
}
