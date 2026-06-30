use crate::types::*;

pub fn linear_entries() -> Vec<AlgorithmEntry> {
    vec![
        AlgorithmEntry {
            algorithm: "array-insert".into(),
            meta: meta_simple!("array-insert","数组插入","Array Insert",Linear,"在数组末尾追加新元素。",time="O(1)",space="O(1)",use=["动态扩容数组"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "void arrayInsert(vector<int>& arr, int val) {",
                    "  arr.push_back(val);",
                    "}"
                ),
                python: py!("def array_insert(arr, val):", "    arr.append(val)"),
                rust: rs!(
                    "fn array_insert(arr: &mut Vec<i32>, val: i32) {",
                    "    arr.push(val);",
                    "}"
                ),
            },
            line_map: lm!("insert"=>[1],"append"=>[2],"done"=>[3]),
        },
        AlgorithmEntry {
            algorithm: "array-delete".into(),
            meta: meta_simple!("array-delete","数组删除","Array Delete",Linear,"删除数组指定位置元素。",time="O(n)",space="O(1)",use=["顺序表维护"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "void arrayDelete(vector<int>& arr, int idx) {",
                    "  if (idx < 0 || idx >= arr.size()) return;",
                    "  for (int i = idx; i < arr.size() - 1; i++)",
                    "    arr[i] = arr[i + 1];",
                    "  arr.pop_back();",
                    "}"
                ),
                python: py!(
                    "def array_delete(arr, idx):",
                    "    if 0 <= idx < len(arr):",
                    "        for i in range(idx, len(arr) - 1):",
                    "            arr[i] = arr[i + 1]",
                    "        arr.pop()"
                ),
                rust: rs!(
                    "fn array_delete(arr: &mut Vec<i32>, idx: usize) {",
                    "    if idx < arr.len() {",
                    "        arr.remove(idx);",
                    "    }",
                    "}"
                ),
            },
            line_map: lm!("delete"=>[1],"select"=>[2],"done"=>[5]),
        },
        AlgorithmEntry {
            algorithm: "linked-list-traverse".into(),
            meta: meta_simple!("linked-list-traverse","链表遍历","Linked List Traverse",Linear,"从头节点依次访问每个节点。",time="O(n)",space="O(1)",use=["链表查询"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "struct ListNode { int val; ListNode* next; };",
                    "void traverse(ListNode* head) {",
                    "  ListNode* cur = head;",
                    "  while (cur != nullptr) {",
                    "    visit(cur->val);",
                    "    cur = cur->next;",
                    "  }",
                    "}"
                ),
                python: py!(
                    "class ListNode:",
                    "    def __init__(self, val):",
                    "        self.val = val; self.next = None",
                    "def traverse(head):",
                    "    cur = head",
                    "    while cur:",
                    "        visit(cur.val); cur = cur.next"
                ),
                rust: rs!(
                    "struct ListNode { val: i32, next: Option<Box<ListNode>> }",
                    "fn traverse(mut cur: &Option<Box<ListNode>>) {",
                    "    while let Some(node) = cur {",
                    "        visit(node.val);",
                    "        cur = &node.next;",
                    "    }",
                    "}"
                ),
            },
            line_map: lm!("traverse"=>[2],"visit"=>[4],"done"=>[7]),
        },
    ]
}
