use crate::types::{StepType, TraceStep, VisualItem, build_initial_items};

pub fn merge_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps: Vec<TraceStep> = Vec::new();
    let mut items = build_initial_items(values);
    let n = items.len();

    steps.push(
        TraceStep::new(StepType::Start, "merge-sort")
            .with_items(items.clone())
            .with_note("开始归并排序"),
    );

    let mut comparisons = 0usize;
    let mut swaps = 0usize;
    let mut writes = 0usize;
    let mut sorted_ids: Vec<String> = Vec::new();

    if n > 0 {
        let mut temp = items.clone();
        merge_sort_recursive(
            &mut items,
            &mut temp,
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
            .with_note("归并排序完成"),
    );

    steps
}

fn merge_sort_recursive(
    items: &mut Vec<VisualItem>,
    temp: &mut Vec<VisualItem>,
    left: usize,
    right: usize,
    steps: &mut Vec<TraceStep>,
    comparisons: &mut usize,
    swaps: &mut usize,
    writes: &mut usize,
    sorted_ids: &mut Vec<String>,
) {
    if left >= right {
        sorted_ids.push(items[left].id.clone());
        steps.push(
            TraceStep::new(StepType::Sorted, "single-sorted")
                .with_items(items.clone())
                .with_sorted(sorted_ids.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!("单个元素 {} 已有序", items[left].value)),
        );
        return;
    }

    let mid = left + (right - left) / 2;
    steps.push(
        TraceStep::new(StepType::Partition, "split")
            .with_items(items.clone())
            .with_range((left, right))
            .with_sorted(sorted_ids.clone())
            .with_stats(*comparisons, *swaps, *writes)
            .with_note(format!("将区间 [{}..{}] 拆分为两半", left, right)),
    );

    merge_sort_recursive(
        items,
        temp,
        left,
        mid,
        steps,
        comparisons,
        swaps,
        writes,
        sorted_ids,
    );
    merge_sort_recursive(
        items,
        temp,
        mid + 1,
        right,
        steps,
        comparisons,
        swaps,
        writes,
        sorted_ids,
    );

    merge(
        items,
        temp,
        left,
        mid,
        right,
        steps,
        comparisons,
        swaps,
        writes,
        sorted_ids,
    );

    // After merge, mark this whole range as sorted-ish.
    for idx in left..=right {
        if !sorted_ids.contains(&items[idx].id) {
            sorted_ids.push(items[idx].id.clone());
        }
    }
    steps.push(
        TraceStep::new(StepType::Merge, "merged")
            .with_items(items.clone())
            .with_range((left, right))
            .with_sorted(sorted_ids.clone())
            .with_stats(*comparisons, *swaps, *writes)
            .with_note(format!("区间 [{}..{}] 已合并有序", left, right)),
    );
}

fn merge(
    items: &mut Vec<VisualItem>,
    temp: &mut Vec<VisualItem>,
    left: usize,
    mid: usize,
    right: usize,
    steps: &mut Vec<TraceStep>,
    comparisons: &mut usize,
    swaps: &mut usize,
    writes: &mut usize,
    sorted_ids: &mut Vec<String>,
) {
    let mut i = left;
    let mut j = mid + 1;
    let mut k = left;

    temp[left..=right].clone_from_slice(&items[left..=right]);

    while i <= mid && j <= right {
        *comparisons += 1;
        steps.push(
            TraceStep::new(StepType::Compare, "compare")
                .with_items(items.clone())
                .with_comparing(vec![temp[i].id.clone(), temp[j].id.clone()])
                .with_range((left, right))
                .with_sorted(sorted_ids.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!("比较 {} 和 {}", temp[i].value, temp[j].value)),
        );

        if temp[i].value <= temp[j].value {
            let prev_id = items[k].id.clone();
            steps.push(
                TraceStep::new(StepType::Overwrite, "write-left")
                    .with_items(items.clone())
                    .with_active(vec![temp[i].id.clone(), prev_id.clone()])
                    .with_range((left, right))
                    .with_sorted(sorted_ids.clone())
                    .with_stats(*comparisons, *swaps, *writes)
                    .with_note(format!("将 {} 写入位置 {}", temp[i].value, k)),
            );
            items[k] = temp[i].clone();
            *writes += 1;
            i += 1;
        } else {
            let prev_id = items[k].id.clone();
            steps.push(
                TraceStep::new(StepType::Overwrite, "write-right")
                    .with_items(items.clone())
                    .with_active(vec![temp[j].id.clone(), prev_id.clone()])
                    .with_range((left, right))
                    .with_sorted(sorted_ids.clone())
                    .with_stats(*comparisons, *swaps, *writes)
                    .with_note(format!("将 {} 写入位置 {}", temp[j].value, k)),
            );
            items[k] = temp[j].clone();
            *writes += 1;
            j += 1;
        }
        *swaps += 1;
        k += 1;
    }

    while i <= mid {
        let prev_id = items[k].id.clone();
        steps.push(
            TraceStep::new(StepType::Overwrite, "write-left")
                .with_items(items.clone())
                .with_active(vec![temp[i].id.clone(), prev_id.clone()])
                .with_range((left, right))
                .with_sorted(sorted_ids.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!("将剩余 {} 写入位置 {}", temp[i].value, k)),
        );
        items[k] = temp[i].clone();
        *writes += 1;
        *swaps += 1;
        i += 1;
        k += 1;
    }

    while j <= right {
        let prev_id = items[k].id.clone();
        steps.push(
            TraceStep::new(StepType::Overwrite, "write-right")
                .with_items(items.clone())
                .with_active(vec![temp[j].id.clone(), prev_id.clone()])
                .with_range((left, right))
                .with_sorted(sorted_ids.clone())
                .with_stats(*comparisons, *swaps, *writes)
                .with_note(format!("将剩余 {} 写入位置 {}", temp[j].value, k)),
        );
        items[k] = temp[j].clone();
        *writes += 1;
        *swaps += 1;
        j += 1;
        k += 1;
    }
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
        let steps = merge_sort_trace(&[]);
        assert_eq!(steps.len(), 2);
        assert!(steps.last().unwrap().items.is_empty());
    }

    #[test]
    fn test_single() {
        let steps = merge_sort_trace(&[42]);
        assert!(is_sorted(&steps.last().unwrap().items));
    }

    #[test]
    fn test_duplicate() {
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let steps = merge_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_random() {
        let values = vec![38, 27, 43, 3, 9, 82, 10];
        let steps = merge_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![3, 9, 10, 27, 38, 43, 82]);
    }
}
