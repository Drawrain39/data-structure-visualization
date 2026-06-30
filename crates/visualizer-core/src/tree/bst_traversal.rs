use crate::types::{StepType, TraceStep, VisualItem, build_initial_items};

pub fn bst_preorder_trace(values: &[i32]) -> Vec<TraceStep> {
    bst_traversal_trace(values, "preorder", |items, _sorted, steps, seq| {
        build_preorder(items, 0, _sorted, steps, seq);
    })
}

pub fn bst_inorder_trace(values: &[i32]) -> Vec<TraceStep> {
    bst_traversal_trace(values, "inorder", |items, _sorted, steps, seq| {
        build_inorder(items, 0, _sorted, steps, seq);
    })
}

pub fn bst_postorder_trace(values: &[i32]) -> Vec<TraceStep> {
    bst_traversal_trace(values, "postorder", |items, _sorted, steps, seq| {
        build_postorder(items, 0, _sorted, steps, seq);
    })
}

pub fn bst_levelorder_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    if items.is_empty() {
        steps.push(TraceStep::new(StepType::Done, "done").with_note("树为空"));
        return steps;
    }

    steps.push(TraceStep::new(StepType::Start, "levelorder").with_items(items.clone()).with_note("开始层序遍历"));

    let mut queue = vec![0usize];
    while let Some(idx) = queue.first().copied() {
        queue.remove(0);
        if idx >= items.len() { continue; }
        steps.push(TraceStep::new(StepType::Visit, "visit").with_items(items.clone()).with_active(vec![items[idx].id]).with_note(format!("访问节点 {}", items[idx].value)));
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        if left < items.len() { queue.push(left); }
        if right < items.len() { queue.push(right); }
    }

    steps.push(TraceStep::new(StepType::Done, "done").with_items(items.clone()).with_note("层序遍历完成"));
    steps
}

type TraversalFn = fn(&Vec<VisualItem>, &Vec<&VisualItem>, &mut Vec<TraceStep>, &mut usize);

fn bst_traversal_trace(values: &[i32], key: &str, f: TraversalFn) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    if items.is_empty() {
        steps.push(TraceStep::new(StepType::Done, "done").with_note("树为空"));
        return steps;
    }
    let sorted: Vec<&VisualItem> = items.iter().collect();
    steps.push(TraceStep::new(StepType::Start, key).with_items(items.clone()).with_note(format!("开始{}遍历", match key {"preorder"=>"前序","inorder"=>"中序","postorder"=>"后序",_=>key})));
    let mut seq = 0usize;
    f(&items, &sorted, &mut steps, &mut seq);
    steps.push(TraceStep::new(StepType::Done, "done").with_items(items.clone()).with_note("遍历完成"));
    steps
}

fn build_preorder(items: &Vec<VisualItem>, idx: usize, _sorted: &Vec<&VisualItem>, steps: &mut Vec<TraceStep>, seq: &mut usize) {
    if idx >= items.len() { return; }
    *seq += 1;
    steps.push(TraceStep::new(StepType::Visit, "visit-pre").with_items(items.clone()).with_active(vec![items[idx].id]).with_note(format!("前序访问: {} (第{}个)", items[idx].value, seq)));
    build_preorder(items, 2*idx+1, _sorted, steps, seq);
    build_preorder(items, 2*idx+2, _sorted, steps, seq);
}

fn build_inorder(items: &Vec<VisualItem>, idx: usize, _sorted: &Vec<&VisualItem>, steps: &mut Vec<TraceStep>, seq: &mut usize) {
    if idx >= items.len() { return; }
    build_inorder(items, 2*idx+1, _sorted, steps, seq);
    *seq += 1;
    steps.push(TraceStep::new(StepType::Visit, "visit-in").with_items(items.clone()).with_active(vec![items[idx].id]).with_note(format!("中序访问: {} (第{}个)", items[idx].value, seq)));
    build_inorder(items, 2*idx+2, _sorted, steps, seq);
}

fn build_postorder(items: &Vec<VisualItem>, idx: usize, _sorted: &Vec<&VisualItem>, steps: &mut Vec<TraceStep>, seq: &mut usize) {
    if idx >= items.len() { return; }
    build_postorder(items, 2*idx+1, _sorted, steps, seq);
    build_postorder(items, 2*idx+2, _sorted, steps, seq);
    *seq += 1;
    steps.push(TraceStep::new(StepType::Visit, "visit-post").with_items(items.clone()).with_active(vec![items[idx].id]).with_note(format!("后序访问: {} (第{}个)", items[idx].value, seq)));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check_done(steps: &[TraceStep]) { assert_eq!(steps.last().unwrap().step_type, StepType::Done); }
    #[test] fn test_preorder() { check_done(&bst_preorder_trace(&[1,2,3,4,5])); }
    #[test] fn test_inorder() { check_done(&bst_inorder_trace(&[1,2,3,4,5])); }
    #[test] fn test_postorder() { check_done(&bst_postorder_trace(&[1,2,3,4,5])); }
    #[test] fn test_levelorder() { check_done(&bst_levelorder_trace(&[1,2,3,4,5])); }
    #[test] fn test_empty_all() {
        for steps in &[bst_preorder_trace(&[]), bst_inorder_trace(&[]), bst_postorder_trace(&[]), bst_levelorder_trace(&[])] {
            assert!(!steps.is_empty());
            assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        }
    }
}
