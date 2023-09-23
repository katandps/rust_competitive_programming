// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/0275

use io_util::*;

fn main() {
    let mut io = IO::default();
    let xy = io.vec2::<f64, f64>(3);
    let ans = circle::cgl_7_c(&xy);
    io.out(format!("{} {} {}\n", ans.0, ans.1, ans.2));

    io.flush();
}