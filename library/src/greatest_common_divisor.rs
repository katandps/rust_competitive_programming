#[allow(unused_imports)]
use greatest_common_divisor::*;

#[allow(dead_code)]
mod greatest_common_divisor {
    use std::mem::swap;

    pub fn gcd(mut a: usize, mut b: usize) -> usize {
        if a < b {
            swap(&mut b, &mut a);
        }
        while b != 0 {
            a = a % b;
            swap(&mut a, &mut b);
        }
        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, gcd(3, 5));
        assert_eq!(2, gcd(4, 6));
        assert_eq!(3, gcd(3, 9));
        assert_eq!(3, gcd(9, 3));
        assert_eq!(11, gcd(11, 11));
    }
}
