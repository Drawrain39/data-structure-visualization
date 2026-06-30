pub mod binary_search;
pub mod hash_search;
pub mod interpolation_search;
pub mod linear_search;

use crate::types::{AlgorithmId, TraceStep};

pub fn generate_trace(algorithm: &str, values: &[i32]) -> Result<Vec<TraceStep>, String> {
    let id = AlgorithmId::from_name(algorithm)
        .ok_or_else(|| format!("unknown algorithm: {}", algorithm))?;

    Ok(match id {
        AlgorithmId::LinearSearch => linear_search::linear_search_trace(values),
        AlgorithmId::BinarySearch => binary_search::binary_search_trace(values),
        AlgorithmId::InterpolationSearch => {
            interpolation_search::interpolation_search_trace(values)
        }
        AlgorithmId::HashSearch => hash_search::hash_search_trace(values),
        _ => {
            return Err(format!(
                "algorithm {} is not a searching algorithm",
                algorithm
            ))
        }
    })
}
