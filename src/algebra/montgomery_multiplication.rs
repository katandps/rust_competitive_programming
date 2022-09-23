//! # モンゴメリ乗算
//! シフト演算などを用いて、整数の積の剰余を高速に求める
//! 法が同じ値で何度も計算する場合に効力を発揮する
//!
//! ## 参考
//! [モンゴメリ乗算-wikipedia](https://ja.wikipedia.org/wiki/%E3%83%A2%E3%83%B3%E3%82%B4%E3%83%A1%E3%83%AA%E4%B9%97%E7%AE%97)

use crate::prelude::*;

#[snippet(name = "montgomery-multiplication", doc_hidden)]
#[derive(Clone, Debug)]
pub struct MontgomeryU64 {
    /// 奇数$N$
    pub n: u64,
    /// $nn^{-1} \equiv 1 \pmod{2^{64}}$
    pub n_inv: u64,
    /// $nh = \frac{n+1}{2}$
    pub nh: u64,
    /// $2^{64} \pmod N$
    pub r: u64,
    /// $-(2^{64}) \pmod N$
    pub r_neg: u64,
    /// $(2^{64})^2 \pmod N$
    pub r_pow2: u64,
    /// (n-1) >> (n-1).trailing_zeros()
    pub d: u64,
    /// (n-1).trailing_zeros()
    pub k: u32,
}

#[snippet(name = "montgomery-multiplication", doc_hidden)]
impl MontgomeryU64 {
    #[inline]
    pub fn new(n: u64) -> Self {
        debug_assert_eq!(n & 1, 1);
        let n_inv = (0..5).fold(n, |x, _a| {
            x.wrapping_mul(2u64.wrapping_sub(n.wrapping_mul(x)))
        });
        debug_assert_eq!(n.wrapping_mul(n_inv), 1);
        let nh = (n >> 1) + 1;
        let r = n.wrapping_neg() % n;
        let r_neg = n - r;
        let r_pow2 = ((n as u128).wrapping_neg() % (n as u128)) as u64;
        let k = (n - 1).trailing_zeros();
        let d = (n - 1) >> k;

        Self {
            n,
            n_inv,
            nh,
            r,
            r_neg,
            r_pow2,
            d,
            k,
        }
    }
    /// # 加法 $\pmod n$
    /// $add(a, b) \equiv a + b \pmod n$
    #[inline]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        debug_assert!(a < self.n);
        debug_assert!(b < self.n);
        let (t, fa) = a.overflowing_add(b);
        let (u, fs) = t.overflowing_sub(self.n);
        if fa || !fs {
            u
        } else {
            t
        }
    }
    /// # 減法 $\pmod n$
    /// $sub(a,b) \equiv a - b \pmod n$
    #[inline]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        debug_assert!(a < self.n);
        debug_assert!(b < self.n);
        let (t, f) = a.overflowing_sub(b);
        if f {
            t.wrapping_add(self.n)
        } else {
            t
        }
    }

    /// # $A$のモンゴメリ表現への変換
    /// $ar(a) = a*r \mod N$
    #[inline]
    pub fn ar(&self, a: u64) -> u64 {
        debug_assert!(a < self.n);
        self.mul(a, self.r_pow2)
    }

    /// $mul(a, b) == (a * b) \mod N$
    #[inline]
    pub fn mul(&self, ar: u64, br: u64) -> u64 {
        debug_assert!(ar < self.n);
        debug_assert!(br < self.n);
        let t: u128 = (ar as u128) * (br as u128);
        let (t, f) = ((t >> 64) as u64).overflowing_sub(
            ((((t as u64).wrapping_mul(self.n_inv) as u128) * self.n as u128) >> 64) as u64,
        );
        if f {
            t.wrapping_add(self.n)
        } else {
            t
        }
    }

    /// # 累乗 $\pmod n$
    #[inline]
    pub fn pow(&self, a: u64, mut b: u64) -> u64 {
        debug_assert!(a < self.n);
        let mut ar = self.ar(a);
        let mut t = if b & 1 == 0 { self.r } else { ar };
        b >>= 1;
        while b != 0 {
            ar = self.mul(ar, ar);
            if b & 1 != 0 {
                t = self.mul(t, ar);
            }
            b >>= 1;
        }
        t
    }
}

#[test]
fn test() {
    const MOD: u64 = 255;
    let mont = MontgomeryU64::new(MOD);
    for i in 0..MOD {
        assert_eq!(mont.ar(i), i * mont.r % MOD);
        for j in 0..MOD {
            assert_eq!(mont.mul(i, j) * mont.r % MOD, i * j % MOD);
            assert_eq!(mont.add(i, j), (i + j) % MOD);
            assert_eq!(mont.sub(i, j), (MOD + i - j) % MOD);
            assert_eq!(mont.mul(i, j), i * j % MOD);
        }
    }
}