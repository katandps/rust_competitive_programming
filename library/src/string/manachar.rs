//! 最長回文半径

/// 最長回文半径をO(N)で求める Manachar's Algorithm
///
/// 返り値は中心をiとしたときの最大回文半径
/// 偶数長の回文半径がほしければsrcにダミー文字を挟む(ex. abba -> a_b_b_a
pub fn manachar<T: Eq>(src: &[T]) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;
    let mut dp = vec![0; src.len()];
    while i < src.len() {
        while i >= j && i + j < src.len() && src[i - j] == src[i + j] {
            j += 1;
        }
        dp[i] = j;
        let mut k = 1;
        while i >= k && k + dp[i - k] < j {
            dp[i + k] = dp[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    dp
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let src: Vec<_> = "abaaababa".chars().collect();
        let res = manachar(&src);
        assert_eq!(vec![1, 2, 1, 4, 1, 2, 3, 2, 1], res);
    }

    #[test]
    fn test_fast_enough() {
        let src: Vec<_> = (0..5000000).map(|i| i % 4).collect();
        let _res = manachar(&src);
    }
}
