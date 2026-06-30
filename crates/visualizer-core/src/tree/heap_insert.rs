use crate::types::{StepType, TraceStep, build_initial_items};

pub fn heap_insert_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    steps.push(
        TraceStep::new(StepType::Start, "heap-insert")
            .with_items(items.clone())
            .with_note("最大堆插入演示"),
    );

    let mut heap: Vec<usize> = Vec::new();

    for item in items.iter() {
        heap.push(item.id);
        steps.push(
            TraceStep::new(StepType::Push, "insert")
                .with_items(items.clone())
                .with_active(heap.clone())
                .with_note(format!("将 {} 插入堆末尾", item.value)),
        );

        let mut i = heap.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if items[heap[i]].value > items[heap[parent]].value {
                steps.push(
                    TraceStep::new(StepType::Swap, "sift-up")
                        .with_items(items.clone())
                        .with_swapping(vec![heap[i], heap[parent]])
                        .with_note(format!(
                            "上浮：{} 大于父节点 {}",
                            items[heap[i]].value, items[heap[parent]].value
                        )),
                );
                heap.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_note("堆插入完成"),
    );

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let steps = heap_insert_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_insert() {
        let steps = heap_insert_trace(&[10, 20, 5, 30, 15]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        assert!(steps.iter().any(|s| s.step_type == StepType::Push));
    }
}
