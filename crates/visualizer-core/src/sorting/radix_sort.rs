use crate::types::{build_initial_items, ItemId, StepType, TraceStep, VisualItem};

pub fn radix_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::with_capacity(values.len().max(1) * 6);
    let mut items = build_initial_items(values);
    let n = items.len();

    steps.push(
        TraceStep::new(StepType::Start, "radix-sort")
            .with_items(items.clone())
            .with_note("开始基数排序"),
    );

    let mut comparisons = 0usize;
    let mut swaps = 0usize;
    let mut writes = 0usize;

    if n <= 1 {
        let sorted: Vec<ItemId> = items.iter().map(|it| it.id).collect();
        steps.push(
            TraceStep::new(StepType::Done, "done")
                .with_items(items)
                .with_sorted(sorted)
                .with_stats(comparisons, swaps, writes)
                .with_note("基数排序完成"),
        );
        return steps;
    }

    let max_val = items.iter().map(|it| it.value).max().unwrap_or(0);
    let mut exp = 1;
    let mut output = vec![VisualItem::new(9999, 0); n];

    while max_val / exp > 0 {
        steps.push(
            TraceStep::new(StepType::Select, "digit")
                .with_items(items.clone())
                .with_stats(comparisons, swaps, writes)
                .with_note(format!("按第 {} 位排序", exp)),
        );

        let mut count = [0usize; 10];
        for item in items.iter() {
            let digit = ((item.value / exp) % 10) as usize;
            count[digit] += 1;
            comparisons += 1;
        }

        for i in 1..10 {
            count[i] += count[i - 1];
        }

        for item in items.iter().rev() {
            let digit = ((item.value / exp) % 10) as usize;
            count[digit] -= 1;
            let pos = count[digit];
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

        items.clone_from_slice(&output);
        swaps += n;
        exp *= 10;
    }

    let sorted: Vec<ItemId> = items.iter().map(|it| it.id).collect();
    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_sorted(sorted)
            .with_stats(comparisons, swaps, writes)
            .with_note("基数排序完成"),
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
        let steps = radix_sort_trace(&[]);
        assert_eq!(steps.len(), 2);
    }
    #[test]
    fn test_single() {
        let steps = radix_sort_trace(&[42]);
        assert!(is_sorted(&steps.last().unwrap().items));
    }
    #[test]
    fn test_random() {
        let values = vec![170, 45, 75, 90, 802, 24, 2, 66];
        let steps = radix_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
        assert_eq!(final_values(&steps), vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }
    #[test]
    fn test_duplicate() {
        let values = vec![53, 89, 53, 10, 21, 10];
        let steps = radix_sort_trace(&values);
        assert!(is_sorted(&steps.last().unwrap().items));
    }
}
