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
