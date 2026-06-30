pub mod factorial;
pub mod fibonacci;
pub mod tower_of_hanoi;

use crate::types::{AlgorithmId, TraceStep};

pub fn generate_trace(algorithm: &str, values: &[i32]) -> Result<Vec<TraceStep>, String> {
    let id = AlgorithmId::from_name(algorithm)
        .ok_or_else(|| format!("unknown algorithm: {}", algorithm))?;

    Ok(match id {
        AlgorithmId::Factorial => factorial::factorial_trace(values),
        AlgorithmId::Fibonacci => fibonacci::fibonacci_trace(values),
        AlgorithmId::TowerOfHanoi => tower_of_hanoi::tower_of_hanoi_trace(values),
        _ => {
            return Err(format!(
                "algorithm {} is not a recursive algorithm",
                algorithm
            ))
        }
    })
}
