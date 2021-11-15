//! # 自然数の分割
//!
//! ## verify
//! [ABC226F](https://atcoder.jp/contests/abc226/submissions/27139567)

use crate::prelude::*;

#[snippet(name = "split-of-natural-number", doc_hidden)]
#[derive(Clone, Debug)]
pub struct SplitOfNumber(Option<Vec<usize>>);
impl Iterator for SplitOfNumber {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        let ret = self.0.clone();
        if let Some(v) = &mut self.0 {
            match v.iter().rposition(|&x| x != 1) {
                None => self.0 = None,
                Some(i) => {
                    let others = v.split_off(i);
                    let mut rest = others.iter().sum::<usize>();
                    let max = others[0] - 1;
                    while rest > 0 {
                        let next = rest.min(max);
                        v.push(next);
                        rest -= next;
                    }
                }
            }
        } else {
            self.0 = None
        };
        ret
    }
}

#[snippet(name = "split-of-natural-number", doc_hidden)]
impl From<usize> for SplitOfNumber {
    fn from(n: usize) -> Self {
        SplitOfNumber(Some(vec![n]))
    }
}

#[snippet(name = "split-of-natural-number", doc_hidden)]
impl From<&[usize]> for SplitOfNumber {
    fn from(src: &[usize]) -> Self {
        SplitOfNumber(Some(src.to_vec()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res: Vec<_> = SplitOfNumber::from(5).collect();
        assert_eq!(
            res,
            vec![
                vec![5],
                vec![4, 1],
                vec![3, 2],
                vec![3, 1, 1],
                vec![2, 2, 1],
                vec![2, 1, 1, 1],
                vec![1, 1, 1, 1, 1]
            ]
        );

        assert_eq!(204226, SplitOfNumber::from(50).count());
    }
}