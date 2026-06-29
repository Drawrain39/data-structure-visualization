use crate::types::{StepType, TraceStep, VisualItem, build_initial_items};

pub fn quick_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps: Vec<TraceStep> = Vec::new();
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
    let mut sorted_ids: Vec<String> = Vec::new();

    if n > 0 {
        quick_sort_recursive(
            &mut items,
            0,
            n - 1,
            &mut steps,
            &mut comparisons,
            &mut swaps,
            &mut writes,
            &mut sorted_ids,
        );
    }

    sorted_ids = items.iter().map(|it| it.id.clone()).collect();
    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items.clone())
            .with_sorted(sorted_ids)
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
    sorted_ids: &mut Vec<String>,
) {
    if low < high {
        steps.push(
            TraceStep::new(StepType::Partition, "partition-call")
                .with_items(items.clone())
                .with_range((low, high))
                .with_sorted(sorted_ids.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!("对区间 [{}..{}] 进行划分", low, high)),
        );

        let p = partition(items, low, high, steps, comparisons, swaps, writes, sorted_ids);

        sorted_ids.push(items[p].id.clone());
        steps.push(
            TraceStep::new(StepType::Sorted, "pivot-sorted")
                .with_items(items.clone())
                .with_sorted(sorted_ids.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!("枢轴 {} 已归位", items[p].value)),
        );

        if p > 0 {
            quick_sort_recursive(
                items,
                low,
                p - 1,
                steps,
                comparisons,
                swaps,
                writes,
                sorted_ids,
            );
        }
        quick_sort_recursive(items, p + 1, high, steps, comparisons, swaps, writes, sorted_ids);
    } else if low == high {
        sorted_ids.push(items[low].id.clone());
        steps.push(
            TraceStep::new(StepType::Sorted, "single-sorted")
                .with_items(items.clone())
                .with_sorted(sorted_ids.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!("单个元素 {} 已有序", items[low].value)),
        );
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
    sorted_ids: &mut Vec<String>,
) -> usize {
    let pivot_idx = high;
    let pivot_id = items[pivot_idx].id.clone();
    steps.push(
        TraceStep::new(StepType::Pivot, "choose-pivot")
            .with_items(items.clone())
            .with_pivot(pivot_id.clone())
            .with_range((low, high))
            .with_sorted(sorted_ids.clone())
            .with_stats(*comparisons, *swaps, *writes)
            .with_note(format!("选择枢轴 {}", items[pivot_idx].value)),
    );

    let mut i = low;
    for j in low..high {
        *comparisons += 1;
        steps.push(
            TraceStep::new(StepType::Compare, "compare")
                .with_items(items.clone())
                .with_comparing(vec![items[j].id.clone(), pivot_id.clone()])
                .with_pivot(pivot_id.clone())
                .with_sorted(sorted_ids.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!("比较 {} 和枢轴 {}", items[j].value, items[pivot_idx].value)),
        );

        if items[j].value <= items[pivot_idx].value {
            if i != j {
                steps.push(
                    TraceStep::new(StepType::Swap, "swap")
                        .with_items(items.clone())
                        .with_swapping(vec![items[i].id.clone(), items[j].id.clone()])
                        .with_pivot(pivot_id.clone())
                        .with_sorted(sorted_ids.clone())
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
                .with_swapping(vec![items[i].id.clone(), items[pivot_idx].id.clone()])
                .with_pivot(pivot_id.clone())
                .with_sorted(sorted_ids.clone())
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
