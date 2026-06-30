use crate::types::{build_initial_items, ItemId, StepType, TraceStep, VisualItem};

pub fn prim_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let n = items.len();

    if n == 0 {
        steps.push(TraceStep::new(StepType::Done, "done").with_note("图为空"));
        return steps;
    }

    steps.push(
        TraceStep::new(StepType::Start, "prim")
            .with_items(items.clone())
            .with_note("Prim 最小生成树"),
    );

    let mut selected = vec![false; n];
    let mut mst: Vec<usize> = Vec::new();
    selected[0] = true;
    mst.push(items[0].id);

    steps.push(
        TraceStep::new(StepType::Select, "start")
            .with_items(items.clone())
            .with_active(vec![items[0].id])
            .with_note("从节点 0 开始"),
    );

    for _ in 0..n - 1 {
        let mut min_w = i32::MAX;
        let mut min_edge = (0, 0);

        for u in 0..n {
            if !selected[u] {
                continue;
            }
            for v in 0..n {
                if selected[v] {
                    continue;
                }
                let w = (items[u].value - items[v].value).abs();
                steps.push(
                    TraceStep::new(StepType::Compare, "compare")
                        .with_items(items.clone())
                        .with_comparing(vec![items[u].id, items[v].id])
                        .with_extra(serde_json::json!({"weight":w}))
                        .with_note(format!("比较边 {}-{} (权={})", u, v, w)),
                );
                if w < min_w {
                    min_w = w;
                    min_edge = (v, u);
                }
            }
        }

        if min_w < i32::MAX {
            selected[min_edge.0] = true;
            mst.push(items[min_edge.0].id);
            steps.push(
                TraceStep::new(StepType::Select, "add")
                    .with_items(items.clone())
                    .with_swapping(vec![items[min_edge.0].id, items[min_edge.1].id])
                    .with_sorted(mst.clone())
                    .with_note(format!(
                        "加入节点 {}: 边 {}-{} (权={})",
                        min_edge.0, min_edge.0, min_edge.1, min_w
                    )),
            );
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_sorted(mst)
            .with_note("Prim 完成"),
    );
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let steps = prim_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
    #[test]
    fn test_mst() {
        let steps = prim_trace(&[4, 8, 1, 3, 7]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
}
