/// Return a value of Geo(p) distribution
pub fn geo(p: f64) -> usize {
    let u = fastrand::f64();
    u.log(1.0 - p).floor() as usize + 1
}
