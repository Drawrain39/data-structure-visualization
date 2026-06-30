use crate::types::{StepType, TraceStep, build_initial_items};

pub fn bfs_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    if items.is_empty() {
        steps.push(TraceStep::new(StepType::Done, "done").with_note("图为空"));
        return steps;
    }

    let mut visited = vec![false; items.len()];
    let mut queue = vec![0usize];
    visited[0] = true;

    steps.push(
        TraceStep::new(StepType::Start, "bfs")
            .with_items(items.clone())
            .with_active(vec![items[0].id])
            .with_note("从节点 0 开始 BFS"),
    );

    while let Some(node) = queue.first().copied() {
        queue.remove(0);
        steps.push(
            TraceStep::new(StepType::Visit, "visit")
                .with_items(items.clone())
                .with_active(vec![items[node].id])
                .with_note(format!("访问节点 {}", node)),
        );

        // Treat next index as adjacent for demo purposes
        if node + 1 < items.len() && !visited[node + 1] {
            visited[node + 1] = true;
            queue.push(node + 1);
            steps.push(
                TraceStep::new(StepType::Enqueue, "enqueue")
                    .with_items(items.clone())
                    .with_active(vec![items[node + 1].id])
                    .with_note(format!("将节点 {} 入队", node + 1)),
            );
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_note("BFS 完成"),
    );
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let steps = bfs_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_bfs() {
        let steps = bfs_trace(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        assert!(steps.iter().any(|s| s.step_type == StepType::Visit));
    }
}
