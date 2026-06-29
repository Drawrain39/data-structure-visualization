use crate::types::{StepType, TraceStep, build_initial_items};

pub fn insertion_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps: Vec<TraceStep> = Vec::new();
    let mut items = build_initial_items(values);
    let n = items.len();

    steps.push(
        TraceStep::new(StepType::Start, "outer")
            .with_items(items.clone())
            .with_note("开始插入排序"),
    );

    let mut comparisons = 0usize;
    let mut swaps = 0usize;
    let mut writes = 0usize;
    let mut sorted_ids: Vec<String> = Vec::new();

    if n > 0 {
        sorted_ids.push(items[0].id.clone());
        steps.push(
            TraceStep::new(StepType::Sorted, "init-sorted")
                .with_items(items.clone())
                .with_sorted(sorted_ids.clone())
                .with_stats(comparisons, swaps, writes)
                .with_note("第一个元素天然有序"),
        );
    }

    for i in 1..n {
        let key = items[i].clone();
        steps.push(
            TraceStep::new(StepType::Select, "select-key")
                .with_items(items.clone())
                .with_active(vec![key.id.clone()])
                .with_sorted(sorted_ids.clone())
                .with_stats(comparisons, swaps, writes)
                .with_note(format!("选择当前元素 {} 作为 key", key.value)),
        );

        let mut j = i;
        while j > 0 {
            comparisons += 1;
            steps.push(
                TraceStep::new(StepType::Compare, "compare")
                    .with_items(items.clone())
                    .with_comparing(vec![items[j - 1].id.clone(), key.id.clone()])
                    .with_sorted(sorted_ids.clone())
                    .with_stats(comparisons, swaps, writes)
                    .with_note(format!("比较 {} 和 key {}", items[j - 1].value, key.value)),
            );

            if items[j - 1].value > key.value {
                steps.push(
                    TraceStep::new(StepType::Overwrite, "shift")
                        .with_items(items.clone())
                        .with_active(vec![items[j - 1].id.clone()])
                        .with_sorted(sorted_ids.clone())
                        .with_stats(comparisons, swaps, writes)
                        .with_note(format!("{} 向后移动", items[j - 1].value)),
                );
                let moved = items[j - 1].clone();
                items[j] = moved;
                writes += 1;
                swaps += 1;
                j -= 1;
            } else {
                break;
            }
        }

        if items[j].id != key.id {
            steps.push(
                TraceStep::new(StepType::Overwrite, "insert")
                    .with_items(items.clone())
                    .with_active(vec![key.id.clone()])
                    .with_sorted(sorted_ids.clone())
                    .with_stats(comparisons, swaps, writes)
                    .with_note(format!("将 key {} 插入位置 {}", key.value, j)),
            );
            items[j] = key;
            writes += 1;
        }

        sorted_ids.push(items[i].id.clone());
        steps.push(
            TraceStep::new(StepType::Sorted, "mark-sorted")
                .with_items(items.clone())
                .with_sorted(sorted_ids.clone())
                .with_stats(comparisons, swaps, writes)
                .with_note(format!("前 {} 个元素已有序", i + 1)),
        );
    }

    sorted_ids = items.iter().map(|it| it.id.clone()).collect();
    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items.clone())
            .with_sorted(sorted_ids)
            .with_stats(comparisons, swaps, writes)
            .with_note("插入排序完成"),
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
        let steps = insertion_sort_trace(&[]);
        assert_eq!(steps.len(), 2);
        assert!(steps.last().unwrap().items.is_empty());
    }

    #[test]
    fn test_single() {
        let steps = insertion_sort_trace(&[42]);
        assert!(is_sorted(&steps.last().unwrap().items));
    }

    #[test]
    fn test_duplicate() {
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let steps = insertion_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_random() {
        let values = vec![12, 11, 13, 5, 6];
        let steps = insertion_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![5, 6, 11, 12, 13]);
    }
}
