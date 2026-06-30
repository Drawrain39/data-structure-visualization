pub mod dp;
pub mod graph;
pub mod linear;
pub mod recursive;
pub mod searching;
pub mod sorting;
pub mod stack_queue;
pub mod tree;
pub mod types;

pub use types::*;

pub fn generate_trace(algorithm: &str, values: &[i32]) -> Result<Vec<TraceStep>, String> {
    let id = AlgorithmId::from_name(algorithm)
        .ok_or_else(|| format!("unknown algorithm: {}", algorithm))?;

    match id.category() {
        AlgorithmCategory::Sorting => sorting::generate_trace(algorithm, values),
        AlgorithmCategory::Searching => searching::generate_trace(algorithm, values),
        AlgorithmCategory::Linear => linear::generate_trace(algorithm, values),
        AlgorithmCategory::StackQueue => stack_queue::generate_trace(algorithm, values),
        AlgorithmCategory::Recursive => recursive::generate_trace(algorithm, values),
        AlgorithmCategory::Tree => tree::generate_trace(algorithm, values),
        AlgorithmCategory::Graph => graph::generate_trace(algorithm, values),
        AlgorithmCategory::Dp => dp::generate_trace(algorithm, values),
    }
}

pub fn list_algorithms(category: AlgorithmCategory) -> Vec<AlgorithmId> {
    use AlgorithmId::*;
    match category {
        AlgorithmCategory::Sorting => vec![
            SelectionSort,
            BubbleSort,
            InsertionSort,
            MergeSort,
            QuickSort,
            HeapSort,
            ShellSort,
            CountingSort,
            BucketSort,
            RadixSort,
        ],
        AlgorithmCategory::Searching => {
            vec![LinearSearch, BinarySearch, InterpolationSearch, HashSearch]
        }
        AlgorithmCategory::Linear => vec![ArrayInsert, ArrayDelete, LinkedListTraverse],
        AlgorithmCategory::StackQueue => vec![StackPushPop, QueueEnqueueDequeue],
        AlgorithmCategory::Recursive => vec![Factorial, Fibonacci, TowerOfHanoi],
        AlgorithmCategory::Tree => vec![
            BstInsert,
            BstSearch,
            HeapInsert,
            AvlInsert,
            BstPreOrder,
            BstInOrder,
            BstPostOrder,
            BstLevelOrder,
        ],
        AlgorithmCategory::Graph => vec![Bfs, Dfs, Dijkstra, TopologicalSort, Kruskal, Prim],
        AlgorithmCategory::Dp => vec![FibonacciDp, Knapsack, LCS, LIS],
    }
}
