pub mod avl_insert;
pub mod bst_insert;
pub mod bst_search;
pub mod bst_traversal;
pub mod heap_insert;

use crate::types::{AlgorithmId, TraceStep};

pub fn generate_trace(algorithm: &str, values: &[i32]) -> Result<Vec<TraceStep>, String> {
    let id = AlgorithmId::from_name(algorithm)
        .ok_or_else(|| format!("unknown algorithm: {}", algorithm))?;

    Ok(match id {
        AlgorithmId::BstInsert => bst_insert::bst_insert_trace(values),
        AlgorithmId::BstSearch => bst_search::bst_search_trace(values),
        AlgorithmId::HeapInsert => heap_insert::heap_insert_trace(values),
        AlgorithmId::AvlInsert => avl_insert::avl_insert_trace(values),
        AlgorithmId::BstPreOrder => bst_traversal::bst_preorder_trace(values),
        AlgorithmId::BstInOrder => bst_traversal::bst_inorder_trace(values),
        AlgorithmId::BstPostOrder => bst_traversal::bst_postorder_trace(values),
        AlgorithmId::BstLevelOrder => bst_traversal::bst_levelorder_trace(values),
        _ => return Err(format!("algorithm {} is not a tree algorithm", algorithm)),
    })
}
