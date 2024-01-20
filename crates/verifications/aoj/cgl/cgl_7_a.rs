//! # Intersection of Circles(円の交差判定)

use rust_lib_for_comp::geometry::circle::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl7A;
impl verify::Solver for Cgl7A {
    const PROBLEM_ID: &'static str = "CGL_7_A";
    const EPSILON: Option<f64> = Some(1e-6);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (x1, y1, r1) = reader.v3::<f64, f64, f64>();
        let (x2, y2, r2) = reader.v3::<f64, f64, f64>();
        let c1 = Circle::new(x1, y1, r1);
        let c2 = Circle::new(x2, y2, r2);
        writeln!(
            write,
            "{}",
            match CircleIntersection::intersect(&c1, &c2) {
                CircleIntersection::NotCross => 4,
                CircleIntersection::Circumscribed => 3,
                CircleIntersection::Intersect => 2,
                CircleIntersection::Inscribed => 1,
                CircleIntersection::Included => 0,
            }
        )
        .ok();
        write.flush().ok();
    }
}