use crate::types::{ItemId, StepType, TraceStep, VisualItem, build_initial_items};

pub fn quick_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::with_capacity(values.len().max(1) * 4);
    let mut items = build_initial_items(values);
    let n = items.len();

    steps.push(
        TraceStep::new(StepType::Start, "quick-sort")
            .with_items(items.clone())
            .with_note("开始快速排序"),
    );

    let mut comparisons = 0usize;
    let mut swaps = 0usize;
    let mut writes = 0usize;
    let mut sorted: Vec<ItemId> = Vec::with_capacity(n);

    if n > 0 {
        quick_sort_recursive(
            &mut items,
            0,
            n - 1,
            &mut steps,
            &mut comparisons,
            &mut swaps,
            &mut writes,
            &mut sorted,
        );
    }

    sorted = items.iter().map(|it| it.id).collect();
    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_sorted(sorted)
            .with_stats(comparisons, swaps, writes)
            .with_note("快速排序完成"),
    );

    steps
}

fn quick_sort_recursive(
    items: &mut Vec<VisualItem>,
    low: usize,
    high: usize,
    steps: &mut Vec<TraceStep>,
    comparisons: &mut usize,
    swaps: &mut usize,
    writes: &mut usize,
    sorted: &mut Vec<ItemId>,
) {
    if low < high {
        let p = partition(items, low, high, steps, comparisons, swaps, writes, sorted);

        sorted.push(items[p].id);

        if p > 0 {
            quick_sort_recursive(
                items,
                low,
                p - 1,
                steps,
                comparisons,
                swaps,
                writes,
                sorted,
            );
        }
        quick_sort_recursive(items, p + 1, high, steps, comparisons, swaps, writes, sorted);
    } else if low == high {
        sorted.push(items[low].id);
    }
}

fn partition(
    items: &mut Vec<VisualItem>,
    low: usize,
    high: usize,
    steps: &mut Vec<TraceStep>,
    comparisons: &mut usize,
    swaps: &mut usize,
    writes: &mut usize,
    sorted: &mut Vec<ItemId>,
) -> usize {
    let pivot_idx = high;
    let pivot_id = items[pivot_idx].id;

    let mut i = low;
    for j in low..high {
        *comparisons += 1;
        steps.push(
            TraceStep::new(StepType::Compare, "compare")
                .with_items(items.clone())
                .with_comparing(vec![items[j].id, pivot_id])
                .with_pivot(pivot_id)
                .with_sorted(sorted.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!("比较 {} 和枢轴 {}", items[j].value, items[pivot_idx].value)),
        );

        if items[j].value <= items[pivot_idx].value {
            if i != j {
                steps.push(
                    TraceStep::new(StepType::Swap, "swap")
                        .with_items(items.clone())
                        .with_swapping(vec![items[i].id, items[j].id])
                        .with_pivot(pivot_id)
                        .with_sorted(sorted.clone())
                        .with_stats(*comparisons, *swaps, *writes)
                        .with_note(format!("将 {} 换到左侧", items[j].value)),
                );
                items.swap(i, j);
                *swaps += 1;
                *writes += 2;
            }
            i += 1;
        }
    }

    if i != high {
        steps.push(
            TraceStep::new(StepType::Swap, "pivot-swap")
                .with_items(items.clone())
                .with_swapping(vec![items[i].id, items[pivot_idx].id])
                .with_pivot(pivot_id)
                .with_sorted(sorted.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note("将枢轴放入正确位置"),
        );
        items.swap(i, high);
        *swaps += 1;
        *writes += 2;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::VisualItem;

    fn final_values(steps: &[TraceStep]) -> Vec<i32> {
        steps.last().unwrap().items.iter().map(|it| it.value).collect()
    }

    fn is_sorted(values: &[VisualItem]) -> bool {
        values.windows(2).all(|w| w[0].value <= w[1].value)
    }

    #[test]
    fn test_empty() {
        let steps = quick_sort_trace(&[]);
        assert_eq!(steps.len(), 2);
        assert!(steps.last().unwrap().items.is_empty());
    }

    #[test]
    fn test_single() {
        let steps = quick_sort_trace(&[42]);
        assert!(is_sorted(&steps.last().unwrap().items));
    }

    #[test]
    fn test_duplicate() {
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let steps = quick_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_random() {
        let values = vec![10, 7, 8, 9, 1, 5];
        let steps = quick_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![1, 5, 7, 8, 9, 10]);
    }
}
