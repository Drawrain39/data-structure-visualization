use crate::types::{build_initial_items, ItemId, StepType, TraceStep};

pub fn selection_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::with_capacity(values.len().max(1) * 3);
    let mut items = build_initial_items(values);
    let n = items.len();

    steps.push(
        TraceStep::new(StepType::Start, "outer")
            .with_items(items.clone())
            .with_note("开始选择排序"),
    );

    let mut comparisons = 0usize;
    let mut swaps = 0usize;
    let mut writes = 0usize;
    let mut sorted: Vec<ItemId> = Vec::with_capacity(n);

    for i in 0..n {
        if i > 0 {
            sorted.push(items[i - 1].id);
        }

        let mut min_idx = i;
        if i + 1 < n {
            steps.push(
                TraceStep::new(StepType::Select, "select-min")
                    .with_items(items.clone())
                    .with_active(vec![items[i].id])
                    .with_min(items[min_idx].id)
                    .with_sorted(sorted.clone())
                    .with_stats(comparisons, swaps, writes)
                    .with_note(format!("假设位置 {} 为当前最小值", i)),
            );
        }

        for j in (i + 1)..n {
            comparisons += 1;
            steps.push(
                TraceStep::new(StepType::Compare, "compare")
                    .with_items(items.clone())
                    .with_comparing(vec![items[min_idx].id, items[j].id])
                    .with_min(items[min_idx].id)
                    .with_sorted(sorted.clone())
                    .with_stats(comparisons, swaps, writes)
                    .with_note(format!(
                        "比较 {} 与 {}",
                        items[min_idx].value, items[j].value
                    )),
            );

            if items[j].value < items[min_idx].value {
                min_idx = j;
                steps.push(
                    TraceStep::new(StepType::Select, "update-min")
                        .with_items(items.clone())
                        .with_active(vec![items[min_idx].id])
                        .with_min(items[min_idx].id)
                        .with_sorted(sorted.clone())
                        .with_stats(comparisons, swaps, writes)
                        .with_note(format!("发现新的最小值 {}", items[min_idx].value)),
                );
            }
        }

        if min_idx != i {
            steps.push(
                TraceStep::new(StepType::Swap, "swap")
                    .with_items(items.clone())
                    .with_swapping(vec![items[i].id, items[min_idx].id])
                    .with_sorted(sorted.clone())
                    .with_stats(comparisons, swaps, writes)
                    .with_note(format!("交换位置 {} 和 {}", i, min_idx)),
            );
            items.swap(i, min_idx);
            swaps += 1;
            writes += 2;
        }
    }

    sorted = items.iter().map(|it| it.id).collect();
    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_sorted(sorted)
            .with_stats(comparisons, swaps, writes)
            .with_note("选择排序完成"),
    );

    steps
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
        let steps = selection_sort_trace(&[]);
        assert_eq!(steps.len(), 2);
        assert!(steps.last().unwrap().items.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_single() {
        let steps = selection_sort_trace(&[42]);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_duplicate() {
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let steps = selection_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        let sorted = final_values(&steps);
        assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_random() {
        let values = vec![64, 25, 12, 22, 11];
        let steps = selection_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        let sorted = final_values(&steps);
        assert_eq!(sorted, vec![11, 12, 22, 25, 64]);
    }
}
