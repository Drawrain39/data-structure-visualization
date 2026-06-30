use crate::types::{StepType, TraceStep, VisualItem, build_initial_items};

pub fn dfs_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    if items.is_empty() {
        steps.push(TraceStep::new(StepType::Done, "done").with_note("图为空"));
        return steps;
    }

    let mut visited = vec![false; items.len()];

    steps.push(
        TraceStep::new(StepType::Start, "dfs")
            .with_items(items.clone())
            .with_active(vec![items[0].id])
            .with_note("从节点 0 开始 DFS"),
    );

    dfs_recursive(0, &items, &mut visited, &mut steps);

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_note("DFS 完成"),
    );
    steps
}

fn dfs_recursive(
    node: usize,
    items: &[VisualItem],
    visited: &mut [bool],
    steps: &mut Vec<TraceStep>,
) {
    visited[node] = true;
    steps.push(
        TraceStep::new(StepType::Visit, "visit")
            .with_items(items.to_vec())
            .with_active(vec![items[node].id])
            .with_note(format!("访问节点 {}", node)),
    );

    if node + 1 < items.len() && !visited[node + 1] {
        dfs_recursive(node + 1, items, visited, steps);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let steps = dfs_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_dfs() {
        let steps = dfs_trace(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        assert!(steps.iter().any(|s| s.step_type == StepType::Visit));
    }
}
