//! # 桁分割
//! 整数を10進数として桁ごとの値がはいったvecに変換する
//!
//! ```
//! # use rust_lib_for_comp::algo::int_to_digits::int_to_digits;
//! let a = 12345;
//! let d = int_to_digits(a);
//! assert_eq!(d, vec![1, 2, 3, 4, 5])
//! ```
use crate::prelude::*;

#[snippet(name = "int-to-digits", doc_hidden)]
pub fn int_to_digits(mut n: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    if n == 0 {
        ret.push(0)
    }
    while n > 0 {
        ret.push(n % 10);
        n /= 10;
    }
    ret.reverse();
    ret
}
