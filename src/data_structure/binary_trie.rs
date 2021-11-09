//! 非負整数をBit列とみなしてトライ木に載せたもの
/// multiset的な機能を持つ
use crate::prelude::*;

#[snippet(name = "binary-trie", doc_hidden)]
type TrieValue = u64;

#[snippet(name = "binary-trie", doc_hidden)]
#[derive(Clone, Default)]
pub struct BinaryTrie {
    root: TrieNode,
}

#[snippet(name = "binary-trie", doc_hidden)]
impl BinaryTrie {
    /// $`2^{60}`$未満の非負整数を登録できる
    pub const BIT_LEN: i64 = 60;

    /// 今までにinsertした個数
    pub fn size(&self) -> usize {
        self.root.count
    }

    /// vをinsertする
    pub fn insert(&mut self, v: u64) {
        self.root.add(v, Self::BIT_LEN - 1);
    }

    /// vを一つ削除する
    pub fn erase(&mut self, v: TrieValue) {
        self.root.sub(v, Self::BIT_LEN - 1);
    }

    /// biasとXORをとったときに最小値となるような値を取得する
    pub fn min_element(&self, bias: Option<TrieValue>) -> TrieValue {
        self.root.get_min(bias.unwrap_or(0), Self::BIT_LEN - 1)
    }

    /// biasとXORをとったときに最大値となるような値を取得する
    pub fn max_element(&self, bias: Option<TrieValue>) -> TrieValue {
        self.root.get_min(
            bias.map_or((1 << Self::BIT_LEN) - 1, |b| b ^ ((1 << Self::BIT_LEN) - 1)),
            Self::BIT_LEN - 1,
        )
    }

    pub fn nth(&self, k: usize) -> TrieValue {
        assert!(k <= self.size());
        self.root.get(k, Self::BIT_LEN - 1)
    }
}

#[snippet(name = "binary-trie", doc_hidden)]
impl Debug for BinaryTrie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.size())
                .map(|i| self.nth(i).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[snippet(name = "binary-trie", doc_hidden)]
#[derive(Clone)]
struct TrieNode {
    count: usize,
    child: Vec<Option<TrieNode>>,
}

#[snippet(name = "binary-trie", doc_hidden)]
impl Default for TrieNode {
    fn default() -> Self {
        Self {
            count: 0,
            child: vec![None, None],
        }
    }
}

#[snippet(name = "binary-trie", doc_hidden)]
impl TrieNode {
    pub fn add(&mut self, v: TrieValue, b: i64) {
        self.count += 1;
        if b < 0 {
            return;
        }
        let dst = (v >> b & 1) as usize;
        if let Some(c) = self.child[dst].as_mut() {
            c.add(v, b - 1);
        } else {
            let mut node = TrieNode::default();
            node.add(v, b - 1);
            self.child[dst] = Some(node);
        }
    }

    pub fn sub(&mut self, v: TrieValue, b: i64) {
        self.count -= 1;
        if b < 0 {
            return;
        }
        let dst = (v >> b & 1) as usize;
        assert!(self.child[dst].is_some());
        if let Some(c) = self.child[dst].as_mut() {
            c.sub(v, b - 1);
        } else {
            unreachable!()
        }
    }

    pub fn get_min(&self, bias: TrieValue, b: i64) -> TrieValue {
        if b < 0 {
            return 0;
        }
        let mut dst = (bias >> b & 1) as usize;
        if self.child[dst].is_none() {
            dst ^= 1;
        }
        self.child[dst]
            .as_ref()
            .map_or(0, |c| c.get_min(bias, b - 1))
            | (dst << b) as TrieValue
    }

    pub fn get(&self, k: usize, b: i64) -> TrieValue {
        if b < 0 {
            return 0;
        }
        let m = self.child[0].as_ref().map_or(0, |c| c.count);
        if k < m {
            self.child[0].as_ref().unwrap().get(k, b - 1)
        } else {
            self.child[1].as_ref().map_or(0, |c| c.get(k - m, b - 1)) | (1 << b)
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        let mut trie = BinaryTrie::default();
        trie.insert(5);
        trie.insert(6);
        trie.insert(7);
        trie.insert(8);
        trie.insert(9);
        assert_eq!(5, trie.size());
        assert_eq!(5, trie.nth(0));
        assert_eq!(6, trie.nth(1));
        assert_eq!(7, trie.nth(2));
        assert_eq!(8, trie.nth(3));
        assert_eq!(9, trie.nth(4));
        assert_eq!(5, trie.min_element(None));
        assert_eq!(9, trie.max_element(None));

        trie.erase(5);
        trie.erase(7);

        assert_eq!(3, trie.size());
        assert_eq!(6, trie.nth(0));
        assert_eq!(8, trie.nth(1));
        assert_eq!(9, trie.nth(2));
    }
}
