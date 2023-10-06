//! LCP配列 Longest Common Prefix Array
use prelude::*;
use suffix_array::SuffixArray;

#[snippet(name = "longest-common-prefix-array", doc_hidden)]
pub struct LCPArray {
    pub lcp: Vec<usize>,
}

#[snippet(name = "longest-common-prefix-array", doc_hidden)]
impl LCPArray {
    pub fn build(sa: &SuffixArray) -> Self {
        let n = sa.source.len();
        assert!(n > 0);

        let mut rank = vec![0; n];
        sa.sa.iter().enumerate().for_each(|(i, sai)| rank[*sai] = i);

        let mut lcp = vec![0; n];
        let mut h = 0usize;
        for i in 0..n {
            h = h.saturating_sub(1);
            if rank[i] == 0 {
                continue;
            }
            let j = sa.sa[rank[i] - 1];
            while j + h < n && i + h < n {
                if sa.source[j + h] != sa.source[i + h] {
                    break;
                }
                h += 1;
            }
            lcp[rank[i]] = h;
        }
        LCPArray { lcp }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let sa = SuffixArray::build(
            &b"mississippi"
                .iter()
                .map(|&u| u as usize)
                .collect::<Vec<_>>(),
        );
        let lcp = LCPArray::build(&sa);
        assert_eq!(vec![11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2], sa.sa);
        assert_eq!(vec![0, 0, 1, 1, 4, 0, 0, 1, 0, 2, 1, 3], lcp.lcp);
    }
}
