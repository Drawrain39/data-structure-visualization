use crate::types::{ItemId, StepType, TraceStep, VisualItem, build_initial_items};

pub fn kruskal_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let n = items.len();

    if n == 0 {
        steps.push(TraceStep::new(StepType::Done, "done").with_note("图为空"));
        return steps;
    }

    steps.push(TraceStep::new(StepType::Start, "kruskal").with_items(items.clone()).with_note("Kruskal 最小生成树"));

    // Build edges between consecutive nodes with weight = |value diff|
    let mut edges: Vec<(usize, usize, i32)> = Vec::new();
    for i in 0..n {
        for j in (i+1)..n {
            if (i+1 == j) || (i == 0 && j == n-1) {
                edges.push((i, j, (items[i].value - items[j].value).abs()));
            }
        }
    }
    edges.sort_by_key(|&(_, _, w)| w);

    let mut parent: Vec<usize> = (0..n).collect();
    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x { parent[x] = find(parent, parent[x]); }
        parent[x]
    }
    fn union(parent: &mut [usize], a: usize, b: usize) {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra != rb { parent[ra] = rb; }
    }

    let mut mst: Vec<usize> = Vec::new();

    for &(u, v, w) in edges.iter() {
        steps.push(TraceStep::new(StepType::Compare, "consider").with_items(items.clone()).with_comparing(vec![items[u].id, items[v].id]).with_extra(serde_json::json!({"weight":w})).with_note(format!("考虑边 {}-{} (权={})", u, v, w)));

        if find(&mut parent, u) != find(&mut parent, v) {
            union(&mut parent, u, v);
            mst.push(u);
            mst.push(v);
            steps.push(TraceStep::new(StepType::Select, "select").with_items(items.clone()).with_swapping(vec![items[u].id, items[v].id]).with_sorted(mst.clone()).with_note(format!("选中边 {}-{}", u, v)));
        }
    }

    steps.push(TraceStep::new(StepType::Done, "done").with_items(items).with_sorted(mst).with_note("Kruskal 完成"));
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_empty() { let steps = kruskal_trace(&[]); assert!(!steps.is_empty()); assert_eq!(steps.last().unwrap().step_type, StepType::Done); }
    #[test] fn test_mst() { let steps = kruskal_trace(&[4, 8, 1, 3, 7]); assert_eq!(steps.last().unwrap().step_type, StepType::Done); }
}
