use serde::{Deserialize, Serialize};

pub type ItemId = usize;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct VisualItem {
    pub id: ItemId,
    pub value: i32,
}

impl VisualItem {
    pub fn new(id: ItemId, value: i32) -> Self {
        Self { id, value }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepType {
    Start,
    Compare,
    Swap,
    Overwrite,
    Select,
    Pivot,
    Partition,
    Merge,
    Sorted,
    Done,
    Highlight,
    Visit,
    Push,
    Pop,
    Enqueue,
    Dequeue,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceStep {
    pub step_type: StepType,
    pub line_key: String,
    pub items: Vec<VisualItem>,
    pub active: Vec<ItemId>,
    pub comparing: Vec<ItemId>,
    pub swapping: Vec<ItemId>,
    pub sorted: Vec<ItemId>,
    pub pivot_id: Option<ItemId>,
    pub min_id: Option<ItemId>,
    pub boundary_id: Option<ItemId>,
    pub range: Option<(usize, usize)>,
    pub comparisons: usize,
    pub swaps: usize,
    pub writes: usize,
    pub note: String,
    pub extra: serde_json::Value,
}

impl TraceStep {
    pub fn new(step_type: StepType, line_key: impl Into<String>) -> Self {
        Self {
            step_type,
            line_key: line_key.into(),
            items: Vec::new(),
            active: Vec::new(),
            comparing: Vec::new(),
            swapping: Vec::new(),
            sorted: Vec::new(),
            pivot_id: None,
            min_id: None,
            boundary_id: None,
            range: None,
            comparisons: 0,
            swaps: 0,
            writes: 0,
            note: String::new(),
            extra: serde_json::Value::Null,
        }
    }

    pub fn with_items(mut self, items: Vec<VisualItem>) -> Self {
        self.items = items;
        self
    }

    pub fn with_active(mut self, active: Vec<ItemId>) -> Self {
        self.active = active;
        self
    }

    pub fn with_comparing(mut self, comparing: Vec<ItemId>) -> Self {
        self.comparing = comparing;
        self
    }

    pub fn with_swapping(mut self, swapping: Vec<ItemId>) -> Self {
        self.swapping = swapping;
        self
    }

    pub fn with_sorted(mut self, sorted: Vec<ItemId>) -> Self {
        self.sorted = sorted;
        self
    }

    pub fn with_pivot(mut self, pivot_id: ItemId) -> Self {
        self.pivot_id = Some(pivot_id);
        self
    }

    pub fn with_min(mut self, min_id: ItemId) -> Self {
        self.min_id = Some(min_id);
        self
    }

    pub fn with_boundary(mut self, boundary_id: ItemId) -> Self {
        self.boundary_id = Some(boundary_id);
        self
    }

    pub fn with_range(mut self, range: (usize, usize)) -> Self {
        self.range = Some(range);
        self
    }

    pub fn with_stats(mut self, comparisons: usize, swaps: usize, writes: usize) -> Self {
        self.comparisons = comparisons;
        self.swaps = swaps;
        self.writes = writes;
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = note.into();
        self
    }

    pub fn with_extra(mut self, extra: serde_json::Value) -> Self {
        self.extra = extra;
        self
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlgorithmCategory {
    Sorting,
    Searching,
    Linear,
    StackQueue,
    Recursive,
    Tree,
    Graph,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlgorithmId {
    // Sorting
    SelectionSort,
    BubbleSort,
    InsertionSort,
    QuickSort,
    MergeSort,
    HeapSort,
    // Searching
    LinearSearch,
    BinarySearch,
    // Linear
    ArrayInsert,
    ArrayDelete,
    LinkedListTraverse,
    // Stack & Queue
    StackPushPop,
    QueueEnqueueDequeue,
    // Recursive
    Factorial,
    Fibonacci,
    TowerOfHanoi,
    // Tree
    BstInsert,
    BstSearch,
    HeapInsert,
    // Graph
    Bfs,
    Dfs,
}

impl AlgorithmId {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().replace('-', "").as_str() {
            // Sorting
            "selectionsort" | "selection" => Some(Self::SelectionSort),
            "bubblesort" | "bubble" => Some(Self::BubbleSort),
            "insertionsort" | "insertion" => Some(Self::InsertionSort),
            "quicksort" | "quick" => Some(Self::QuickSort),
            "mergesort" | "merge" => Some(Self::MergeSort),
            "heapsort" | "heap" => Some(Self::HeapSort),
            // Searching
            "linearsearch" | "linear" => Some(Self::LinearSearch),
            "binarysearch" | "binary" => Some(Self::BinarySearch),
            // Linear
            "arrayinsert" => Some(Self::ArrayInsert),
            "arraydelete" => Some(Self::ArrayDelete),
            "linkedlisttraverse" | "linkedlist" => Some(Self::LinkedListTraverse),
            // Stack/Queue
            "stackpushpop" | "stack" => Some(Self::StackPushPop),
            "queueenqueuedequeue" | "queue" => Some(Self::QueueEnqueueDequeue),
            // Recursive
            "factorial" => Some(Self::Factorial),
            "fibonacci" => Some(Self::Fibonacci),
            "towerofhanoi" | "hanoi" => Some(Self::TowerOfHanoi),
            // Tree
            "bstinsert" => Some(Self::BstInsert),
            "bstsearch" => Some(Self::BstSearch),
            "heapinsert" => Some(Self::HeapInsert),
            // Graph
            "bfs" => Some(Self::Bfs),
            "dfs" => Some(Self::Dfs),
            _ => None,
        }
    }

    pub fn category(self) -> AlgorithmCategory {
        match self {
            Self::SelectionSort |
            Self::BubbleSort |
            Self::InsertionSort |
            Self::QuickSort |
            Self::MergeSort |
            Self::HeapSort => AlgorithmCategory::Sorting,
            Self::LinearSearch |
            Self::BinarySearch => AlgorithmCategory::Searching,
            Self::ArrayInsert |
            Self::ArrayDelete |
            Self::LinkedListTraverse => AlgorithmCategory::Linear,
            Self::StackPushPop |
            Self::QueueEnqueueDequeue => AlgorithmCategory::StackQueue,
            Self::Factorial |
            Self::Fibonacci |
            Self::TowerOfHanoi => AlgorithmCategory::Recursive,
            Self::BstInsert |
            Self::BstSearch |
            Self::HeapInsert => AlgorithmCategory::Tree,
            Self::Bfs |
            Self::Dfs => AlgorithmCategory::Graph,
        }
    }
}

pub fn build_initial_items(values: &[i32]) -> Vec<VisualItem> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| VisualItem::new(i, v))
        .collect()
}

pub fn sorted_indices(items: &[VisualItem]) -> Vec<usize> {
    if items.is_empty() {
        return Vec::new();
    }
    let mut sorted = vec![items[0].id];
    for it in items.iter().skip(1) {
        if it.value >= items[it.id.min(items.len() - 1)].value {
            sorted.push(it.id);
        }
    }
    sorted
}
