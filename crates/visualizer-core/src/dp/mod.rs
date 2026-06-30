pub mod fibonacci_dp;
pub mod knapsack;
pub mod lcs;
pub mod lis;

use crate::types::{AlgorithmId, TraceStep};

pub fn generate_trace(algorithm: &str, values: &[i32]) -> Result<Vec<TraceStep>, String> {
    let id = AlgorithmId::from_name(algorithm)
        .ok_or_else(|| format!("unknown algorithm: {}", algorithm))?;

    Ok(match id {
        AlgorithmId::FibonacciDp => fibonacci_dp::fibonacci_dp_trace(values),
        AlgorithmId::Knapsack => knapsack::knapsack_trace(values),
        AlgorithmId::LCS => lcs::lcs_trace(values),
        AlgorithmId::LIS => lis::lis_trace(values),
        _ => return Err(format!("algorithm {} is not a DP algorithm", algorithm)),
    })
}
