use crate::types::*;

pub fn stack_queue_entries() -> Vec<AlgorithmEntry> {
    vec![
        AlgorithmEntry { algorithm: "stack-push-pop".into(),
            meta: meta_simple!("stack-push-pop","栈操作","Stack Push/Pop",StackQueue,"先入后出：依次入栈再出栈。",time="O(1)",space="O(n)",use=["表达式求值","回溯"]),
            samples: CodeSamples {
                cpp: cpp!("void stackDemo(vector<int>& arr) {","  stack<int> s;","  for (int v : arr) s.push(v);","  while (!s.empty()) {","    int top = s.top();","    s.pop();","  }","}"),
                python: py!("def stack_demo(arr):","    stack = []","    for v in arr:","        stack.append(v)  # push","    while stack:","        top = stack.pop()"),
                rust: rs!("fn stack_demo(arr: &[i32]) {","    let mut stack = Vec::new();","    for &v in arr { stack.push(v); }","    while let Some(top) = stack.pop() {","        // process top","    }","}"),
            },
            line_map: lm!("stack"=>[1],"push"=>[3],"pop"=>[5],"done"=>[7]),
        },
        AlgorithmEntry { algorithm: "queue-enqueue-dequeue".into(),
            meta: meta_simple!("queue-enqueue-dequeue","队列操作","Queue Enqueue/Dequeue",StackQueue,"先入先出：依次入队再出队。",time="O(1)",space="O(n)",use=["BFS","任务调度"]),
            samples: CodeSamples {
                cpp: cpp!("void queueDemo(vector<int>& arr) {","  queue<int> q;","  for (int v : arr) q.push(v);","  while (!q.empty()) {","    int front = q.front();","    q.pop();","  }","}"),
                python: py!("def queue_demo(arr):","    from collections import deque","    q = deque()","    for v in arr:","        q.append(v)  # enqueue","    while q:","        front = q.popleft()  # dequeue"),
                rust: rs!("use std::collections::VecDeque;","fn queue_demo(arr: &[i32]) {","    let mut q = VecDeque::new();","    for &v in arr { q.push_back(v); }","    while let Some(front) = q.pop_front() {","        // process front","    }","}"),
            },
            line_map: lm!("queue"=>[2],"enqueue"=>[4],"dequeue"=>[5],"done"=>[7]),
        },
    ]
}
