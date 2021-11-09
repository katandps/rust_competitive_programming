//! # $`p^e \bmod m`$
//! ## 計算量
//! $` \log e`$
//! ## verified by
//! [ATC002B](https://atcoder.jp/contests/atc002/submissions/26825488)
pub fn pow(mut p: usize, mut e: usize, m: usize) -> usize {
    let mut res = 1;
    while e > 0 {
        if e & 1 == 1 {
            res *= p;
            res %= m;
        }
        e >>= 1;
        p *= p;
        p %= m;
    }
    res
}
