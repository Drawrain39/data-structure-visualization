use crate::types::{StepType, TraceStep, build_initial_items};

pub fn topological_sort_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let n = items.len();

    if n == 0 {
        steps.push(TraceStep::new(StepType::Done, "done").with_note("图为空"));
        return steps;
    }

    steps.push(TraceStep::new(StepType::Start, "topo-sort").with_items(items.clone()).with_note("拓扑排序 (Kahn 算法)"));

    let mut indegree = vec![0usize; n];
    // Build DAG edges: node i → i+1 for demo
    let adj: Vec<Vec<usize>> = (0..n).map(|i| if i+1 < n { vec![i+1] } else { vec![] }).collect();
    for edges in adj.iter() {
        for &v in edges {
            indegree[v] += 1;
        }
    }

    let mut queue: Vec<usize> = indegree.iter().enumerate().filter(|(_,&d)| d == 0).map(|(i,_)| i).collect();
    let mut order: Vec<usize> = Vec::new();

    steps.push(TraceStep::new(StepType::Select, "init-queue").with_items(items.clone()).with_active(queue.clone()).with_note(format!("入度为 0 的节点入队")));

    while let Some(u) = queue.first().copied() {
        queue.remove(0);
        order.push(u);

        steps.push(TraceStep::new(StepType::Visit, "process").with_items(items.clone()).with_active(vec![items[u].id]).with_sorted(order.clone()).with_note(format!("处理节点 {}, 输出顺序加入", u)));

        for &v in &adj[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                queue.push(v);
                steps.push(TraceStep::new(StepType::Push, "enqueue").with_items(items.clone()).with_active(vec![items[v].id]).with_sorted(order.clone()).with_note(format!("节点 {} 入度为 0，入队", v)));
            }
        }
    }

    steps.push(TraceStep::new(StepType::Done, "done").with_items(items).with_sorted(order).with_note("拓扑排序完成"));
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_empty() { let steps = topological_sort_trace(&[]); assert!(!steps.is_empty()); assert_eq!(steps.last().unwrap().step_type, StepType::Done); }
    #[test] fn test_dag() { let steps = topological_sort_trace(&[1, 2, 3, 4, 5]); assert_eq!(steps.last().unwrap().step_type, StepType::Done); assert!(!steps.last().unwrap().sorted.is_empty()); }
}
