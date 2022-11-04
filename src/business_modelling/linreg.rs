use std::error::Error;

use rust_decimal::prelude::ToPrimitive;

#[derive(Debug)]
pub struct Linreg {}

// Least Squares Method
impl Linreg {
    pub fn linear_regress(domain: Vec<f32>, range: Vec<f32>) -> Result<(f32, f32), Box<dyn Error>> {
        let domain_len = domain.len();
        let range_len = range.len();

        if range_len == 0 || domain_len == 0 {
            // Check if domain and range is not empty
            Err("Insufficient series lengths")?;
        } else if range_len != domain_len {
            // Check if domain and range has the same lengths
            Err("Range length is not equal to domain length or vice versa")?;
        }

        // Get x̄ (domain)
        let x̄ = &domain.iter().fold(0.0, |mut acc, x| {
            acc = acc + x;
            return acc;
        }) / domain_len.to_f32().unwrap();

        // Get ȳ (range)
        let ȳ = &range.iter().fold(0.0, |mut acc, y| {
            acc = acc + y;
            return acc;
        }) / range_len.to_f32().unwrap();

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
        return Ok((0.0, 0.0));
    }
}
