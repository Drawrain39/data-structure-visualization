use crate::types::{ItemId, StepType, TraceStep, VisualItem, build_initial_items};

pub fn shell_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::with_capacity(values.len().max(1) * 4);
    let mut items = build_initial_items(values);
    let n = items.len();

    steps.push(
        TraceStep::new(StepType::Start, "shell-sort")
            .with_items(items.clone())
            .with_note("开始希尔排序"),
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
                .with_note("希尔排序完成"),
        );
        return steps;
    }

    let mut gap = n / 2;
    while gap > 0 {
        steps.push(
            TraceStep::new(StepType::Select, "gap")
                .with_items(items.clone())
                .with_sorted(sorted.clone())
                .with_stats(comparisons, swaps, writes)
                .with_note(format!("当前增量 gap = {}", gap)),
        );

        for i in gap..n {
            let key = items[i].clone();
            steps.push(
                TraceStep::new(StepType::Select, "select-key")
                    .with_items(items.clone())
                    .with_active(vec![key.id])
                    .with_sorted(sorted.clone())
                    .with_stats(comparisons, swaps, writes)
                    .with_note(format!("选择 key = {}", key.value)),
            );

            let mut j = i;
            while j >= gap {
                comparisons += 1;
                steps.push(
                    TraceStep::new(StepType::Compare, "compare")
                        .with_items(items.clone())
                        .with_comparing(vec![items[j - gap].id, key.id])
                        .with_sorted(sorted.clone())
                        .with_stats(comparisons, swaps, writes)
                        .with_note(format!("比较 {} 和 {}", items[j - gap].value, key.value)),
                );

                if items[j - gap].value > key.value {
                    steps.push(
                        TraceStep::new(StepType::Overwrite, "shift")
                            .with_items(items.clone())
                            .with_active(vec![items[j - gap].id])
                            .with_sorted(sorted.clone())
                            .with_stats(comparisons, swaps, writes)
                            .with_note(format!("{} 向后移动", items[j - gap].value)),
                    );
                    let moved = items[j - gap].clone();
                    items[j] = moved;
                    writes += 1;
                    swaps += 1;
                    j -= gap;
                } else {
                    break;
                }
            }

            if items[j].id != key.id {
                steps.push(
                    TraceStep::new(StepType::Overwrite, "insert")
                        .with_items(items.clone())
                        .with_active(vec![key.id])
                        .with_sorted(sorted.clone())
                        .with_stats(comparisons, swaps, writes)
                        .with_note(format!("将 key {} 插入位置 {}", key.value, j)),
                );
                items[j] = key;
                writes += 1;
            }
        }

        gap /= 2;
    }

    sorted = items.iter().map(|it| it.id).collect();
    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_sorted(sorted)
            .with_stats(comparisons, swaps, writes)
            .with_note("希尔排序完成"),
    );

    steps
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
        let steps = shell_sort_trace(&[]);
        assert_eq!(steps.len(), 2);
        assert!(steps.last().unwrap().items.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_single() {
        let steps = shell_sort_trace(&[42]);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_duplicate() {
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let steps = shell_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_random() {
        let values = vec![12, 34, 54, 2, 3, 7, 8, 99];
        let steps = shell_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![2, 3, 7, 8, 12, 34, 54, 99]);
    }

    #[test]
    fn test_reverse() {
        let values = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let steps = shell_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
