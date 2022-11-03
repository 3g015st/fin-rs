#[derive(Debug)]
pub struct Linreg {}

// Least Squares Method
impl Linreg {
    pub fn linear_regress(domain: Vec<f32>, range: Vec<f32>) -> (f32, f32) {
        // Check if domain and range has the same lengths

        // Check if domain and range is not empty

        // Get x̄ (domain)

        // Get ȳ (range)

        // Loop over
        // Get domainDiff = x -  x̄
        // Get rangeDiff = y -  ȳ
        // Get prod = domainDiff * rangeDiff
        // mxNumerator = Σ(prod)
        // domainDiffSquared
        // mxDivisor = Σ(domainDiffSquared)
        // End loop here
        // mx = mxNumerator / mxDivisor

        // Get b = ȳ - mx(x̄)

        // Return (mx, b)
    }
}
