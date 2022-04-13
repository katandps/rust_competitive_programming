//! # ディニッツ法
//! 最大流問題を解く
//!
//! ## 計算量
//! $` O (V^{2} E)`$
//! ただし、ほとんどの場合さらに高速に動作する
use crate::prelude::*;

#[snippet(name = "dinic", doc_hidden)]
struct Edge {
    pub to: usize,
    pub rev: usize,
    pub cap: i64,
}

#[snippet(name = "dinic", doc_hidden)]
pub struct Dinic {
    g: Vec<Vec<Edge>>,
    level: Vec<i32>,
    iter: Vec<usize>,
}

#[snippet(name = "dinic", doc_hidden)]
impl Dinic {
    pub fn new(v: usize) -> Dinic {
        let mut g: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..v {
            g.push(Vec::new());
        }
        Dinic {
            g,
            level: vec![0; v],
            iter: vec![0; v],
        }
    }

    ///辺と、最大流量を設定する
    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let to_len = self.g[to].len();
        let from_len = self.g[from].len();
        self.g[from].push(Edge {
            to,
            rev: to_len,
            cap,
        });
        self.g[to].push(Edge {
            to: from,
            rev: from_len,
            cap: 0,
        });
    }

    fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
        if v == t {
            return f;
        }
        while self.iter[v] < self.g[v].len() {
            let (e_cap, e_to, e_rev);
            {
                let edge = &mut self.g[v][self.iter[v]];
                e_cap = edge.cap;
                e_to = edge.to;
                e_rev = edge.rev;
            }
            if e_cap > 0 && self.level[v] < self.level[e_to] {
                let d = self.dfs(e_to, t, std::cmp::min(f, e_cap));
                if d > 0 {
                    self.g[v][self.iter[v]].cap -= d;
                    self.g[e_to][e_rev].cap += d;
                    return d;
                }
            }
            self.iter[v] += 1;
        }
        0
    }

    fn bfs(&mut self, s: usize) {
        let v = self.level.len();
        self.level = vec![-1; v];
        self.level[s] = 0;
        let mut deque = VecDeque::new();
        deque.push_back(s);
        while !deque.is_empty() {
            let v = deque.pop_front().unwrap();
            for e in &self.g[v] {
                if e.cap > 0 && self.level[e.to] < 0 {
                    self.level[e.to] = self.level[v] + 1;
                    deque.push_back(e.to);
                }
            }
        }
    }

    /// # 最大フロー問題を解く
    /// ## 計算慮
    /// $`O(V^2E)`$
    pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let v = self.level.len();
        let mut flow: i64 = 0;
        loop {
            self.bfs(s);
            if self.level[t] < 0 {
                return flow;
            }
            self.iter = vec![0; v];
            loop {
                let f = self.dfs(s, t, std::i64::MAX);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
    }
}
