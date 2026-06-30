use crate::types::{build_initial_items, StepType, TraceStep, VisualItem};

pub fn dijkstra_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let n = items.len();

    if n == 0 {
        steps.push(TraceStep::new(StepType::Done, "done").with_note("图为空"));
        return steps;
    }

    steps.push(
        TraceStep::new(StepType::Start, "dijkstra")
            .with_items(items.clone())
            .with_note("Dijkstra 最短路径"),
    );

    let mut dist = vec![i32::MAX; n];
    let mut visited = vec![false; n];
    dist[0] = 0;

    steps.push(
        TraceStep::new(StepType::Select, "init")
            .with_items(items.clone())
            .with_active(vec![items[0].id])
            .with_note("起点设为节点 0，距离 = 0"),
    );

    for _ in 0..n {
        let mut u = None;
        let mut min_dist = i32::MAX;
        for i in 0..n {
            if !visited[i] && dist[i] < min_dist {
                min_dist = dist[i];
                u = Some(i);
            }
        }
        if u.is_none() {
            break;
        }
        let u = u.unwrap();
        visited[u] = true;

        steps.push(
            TraceStep::new(StepType::Visit, "visit")
                .with_items(items.clone())
                .with_active(vec![items[u].id])
                .with_sorted(
                    visited
                        .iter()
                        .enumerate()
                        .filter(|(_, &v)| v)
                        .map(|(i, _)| i)
                        .collect(),
                )
                .with_note(format!("选中节点 {}, 距离 = {}", u, dist[u])),
        );

        // Relax edges: connect to neighbors with weight = |value diff|
        for v in 0..n {
            if visited[v] {
                continue;
            }
            let w = (items[u].value - items[v].value).abs();
            if dist[u] != i32::MAX && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                steps.push(
                    TraceStep::new(StepType::Relax, "relax")
                        .with_items(items.clone())
                        .with_comparing(vec![items[u].id, items[v].id])
                        .with_extra(
                            serde_json::json!({"from":u,"to":v,"weight":w,"new_dist":dist[v]}),
                        )
                        .with_note(format!(
                            "松弛边 {}-{} (权={}): 更新距离为 {}",
                            u, v, w, dist[v]
                        )),
                );
            }
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_note("Dijkstra 完成"),
    );
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let steps = dijkstra_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
    #[test]
    fn test_path() {
        let steps = dijkstra_trace(&[0, 4, 2, 7, 1, 5]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        assert!(steps.iter().any(|s| s.step_type == StepType::Relax));
    }
}
