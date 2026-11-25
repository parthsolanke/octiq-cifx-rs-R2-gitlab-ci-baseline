//! Trivial, dependency-free library so `cargo build` and `cargo test` are fast.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn double(a: i32) -> i32 {
    a * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn doubles() {
        assert_eq!(double(21), 42);
    }

    #[test]
    fn adds_negative() {
        assert_eq!(add(-5, 3), -2);
    }

    #[test]
    fn adds_zero() {
        assert_eq!(add(7, 0), 7);
    }

    #[test]
    fn doubles_zero() {
        assert_eq!(double(0), 0);
    }

    #[test]
    fn doubles_negative() {
        assert_eq!(double(-8), -16);
    }

    #[test]
    fn adds_zero_pair() {
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn adds_opposite_signs_to_zero() {
        assert_eq!(add(123, -123), 0);
    }

    #[test]
    fn adds_extreme_values() {
        assert_eq!(add(i32::MIN, i32::MAX), -1);
    }

    #[test]
    fn double_matches_adding_itself() {
        for value in [-3, 0, 7] {
            assert_eq!(double(value), add(value, value));
        }
    }

    #[test]
    fn doubles_one() {
        assert_eq!(double(1), 2);
    }
}
