use crate::types::{ItemId, StepType, TraceStep, VisualItem, build_initial_items};

pub fn counting_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::with_capacity(values.len().max(1) * 4);
    let items = build_initial_items(values);
    let n = items.len();

    steps.push(
        TraceStep::new(StepType::Start, "counting-sort")
            .with_items(items.clone())
            .with_note("开始计数排序"),
    );

    let mut comparisons = 0usize;
    let swaps = 0usize;
    let mut writes = 0usize;

    if n <= 1 {
        let sorted: Vec<ItemId> = items.iter().map(|it| it.id).collect();
        steps.push(TraceStep::new(StepType::Done, "done").with_items(items).with_sorted(sorted).with_stats(comparisons, swaps, writes).with_note("计数排序完成"));
        return steps;
    }

    let max_val = items.iter().map(|it| it.value).max().unwrap_or(0);
    let min_val = items.iter().map(|it| it.value).min().unwrap_or(0);
    let range = (max_val - min_val + 1) as usize;

    steps.push(
        TraceStep::new(StepType::Select, "find-range")
            .with_items(items.clone())
            .with_stats(comparisons, swaps, writes)
            .with_note(format!("值范围: {} ~ {}", min_val, max_val)),
    );

    let mut count = vec![0usize; range];
    for item in items.iter() {
        count[(item.value - min_val) as usize] += 1;
        comparisons += 1;
        steps.push(
            TraceStep::new(StepType::Compare, "count")
                .with_items(items.clone())
                .with_active(vec![item.id])
                .with_stats(comparisons, swaps, writes)
                .with_note(format!("计数: {} 出现 {} 次", item.value, count[(item.value - min_val) as usize])),
        );
    }

    for i in 1..range {
        count[i] += count[i - 1];
    }

    steps.push(
        TraceStep::new(StepType::Select, "prefix-sum")
            .with_items(items.clone())
            .with_stats(comparisons, swaps, writes)
            .with_note("计算前缀和"),
    );

    let mut output = vec![VisualItem::new(9999, 0); n];
    for item in items.iter().rev() {
        let idx = (item.value - min_val) as usize;
        count[idx] -= 1;
        let pos = count[idx];
        output[pos] = item.clone();
        writes += 1;
        steps.push(
            TraceStep::new(StepType::Overwrite, "place")
                .with_items(output.clone())
                .with_active(vec![item.id])
                .with_stats(comparisons, swaps, writes)
                .with_note(format!("将 {} 放到位置 {}", item.value, pos)),
        );
    }

    let sorted: Vec<ItemId> = output.iter().map(|it| it.id).collect();
    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(output)
            .with_sorted(sorted)
            .with_stats(comparisons, swaps, writes)
            .with_note("计数排序完成"),
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

    #[test] fn test_empty() { let steps = counting_sort_trace(&[]); assert_eq!(steps.len(), 2); assert!(steps.last().unwrap().items.is_empty()); }
    #[test] fn test_single() { let steps = counting_sort_trace(&[42]); assert!(is_sorted(&steps.last().unwrap().items)); }
    #[test] fn test_duplicate() { let values = vec![3, 1, 4, 1, 5, 9, 2, 6]; let steps = counting_sort_trace(&values); assert!(is_sorted(&steps.last().unwrap().items)); assert_eq!(final_values(&steps), vec![1, 1, 2, 3, 4, 5, 6, 9]); }
    #[test] fn test_random() { let values = vec![5, 3, 8, 4, 2]; let steps = counting_sort_trace(&values); assert!(is_sorted(&steps.last().unwrap().items)); assert_eq!(final_values(&steps), vec![2, 3, 4, 5, 8]); }
}
