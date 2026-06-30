pub mod bfs;
pub mod dfs;

use crate::types::{AlgorithmId, TraceStep};

pub fn generate_trace(algorithm: &str, values: &[i32]) -> Result<Vec<TraceStep>, String> {
    let id = AlgorithmId::from_name(algorithm)
        .ok_or_else(|| format!("unknown algorithm: {}", algorithm))?;

    Ok(match id {
        AlgorithmId::Bfs => bfs::bfs_trace(values),
        AlgorithmId::Dfs => dfs::dfs_trace(values),
        _ => return Err(format!("algorithm {} is not a graph algorithm", algorithm)),
    })
}
