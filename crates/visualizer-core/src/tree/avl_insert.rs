use crate::types::{build_initial_items, StepType, TraceStep, VisualItem};

pub fn avl_insert_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    steps.push(
        TraceStep::new(StepType::Start, "avl-insert")
            .with_items(items.clone())
            .with_note("AVL 树插入演示"),
    );

    let mut inserted: Vec<usize> = Vec::new();

    for item in items.iter() {
        steps.push(
            TraceStep::new(StepType::Select, "insert-val")
                .with_items(items.clone())
                .with_active(vec![item.id])
                .with_note(format!("插入 {}", item.value)),
        );

        inserted.push(item.id);
        inserted.sort_by_key(|&id| items[id].value);

        // Find position in sorted order
        let _pos = inserted.iter().position(|&id| id == item.id).unwrap_or(0);
        steps.push(
            TraceStep::new(StepType::Compare, "compare")
                .with_items(items.clone())
                .with_active(vec![item.id])
                .with_note(format!("按 BST 规则比较")),
        );

        // Compute balance factors (simplified)
        if inserted.len() >= 3 {
            let root_idx = inserted.len() / 2;
            let left_height = root_idx;
            let right_height = inserted.len() - root_idx - 1;
            let balance: i32 = left_height as i32 - right_height as i32;

            if balance.abs() >= 2 {
                let rotation_type = if balance > 0 { "LL/R" } else { "RR/L" };
                steps.push(
                    TraceStep::new(StepType::Rotate, "rotate")
                        .with_items(items.clone())
                        .with_active(vec![items[inserted[root_idx]].id])
                        .with_pivot(items[inserted[root_idx]].id)
                        .with_note(format!("{} 旋转，平衡因子 = {}", rotation_type, balance)),
                );
            }
        }
    }

    // Build balanced BST representation
    let mut balanced: Vec<usize> = Vec::new();
    fn build_balanced(inserted: &[usize], items: &[VisualItem], balanced: &mut Vec<usize>) {
        if inserted.is_empty() {
            return;
        }
        let mid = inserted.len() / 2;
        balanced.push(inserted[mid]);
        build_balanced(&inserted[..mid], items, balanced);
        build_balanced(&inserted[mid + 1..], items, balanced);
    }
    build_balanced(&inserted, &items, &mut balanced);

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items.clone())
            .with_sorted(balanced)
            .with_note("AVL 插入完成"),
    );
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let steps = avl_insert_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
    #[test]
    fn test_insert() {
        let steps = avl_insert_trace(&[50, 30, 70, 20, 40, 60, 80]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
    #[test]
    fn test_unbalanced() {
        let steps = avl_insert_trace(&[10, 20, 30, 40, 50]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        assert!(steps.len() > 5);
    }
}
