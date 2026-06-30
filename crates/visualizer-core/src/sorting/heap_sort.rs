use crate::types::{build_initial_items, ItemId, StepType, TraceStep, VisualItem};

pub fn heap_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::with_capacity(values.len().max(1) * 4);
    let mut items = build_initial_items(values);
    let n = items.len();

    steps.push(
        TraceStep::new(StepType::Start, "heap-sort")
            .with_items(items.clone())
            .with_note("开始堆排序"),
    );

    let mut comparisons = 0usize;
    let mut swaps = 0usize;
    let mut writes = 0usize;
    let mut sorted: Vec<ItemId> = Vec::with_capacity(n);

    if n <= 1 {
        sorted = items.iter().map(|it| it.id).collect();
        steps.push(
            TraceStep::new(StepType::Done, "done")
                .with_items(items)
                .with_sorted(sorted)
                .with_stats(comparisons, swaps, writes)
                .with_note("堆排序完成"),
        );
        return steps;
    }

    // Build max heap
    for i in (0..n / 2).rev() {
        heapify(
            &mut items,
            n,
            i,
            &mut steps,
            &mut comparisons,
            &mut swaps,
            &mut writes,
            &sorted,
            true,
        );
    }

    // Extract elements one by one
    for i in (1..n).rev() {
        steps.push(
            TraceStep::new(StepType::Swap, "extract-max")
                .with_items(items.clone())
                .with_swapping(vec![items[0].id, items[i].id])
                .with_sorted(sorted.clone())
                .with_stats(comparisons, swaps, writes)
                .with_note(format!("将堆顶 {} 与位置 {} 交换", items[0].value, i)),
        );
        items.swap(0, i);
        swaps += 1;
        writes += 2;
        sorted.push(items[i].id);

        heapify(
            &mut items,
            i,
            0,
            &mut steps,
            &mut comparisons,
            &mut swaps,
            &mut writes,
            &sorted,
            false,
        );
    }

    sorted.push(items[0].id);
    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items.clone())
            .with_sorted(sorted.clone())
            .with_stats(comparisons, swaps, writes)
            .with_note("堆排序完成"),
    );

    steps
}

fn heapify(
    items: &mut Vec<VisualItem>,
    heap_size: usize,
    root: usize,
    steps: &mut Vec<TraceStep>,
    comparisons: &mut usize,
    swaps: &mut usize,
    writes: &mut usize,
    sorted: &Vec<ItemId>,
    building: bool,
) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < heap_size {
        *comparisons += 1;
        steps.push(
            TraceStep::new(StepType::Compare, "compare")
                .with_items(items.clone())
                .with_comparing(vec![items[largest].id, items[left].id])
                .with_sorted(sorted.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!(
                    "比较 {} 和 {}",
                    items[largest].value, items[left].value
                )),
        );
        if items[left].value > items[largest].value {
            largest = left;
        }
    }

    if right < heap_size {
        *comparisons += 1;
        steps.push(
            TraceStep::new(StepType::Compare, "compare")
                .with_items(items.clone())
                .with_comparing(vec![items[largest].id, items[right].id])
                .with_sorted(sorted.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!(
                    "比较 {} 和 {}",
                    items[largest].value, items[right].value
                )),
        );
        if items[right].value > items[largest].value {
            largest = right;
        }
    }

    if largest != root {
        let key = if building { "heapify" } else { "sink" };
        steps.push(
            TraceStep::new(StepType::Swap, key)
                .with_items(items.clone())
                .with_swapping(vec![items[root].id, items[largest].id])
                .with_sorted(sorted.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!(
                    "下沉：交换 {} 和 {}",
                    items[root].value, items[largest].value
                )),
        );
        items.swap(root, largest);
        *swaps += 1;
        *writes += 2;

        heapify(
            items,
            heap_size,
            largest,
            steps,
            comparisons,
            swaps,
            writes,
            sorted,
            building,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::VisualItem;

    fn final_values(steps: &[TraceStep]) -> Vec<i32> {
        steps
            .last()
            .unwrap()
            .items
            .iter()
            .map(|it| it.value)
            .collect()
    }

    fn is_sorted(values: &[VisualItem]) -> bool {
        values.windows(2).all(|w| w[0].value <= w[1].value)
    }

    #[test]
    fn test_empty() {
        let steps = heap_sort_trace(&[]);
        assert_eq!(steps.len(), 2);
        assert!(steps.last().unwrap().items.is_empty());
    }

    #[test]
    fn test_single() {
        let steps = heap_sort_trace(&[42]);
        assert!(is_sorted(&steps.last().unwrap().items));
    }

    #[test]
    fn test_duplicate() {
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let steps = heap_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_random() {
        let values = vec![12, 11, 13, 5, 6, 7];
        let steps = heap_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![5, 6, 7, 11, 12, 13]);
    }
}
