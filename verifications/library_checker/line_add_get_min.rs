//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/line_add_get_min
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    library_checker_solver::line_add_get_min::solve(io_util::IO::default());
}
