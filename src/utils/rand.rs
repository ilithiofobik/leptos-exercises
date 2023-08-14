/// Return a value of Geo(p) distribution
pub fn geo(p: f64) -> usize {
    let u = fastrand::f64();
    let q = 1.0 - p;
    (u.log(q) + 1.0).floor() as usize
}