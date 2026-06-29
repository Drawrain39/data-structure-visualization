pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;

use crate::types::TraceStep;

pub fn generate_trace(algorithm: &str, values: &[i32]) -> Result<Vec<TraceStep>, String> {
    use crate::types::AlgorithmId;

    let id = AlgorithmId::from_name(algorithm)
        .ok_or_else(|| format!("unknown algorithm: {}", algorithm))?;

    Ok(match id {
        AlgorithmId::SelectionSort => selection_sort::selection_sort_trace(values),
        AlgorithmId::BubbleSort => bubble_sort::bubble_sort_trace(values),
        AlgorithmId::InsertionSort => insertion_sort::insertion_sort_trace(values),
        AlgorithmId::QuickSort => quick_sort::quick_sort_trace(values),
        AlgorithmId::MergeSort => merge_sort::merge_sort_trace(values),
    })
}
