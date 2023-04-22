// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_4_A
use convex_hull::Polygon;
use io_util::*;
use plane_float::Vector;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let points = xy
        .into_iter()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let convex_hull = Polygon::convex_hull(points, true);
    let mut poly = Polygon::new(convex_hull.nodes.into_iter().map(Vector::swap).collect());
    // yについて正規化
    poly.normalize();
    let ans = Polygon::new(poly.nodes.into_iter().map(Vector::swap).collect());

    io.out(ans.number_of_sides().line());
    for p in ans.nodes {
        io.out(format!("{} {}", p.x, p.y).line());
    }
    io.flush();
}
