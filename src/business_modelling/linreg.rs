use std::error::Error;

use rust_decimal::prelude::ToPrimitive;

#[derive(Debug)]
pub struct Linreg {}

// Least Squares Method
impl Linreg {
    pub fn linear_regress(
        domain: &Vec<f32>,
        range: &Vec<f32>,
    ) -> Result<(f32, f32), Box<dyn Error>> {
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
        let domain_mean = domain.iter().fold(0.0, |mut acc, x| {
            acc = acc + x;
            return acc;
        }) / domain_len.to_f32().unwrap();

        // Get ȳ (range)
        let range_mean = range.iter().fold(0.0, |mut acc, y| {
            acc = acc + y;
            return acc;
        }) / range_len.to_f32().unwrap();

        // Loop over
        // mxNumerator = Σ(prod)
        let mut mx_numerator = 0.0;
        // mxDivisor = Σ(domainDiffSquared)
        let mut mx_divisor = 0.0;

        for idx in 0..domain_len {
            // Get domainDiff = x -  x̄
            let domain_diff = &domain[idx] - &domain_mean;
            // Get rangeDiff = y -  ȳ
            let range_diff = &range[idx] - &range_mean;

            // Get prod = domainDiff * rangeDiff
            let product = &domain_diff * &range_diff;
            mx_numerator = mx_numerator + product;

            let domain_diff_squared = f32::powf(domain_diff, 2.0);
            mx_divisor = mx_divisor + domain_diff_squared;
        }
        // mx = mxNumerator / mxDivisor
        let mx = mx_numerator / mx_divisor;
        // Get b = ȳ - mx(x̄)
        let b = &range_mean - (mx * domain_mean);

        return Ok((mx, b));
    }
}
