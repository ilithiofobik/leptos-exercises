#[test]
fn geo_distr_test() {
    use crate::utils::rand::geo;

    let num_of_exps = 10_000_000;
    let num_of_exps_f = num_of_exps as f64;
    let tol = 0.05;
    let ps = [0.25, 0.5, 0.75];
    let ks = [1, 2, 4];

    for p in ps {
        let exps: Vec<i32> = (0..num_of_exps).map(|_| geo(p) as i32).collect();
        for k in ks {
            let er = p * (1.0 - p).powi(k - 1) * num_of_exps_f;
            let rr = exps.iter().filter(|&&l| l == k).count() as f64;
            let abs_err = (rr - er).abs();
            assert!(abs_err < tol * er);
        }
    }
}
