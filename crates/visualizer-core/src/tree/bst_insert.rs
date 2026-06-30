use crate::types::{StepType, TraceStep, build_initial_items};

pub fn bst_insert_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    steps.push(
        TraceStep::new(StepType::Start, "bst-insert")
            .with_items(items.clone())
            .with_note("二叉搜索树插入演示"),
    );

    #[derive(Clone)]
    struct Node {
        id: usize,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }

    let mut root: Option<Box<Node>> = None;

    for item in items.iter() {
        steps.push(
            TraceStep::new(StepType::Select, "insert")
                .with_items(items.clone())
                .with_active(vec![item.id])
                .with_note(format!("插入 {}", item.value)),
        );

        if root.is_none() {
            root = Some(Box::new(Node {
                id: item.id,
                left: None,
                right: None,
            }));
            steps.push(
                TraceStep::new(StepType::Highlight, "root")
                    .with_items(items.clone())
                    .with_active(vec![item.id])
                    .with_note(format!("{} 作为根节点", item.value)),
            );
            continue;
        }

        let mut current = root.as_mut().unwrap();
        loop {
            let current_id = current.id;
            steps.push(
                TraceStep::new(StepType::Compare, "compare")
                    .with_items(items.clone())
                    .with_comparing(vec![item.id, current_id])
                    .with_note(format!(
                        "比较 {} 和当前节点 {}",
                        item.value, items[current_id].value
                    )),
            );

            if item.value < items[current_id].value {
                if current.left.is_none() {
                    current.left = Some(Box::new(Node {
                        id: item.id,
                        left: None,
                        right: None,
                    }));
                    steps.push(
                        TraceStep::new(StepType::Highlight, "left-child")
                            .with_items(items.clone())
                            .with_active(vec![item.id])
                            .with_note(format!("{} 作为左子节点", item.value)),
                    );
                    break;
                }
                current = current.left.as_mut().unwrap();
            } else {
                if current.right.is_none() {
                    current.right = Some(Box::new(Node {
                        id: item.id,
                        left: None,
                        right: None,
                    }));
                    steps.push(
                        TraceStep::new(StepType::Highlight, "right-child")
                            .with_items(items.clone())
                            .with_active(vec![item.id])
                            .with_note(format!("{} 作为右子节点", item.value)),
                    );
                    break;
                }
                current = current.right.as_mut().unwrap();
            }
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_note("二叉搜索树插入完成"),
    );

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let steps = bst_insert_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_insert() {
        let steps = bst_insert_trace(&[50, 30, 70, 20, 40, 60, 80]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        assert!(steps.iter().any(|s| s.step_type == StepType::Compare));
    }
}
