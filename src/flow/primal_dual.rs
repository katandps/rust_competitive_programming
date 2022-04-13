//! # Primal-dual法
//! 最小費用流問題を最短路反復(with Dijkstra)で解く。
//!
//! ## verify
//! [GRL_6_B](https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=6482630#2)
use crate::graph::adjacency_list::Graph;
use crate::graph::GraphTrait;
use crate::prelude::*;

macro_rules! chmin {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_min = min!($($cmps),+);if $base > cmp_min {$base = cmp_min;true} else {false}}};}
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$b} else {$a}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = min!($($rest),+);if $a > b {b} else {$a}}};
}

#[snippet(name = "primal_dual", doc_hidden)]
#[derive(Clone, Debug)]
pub struct PrimalDual<F: Copy + Debug, C: Copy + Debug> {
    graph: Graph<(C, F)>,
    rev: Vec<usize>,
}

#[snippet(name = "primal_dual", doc_hidden)]
impl<C: Copy + Debug, F: Copy + Debug> PrimalDual<C, F>
where
    F: Ord + Zero + AddAssign + SubAssign + Sub<Output = F> + Mul<C, Output = F>,
    C: Ord + Zero + BoundedAbove + AddAssign + SubAssign + Add<Output = C> + Sub<Output = C>,
{
    pub fn new(n: usize) -> Self {
        Self {
            graph: Graph::new(n),
            rev: Vec::with_capacity(n),
        }
    }
    pub fn add_edge(&mut self, src: usize, dst: usize, cap: F, cost: C) {
        let i = self.graph.add_arc(src, dst, (cap, cost));
        let j = self.graph.add_arc(dst, src, (F::zero(), C::zero() - cost));
        self.rev.resize(j + 1, 0);
        self.rev[i] = j;
        self.rev[j] = i;
    }
    pub fn min_cost_flow(&mut self, s: usize, t: usize, mut f: F) -> Option<F> {
        let v = self.graph.size();
        let mut ret = F::zero();
        let mut pq: BinaryHeap<Reverse<(C, usize)>> = BinaryHeap::new();
        let mut potential = vec![C::zero(); v];
        while f > F::zero() {
            let mut min_cost = vec![C::max_value(); v];
            let mut prev_edge: Vec<Option<usize>> = vec![None; v];
            min_cost[s] = C::zero();
            pq.push(Reverse((C::zero(), s)));
            while let Some(Reverse((cost, src))) = pq.pop() {
                if min_cost[src] < cost {
                    continue;
                }
                for &i in &self.graph.index[src] {
                    let (_src, dst, (cap, cost)) = self.graph.edges[i];
                    if min_cost[src] == C::max_value() {
                        continue;
                    }
                    let next_cost = min_cost[src] + cost + potential[src] - potential[dst];
                    if cap > F::zero() && chmin!(min_cost[dst], next_cost) {
                        prev_edge[dst] = Some(i);
                        pq.push(Reverse((min_cost[dst], dst)))
                    }
                }
            }
            if min_cost[t] == C::max_value() {
                return None;
            }
            for i in 0..v {
                potential[i] += min_cost[i];
            }
            let mut addflow = f;
            let mut cur = t;
            while cur != s {
                let prev_i = prev_edge[cur].unwrap();
                let (src, _dst, (cap, _cost)) = self.graph.edges[prev_i];
                chmin!(addflow, cap);
                cur = src;
            }
            f -= addflow;
            ret += addflow * potential[t];
            let mut cur = t;
            while cur != s {
                let prev_i = prev_edge[cur].unwrap();
                self.graph.edges[prev_i].2 .0 -= addflow;
                self.graph.edges[self.rev[prev_i]].2 .0 += addflow;
                cur = self.graph.edges[prev_i].0;
            }
        }
        Some(ret)
    }
}

impl<F: Copy + Debug, C: Copy + Debug + Add<Output = C> + Display> PrimalDual<F, C> {
    pub fn result(&self) {
        for i in 0..self.graph.size() {
            for &j in &self.graph.index[i] {
                let (src, dst, (cap, _cost)) = self.graph.edges[j];
                let (_rev_src, _rev_dst, (rev_cap, _rev_cost)) = self.graph.edges[self.rev[j]];
                println!("{} -> {} (flow: {}/{})", src, dst, rev_cap, rev_cap + cap);
            }
        }
    }
}