use crate::types::{StepType, TraceStep, build_initial_items};

pub fn bubble_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps: Vec<TraceStep> = Vec::new();
    let mut items = build_initial_items(values);
    let n = items.len();

    steps.push(
        TraceStep::new(StepType::Start, "outer")
            .with_items(items.clone())
            .with_note("开始冒泡排序"),
    );

    let mut comparisons = 0usize;
    let mut swaps = 0usize;
    let mut writes = 0usize;
    let mut sorted_ids: Vec<String> = Vec::new();

    for i in 0..n {
        let mut swapped = false;

        for j in 0..(n - i - 1) {
            comparisons += 1;
            steps.push(
                TraceStep::new(StepType::Compare, "compare")
                    .with_items(items.clone())
                    .with_comparing(vec![items[j].id.clone(), items[j + 1].id.clone()])
                    .with_sorted(sorted_ids.clone())
                    .with_stats(comparisons, swaps, writes)
                    .with_note(format!("比较 {} 和 {}", items[j].value, items[j + 1].value)),
            );

            if items[j].value > items[j + 1].value {
                steps.push(
                    TraceStep::new(StepType::Swap, "swap")
                        .with_items(items.clone())
                        .with_swapping(vec![items[j].id.clone(), items[j + 1].id.clone()])
                        .with_sorted(sorted_ids.clone())
                        .with_stats(comparisons, swaps, writes)
                        .with_note(format!("交换 {} 和 {}", items[j].value, items[j + 1].value)),
                );
                items.swap(j, j + 1);
                swaps += 1;
                writes += 2;
                swapped = true;

                steps.push(
                    TraceStep::new(StepType::Select, "after-swap")
                        .with_items(items.clone())
                        .with_sorted(sorted_ids.clone())
                        .with_stats(comparisons, swaps, writes)
                        .with_note("交换完成"),
                );
            }
        }

        if n >= i + 1 {
            sorted_ids.push(items[n - i - 1].id.clone());
            steps.push(
                TraceStep::new(StepType::Sorted, "mark-sorted")
                    .with_items(items.clone())
                    .with_sorted(sorted_ids.clone())
                    .with_stats(comparisons, swaps, writes)
                    .with_note(format!("位置 {} 已就位", n - i - 1)),
            );
        }

        if !swapped {
            steps.push(
                TraceStep::new(StepType::Select, "early-exit")
                    .with_items(items.clone())
                    .with_sorted(sorted_ids.clone())
                    .with_stats(comparisons, swaps, writes)
                    .with_note("本轮无交换，提前结束"),
            );
            break;
        }
    }

    // Ensure every id is marked sorted in final state.
    sorted_ids = items.iter().map(|it| it.id.clone()).collect();
    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items.clone())
            .with_sorted(sorted_ids)
            .with_stats(comparisons, swaps, writes)
            .with_note("冒泡排序完成"),
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
        let steps = bubble_sort_trace(&[]);
        assert_eq!(steps.len(), 2);
        assert!(steps.last().unwrap().items.is_empty());
    }

    #[test]
    fn test_single() {
        let steps = bubble_sort_trace(&[42]);
        assert!(is_sorted(&steps.last().unwrap().items));
    }

    #[test]
    fn test_duplicate() {
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let steps = bubble_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_random() {
        let values = vec![5, 1, 4, 2, 8];
        let steps = bubble_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![1, 2, 4, 5, 8]);
    }
}
