//! # Treap
//! 乱数を利用して平衡を保つ二分探索木Tree + Heap

use crate::algo::xor_shift::XorShift;
use crate::prelude::*;

#[allow(dead_code)]
pub struct Treap<T: Copy + Debug> {
    randomizer: XorShift,
    node: Box<TreapNode<T>>,
}
impl<T: Copy + Debug + PartialOrd> Treap<T> {
    pub fn new() -> Self {
        Treap {
            randomizer: XorShift::default(),
            node: Box::new(TreapNode(None)),
        }
    }

    pub fn size(&self) -> usize {
        self.node.size()
    }

    pub fn insert(&mut self, pos: usize, x: T) {
        self.node
            .insert(pos, TreapNode::new(x, self.randomizer.next().unwrap()))
    }
}

impl<T: Copy + Debug + PartialOrd + Display> Display for Treap<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{}]", self.node)
    }
}

#[derive(Debug, Clone)]
pub struct TreapNode<T: Copy + Debug>(Option<Node<T>>);

#[derive(Debug, Clone)]
struct Node<T: Copy + Debug> {
    /// キー
    key: T,
    /// 優先度
    p: u64,
    /// 部分木のサイズ
    size: usize,
    /// 左の子
    l: Box<TreapNode<T>>,
    /// 右の子
    r: Box<TreapNode<T>>,
}

impl<T: PartialOrd + Copy + Debug> TreapNode<T> {
    fn new(key: T, p: u64) -> Self {
        Self(Some(Node {
            key,
            p,
            size: 1,
            l: Box::new(Self(None)),
            r: Box::new(Self(None)),
        }))
    }

    fn size(&self) -> usize {
        if let Some(node) = &self.0 {
            node.size
        } else {
            0
        }
    }

    fn update(&mut self) {
        if let Some(node) = self.0.as_mut() {
            node.size = 1 + node.l.size() + node.r.size()
        }
    }

    fn insert(&mut self, key: usize, mut item: Self) {
        let (mut l, mut r) = (Self(None), Self(None));
        self.split(key, &mut l, &mut r);
        self.merge(&mut l);
        self.merge(&mut item);
        self.merge(&mut r);
    }

    /// selfを l: $`[0, key)`$ と r: $`[key, n)`$ に分割する
    fn split(&mut self, key: usize, l: &mut Self, r: &mut Self) {
        self.update();
        // dbg!(&self, key);
        if let Some(ref mut node) = self.0 {
            if key < node.l.size() {
                // 左側の部分木を分割する 部分木の左側がl
                let mut l_temp = Self(None);
                let mut r_temp = Self(None);
                node.l.split(key, &mut l_temp, &mut r_temp);
                swap(l, &mut l_temp);
                swap(&mut node.l, &mut Box::new(r_temp));
                swap(r, self);
            } else {
                // 右側の部分木を分割する
                let mut l_temp = Self(None);
                let mut r_temp = Self(None);
                node.r.split(key - node.l.size(), &mut l_temp, &mut r_temp);
                swap(r, &mut r_temp);
                swap(&mut node.r, &mut Box::new(l_temp));
                swap(l, self);
            }
        } else {
            swap(l, &mut Self(None));
            swap(r, &mut Self(None));
        }
        self.update();
    }

    // self の右に r をマージする
    fn merge(&mut self, r: &mut Self) {
        // dbg!(&self, &r);
        self.update();
        r.update();
        match (self.0.as_mut(), r.0.as_mut()) {
            (Some(left_node), Some(right_node)) => {
                if left_node.p > right_node.p {
                    // 左の根のほうが優先度が大きいとき、左の木の右の子と右の木をマージする
                    left_node.r.merge(r);
                } else {
                    right_node.l.merge(self);
                    swap(self, r);
                }
            }
            (Some(_), None) => (),
            (None, Some(_)) => swap(self, r),
            _ => (),
        }
        self.update();
    }
}

impl<T: PartialOrd + Copy + Debug + Display> Display for TreapNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(node) => write!(f, "{} {} {}", node.l, node.key, node.r),
            _ => write!(f, ""),
        }
    }
}

#[test]
fn format() {
    let mut treap = Treap::<usize>::new();

    for i in 0..1000000 {
        treap.insert(i, i * 2);
    }
    assert_eq!(1000000, treap.size());
}
