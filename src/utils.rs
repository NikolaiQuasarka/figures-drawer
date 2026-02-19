pub fn u32_to_i32(number: u32) -> i32 {
    let number = number as u64;
    let half_range = 1u64 << 31;
    let centered: u64 = number.wrapping_sub(half_range);
    centered as i32
}

pub fn i32_to_u32(number: i32) -> u32 {
    let number = number as i64;
    let half_range = 1i64 << 31;
    let centered: i64 = number.wrapping_add(half_range);
    centered as u32
}

const fn usize_half() -> u64 {
    (((usize::MAX as u128) + 1) / 2) as u64
}

pub fn i32_to_usize(number: i32) -> usize {
    // let number = number as i64;
    // let offset = 1i64 << 31;
    // let shifted: i64 = number.wrapping_add(offset);
    // shifted as u32 as usize
    (number as u64).wrapping_sub(usize_half()) as usize
}

#[cfg(test)]
mod utils_tests {
    use core::assert_eq;

    use super::*;

    #[test]
    fn usize_half_is_correct() {
        assert_eq!(9223372036854775807 + 1, usize_half())
    }

    mod u32_to_i32_tests {
        use super::*;
        #[test]
        fn negatave_numbers() {
            let a = 32u32;
            let a = u32_to_i32(a);
            assert_eq!(-2_147_483_616i32, a)
        }
        #[test]
        fn positive_numbers() {
            let a = 2_436_345_356u32;
            let a = u32_to_i32(a);
            assert_eq!(288_861_708i32, a);
        }
    }

    mod i32_to_usize_tests {
        use super::*;
        use core::assert_eq;

        #[test]
        fn negaitve_numbers() {
            let a = i32_to_usize(-4i32);

            assert_eq!(9223372036854775808 - 4, a)
        }
        #[test]
        fn positive_numbers() {
            let a = i32_to_usize(10i32);

            assert_eq!(9223372036854775808 + 10, a)
        }
        #[test]
        fn zero() {
            let a = i32_to_usize(0i32);

            assert_eq!(9223372036854775808, a)
        }
    }
}
