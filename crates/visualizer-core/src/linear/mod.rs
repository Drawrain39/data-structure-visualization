pub mod array_delete;
pub mod array_insert;
pub mod linked_list_traverse;

use crate::types::{AlgorithmId, TraceStep};

pub fn generate_trace(algorithm: &str, values: &[i32]) -> Result<Vec<TraceStep>, String> {
    let id = AlgorithmId::from_name(algorithm)
        .ok_or_else(|| format!("unknown algorithm: {}", algorithm))?;

    Ok(match id {
        AlgorithmId::ArrayInsert => array_insert::array_insert_trace(values),
        AlgorithmId::ArrayDelete => array_delete::array_delete_trace(values),
        AlgorithmId::LinkedListTraverse => linked_list_traverse::linked_list_traverse_trace(values),
        _ => return Err(format!("algorithm {} is not a linear structure algorithm", algorithm)),
    })
}
