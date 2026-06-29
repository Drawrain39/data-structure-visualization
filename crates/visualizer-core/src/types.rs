use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct VisualItem {
    pub id: String,
    pub value: i32,
}

impl VisualItem {
    pub fn new(id: impl Into<String>, value: i32) -> Self {
        Self {
            id: id.into(),
            value,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceStep {
    pub step_type: StepType,
    pub line_key: String,
    pub items: Vec<VisualItem>,
    pub active: Vec<String>,
    pub comparing: Vec<String>,
    pub swapping: Vec<String>,
    pub sorted: Vec<String>,
    pub pivot_id: Option<String>,
    pub min_id: Option<String>,
    pub boundary_id: Option<String>,
    pub range: Option<(usize, usize)>,
    pub comparisons: usize,
    pub swaps: usize,
    pub writes: usize,
    pub note: String,
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
        }
    }

    pub fn with_items(mut self, items: Vec<VisualItem>) -> Self {
        self.items = items;
        self
    }

    pub fn with_active(mut self, active: Vec<String>) -> Self {
        self.active = active;
        self
    }

    pub fn with_comparing(mut self, comparing: Vec<String>) -> Self {
        self.comparing = comparing;
        self
    }

    pub fn with_swapping(mut self, swapping: Vec<String>) -> Self {
        self.swapping = swapping;
        self
    }

    pub fn with_sorted(mut self, sorted: Vec<String>) -> Self {
        self.sorted = sorted;
        self
    }

    pub fn with_pivot(mut self, pivot_id: impl Into<String>) -> Self {
        self.pivot_id = Some(pivot_id.into());
        self
    }

    pub fn with_min(mut self, min_id: impl Into<String>) -> Self {
        self.min_id = Some(min_id.into());
        self
    }

    pub fn with_boundary(mut self, boundary_id: impl Into<String>) -> Self {
        self.boundary_id = Some(boundary_id.into());
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
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlgorithmId {
    SelectionSort,
    BubbleSort,
    InsertionSort,
    QuickSort,
    MergeSort,
}

impl AlgorithmId {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().replace('-', "").as_str() {
            "selectionsort" | "selection" => Some(Self::SelectionSort),
            "bubblesort" | "bubble" => Some(Self::BubbleSort),
            "insertionsort" | "insertion" => Some(Self::InsertionSort),
            "quicksort" | "quick" => Some(Self::QuickSort),
            "mergesort" | "merge" => Some(Self::MergeSort),
            _ => None,
        }
    }
}

pub fn build_initial_items(values: &[i32]) -> Vec<VisualItem> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| VisualItem::new(format!("item-{}", i), v))
        .collect()
}
