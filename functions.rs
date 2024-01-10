
pub fn linspace(a: f64, b: f64, n: usize) -> Vec<f64> {
    (0..n).map(|i| a + i as f64 * (b - a) / (n - 1) as f64).collect()
}