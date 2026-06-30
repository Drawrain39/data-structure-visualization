use crate::types::{ItemId, StepType, TraceStep, VisualItem, build_initial_items};

pub fn bucket_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::with_capacity(values.len().max(1) * 5);
    let items = build_initial_items(values);
    let n = items.len();

    steps.push(TraceStep::new(StepType::Start, "bucket-sort").with_items(items.clone()).with_note("开始桶排序"));

    let comparisons = 0usize;
    let mut swaps = 0usize;
    let mut writes = 0usize;

    if n <= 1 {
        let sorted: Vec<ItemId> = items.iter().map(|it| it.id).collect();
        steps.push(TraceStep::new(StepType::Done, "done").with_items(items).with_sorted(sorted).with_stats(comparisons, swaps, writes).with_note("桶排序完成"));
        return steps;
    }

    let max_val = items.iter().map(|it| it.value).max().unwrap_or(0);
    let min_val = items.iter().map(|it| it.value).min().unwrap_or(0);
    let bucket_count = ((n as f64).sqrt() as usize).max(1);
    let bucket_size = ((max_val - min_val + 1) as f64 / bucket_count as f64).ceil() as i32;

    steps.push(TraceStep::new(StepType::Select, "create-buckets").with_items(items.clone()).with_stats(comparisons, swaps, writes).with_note(format!("创建 {} 个桶", bucket_count)));

    let mut buckets: Vec<Vec<VisualItem>> = vec![Vec::new(); bucket_count];
    for item in items.iter() {
        let idx = (((item.value - min_val) as i32 / bucket_size.max(1)) as usize).min(bucket_count - 1);
        buckets[idx].push(item.clone());
        writes += 1;
        steps.push(TraceStep::new(StepType::Push, "distribute").with_items(items.clone()).with_active(vec![item.id]).with_stats(comparisons, swaps, writes).with_note(format!("将 {} 放入桶 {}", item.value, idx)));
    }

    let mut result: Vec<VisualItem> = Vec::with_capacity(n);
    for (bi, bucket) in buckets.iter_mut().enumerate() {
        if bucket.is_empty() { continue; }
        bucket.sort_by_key(|it| it.value);
        swaps += bucket.len();
        steps.push(TraceStep::new(StepType::Select, "sort-bucket").with_items(items.clone()).with_active(bucket.iter().map(|it| it.id).collect()).with_stats(comparisons, swaps, writes).with_note(format!("对桶 {} 排序", bi)));
        for item in bucket.iter() {
            result.push(item.clone());
        }
    }

    let sorted: Vec<ItemId> = result.iter().map(|it| it.id).collect();
    steps.push(TraceStep::new(StepType::Done, "done").with_items(result).with_sorted(sorted).with_stats(comparisons, swaps, writes).with_note("桶排序完成"));
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::VisualItem;
    fn final_values(steps: &[TraceStep]) -> Vec<i32> { steps.last().unwrap().items.iter().map(|it| it.value).collect() }
    fn is_sorted(values: &[VisualItem]) -> bool { values.windows(2).all(|w| w[0].value <= w[1].value) }
    #[test] fn test_empty() { let steps = bucket_sort_trace(&[]); assert_eq!(steps.len(), 2); assert!(steps.last().unwrap().items.is_empty()); }
    #[test] fn test_single() { let steps = bucket_sort_trace(&[42]); assert!(is_sorted(&steps.last().unwrap().items)); }
    #[test] fn test_random() { let values = vec![78, 17, 39, 26, 72, 94, 21, 12]; let steps = bucket_sort_trace(&values); assert!(is_sorted(&steps.last().unwrap().items)); assert_eq!(final_values(&steps), vec![12, 17, 21, 26, 39, 72, 78, 94]); }
    #[test] fn test_duplicate() { let values = vec![5, 3, 5, 1, 3]; let steps = bucket_sort_trace(&values); assert!(is_sorted(&steps.last().unwrap().items)); }
}
