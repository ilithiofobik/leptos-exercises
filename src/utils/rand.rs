fn geo_cdf(q: f64, k: i32) -> f64 {
    1.0 - q.powi(k - 1)
}

/// Return a value of Geo(p) distribution
pub fn geo(p: f64) -> usize {
    let x = fastrand::f64();
    let q = 1.0 - p;
    let mut left = 1;
    let mut right = 2;

    while x > geo_cdf(q, right) {
        right *= 2
    }

    while right - left > 1 {
        let mid = (left + right) / 2;
        if geo_cdf(q, mid) < x {
            left = mid;
        } else {
            right = mid;
        }
    }

    left as usize
}

#[test]
fn geo_distr_test() {
    let num_of_exps = 10_000_000;
    let num_of_exps_f = num_of_exps as f64;
    let tol = 0.01;
    let expected = [(1, 0.5), (2, 0.25), (3, 0.125), (4, 0.0625), (5, 0.03125)];
    let exps: Vec<usize> = (0..num_of_exps).map(|_| geo(0.5)).collect();
    for (ev, er) in expected {
        let ratio = exps.iter().filter(|&&l| l == ev).count() as f64 / num_of_exps_f;
        let rel_err = (ratio - er).abs() / er;
        assert!(rel_err < tol);
    }
}
