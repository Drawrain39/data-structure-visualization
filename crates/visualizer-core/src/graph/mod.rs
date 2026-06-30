pub mod bfs;
pub mod dfs;
pub mod dijkstra;
pub mod kruskal;
pub mod prim;
pub mod topological_sort;

use crate::types::{AlgorithmId, TraceStep};

pub fn generate_trace(algorithm: &str, values: &[i32]) -> Result<Vec<TraceStep>, String> {
    let id = AlgorithmId::from_name(algorithm)
        .ok_or_else(|| format!("unknown algorithm: {}", algorithm))?;

    Ok(match id {
        AlgorithmId::Bfs => bfs::bfs_trace(values),
        AlgorithmId::Dfs => dfs::dfs_trace(values),
        AlgorithmId::Dijkstra => dijkstra::dijkstra_trace(values),
        AlgorithmId::TopologicalSort => topological_sort::topological_sort_trace(values),
        AlgorithmId::Kruskal => kruskal::kruskal_trace(values),
        AlgorithmId::Prim => prim::prim_trace(values),
        _ => return Err(format!("algorithm {} is not a graph algorithm", algorithm)),
    })
}
