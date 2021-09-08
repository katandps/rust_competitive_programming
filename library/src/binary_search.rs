#[allow(dead_code)]
mod binary_search {
    fn search() -> isize {
        let mut ng: isize = -1;
        let mut ok: isize = 100;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if is_ok(mid) {
                ok = mid
            } else {
                ng = mid
            }
        }
        ok
    }

    fn is_ok(_key: isize) -> bool {
        return true;
    }
}
