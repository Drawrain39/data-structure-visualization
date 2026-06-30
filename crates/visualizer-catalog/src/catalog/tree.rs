use crate::types::*;

pub fn tree_entries() -> Vec<AlgorithmEntry> {
    vec![
        AlgorithmEntry {
            algorithm: "bst-insert".into(),
            meta: meta_simple!("bst-insert","二叉搜索树插入","BST Insert",Tree,"按大小关系插入到左子树或右子树。",time="O(h)",space="O(1)",use=["动态有序集合"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "struct Node { int val; Node *left, *right; };",
                    "Node* bstInsert(Node* root, int val) {",
                    "  if (!root) return new Node{val, 0, 0};",
                    "  if (val < root->val)",
                    "    root->left = bstInsert(root->left, val);",
                    "  else",
                    "    root->right = bstInsert(root->right, val);",
                    "  return root;",
                    "}"
                ),
                python: py!(
                    "class Node:",
                    "    def __init__(self, val):",
                    "        self.val = val",
                    "        self.left = self.right = None",
                    "def bst_insert(root, val):",
                    "    if not root: return Node(val)",
                    "    if val < root.val:",
                    "        root.left = bst_insert(root.left, val)",
                    "    else:",
                    "        root.right = bst_insert(root.right, val)",
                    "    return root"
                ),
                rust: rs!(
                    "struct Node { val: i32, left: Option<Box<Node>>, right: Option<Box<Node>> }",
                    "fn bst_insert(root: Option<Box<Node>>, val: i32) -> Option<Box<Node>> {",
                    "    match root {",
                    "        None => Some(Box::new(Node{val,left:None,right:None})),",
                    "        Some(mut n) => {",
                    "            if val < n.val { n.left = bst_insert(n.left, val); }",
                    "            else { n.right = bst_insert(n.right, val); }",
                    "            Some(n)",
                    "        }",
                    "    }",
                    "}"
                ),
            },
            line_map: lm!("bst-insert"=>[5],"insert"=>[6],"root"=>[7],"compare"=>[8],"left-child"=>[9],"right-child"=>[10],"done"=>[11]),
        },
        AlgorithmEntry {
            algorithm: "bst-search".into(),
            meta: meta_simple!("bst-search","二叉搜索树查找","BST Search",Tree,"在二叉搜索树中按大小关系查找目标。",time="O(h)",space="O(1)",use=["有序数据查找"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "bool bstSearch(Node* root, int target) {",
                    "  Node* cur = root;",
                    "  while (cur) {",
                    "    if (cur->val == target) return true;",
                    "    if (target < cur->val) cur = cur->left;",
                    "    else cur = cur->right;",
                    "  }",
                    "  return false;",
                    "}"
                ),
                python: py!(
                    "def bst_search(root, target):",
                    "    cur = root",
                    "    while cur:",
                    "        if cur.val == target:",
                    "            return True",
                    "        if target < cur.val:",
                    "            cur = cur.left",
                    "        else:",
                    "            cur = cur.right",
                    "    return False"
                ),
                rust: rs!(
                    "fn bst_search(root: &Option<Box<Node>>, target: i32) -> bool {",
                    "    let mut cur = root;",
                    "    while let Some(n) = cur {",
                    "        if n.val == target { return true; }",
                    "        if target < n.val { cur = &n.left; }",
                    "        else { cur = &n.right; }",
                    "    }",
                    "    false",
                    "}"
                ),
            },
            line_map: lm!("bst-search"=>[1],"compare"=>[4],"found"=>[4],"done"=>[8]),
        },
        AlgorithmEntry {
            algorithm: "heap-insert".into(),
            meta: meta_simple!("heap-insert","堆插入","Heap Insert",Tree,"将元素插入堆末尾并上浮调整。",time="O(log n)",space="O(1)",use=["优先队列"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "void heapInsert(vector<int>& heap, int val) {",
                    "  heap.push_back(val);",
                    "  int i = heap.size() - 1;",
                    "  while (i > 0 && heap[(i-1)/2] < heap[i]) {",
                    "    swap(heap[i], heap[(i-1)/2]);",
                    "    i = (i - 1) / 2;",
                    "  }",
                    "}"
                ),
                python: py!(
                    "def heap_insert(heap, val):",
                    "    heap.append(val)",
                    "    i = len(heap) - 1",
                    "    while i > 0 and heap[(i-1)//2] < heap[i]:",
                    "        heap[i], heap[(i-1)//2] = heap[(i-1)//2], heap[i]",
                    "        i = (i - 1) // 2"
                ),
                rust: rs!(
                    "fn heap_insert(heap: &mut Vec<i32>, val: i32) {",
                    "    heap.push(val);",
                    "    let mut i = heap.len() - 1;",
                    "    while i > 0 && heap[(i-1)/2] < heap[i] {",
                    "        heap.swap(i, (i-1)/2);",
                    "        i = (i - 1) / 2;",
                    "    }",
                    "}"
                ),
            },
            line_map: lm!("heap-insert"=>[1],"insert"=>[2],"sift-up"=>[5],"done"=>[7]),
        },
        AlgorithmEntry {
            algorithm: "avl-insert".into(),
            meta: meta_simple!("avl-insert","AVL 插入","AVL Insert",Tree,"插入后通过旋转保持平衡，确保高度差不超过 1。",time="O(log n)",space="O(1)",use=["频繁增删的有序集合"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "struct AVLNode { int val, h; AVLNode *l,*r; };",
                    "int height(AVLNode* n) { return n?n->h:0; }",
                    "AVLNode* rotateRight(AVLNode* y) {",
                    "  AVLNode* x=y->l; y->l=x->r; x->r=y;",
                    "  y->h=1+max(height(y->l),height(y->r));",
                    "  x->h=1+max(height(x->l),height(x->r)); return x;",
                    "}",
                    "AVLNode* avlInsert(AVLNode* n,int v) {",
                    "  if(!n) return new AVLNode{v,1,0,0};",
                    "  if(v<n->val) n->l=avlInsert(n->l,v);",
                    "  else n->r=avlInsert(n->r,v);",
                    "  n->h=1+max(height(n->l),height(n->r));",
                    "  int b=height(n->l)-height(n->r);",
                    "  if(b>1&&v<n->l->val) return rotateRight(n);",
                    "  return n;",
                    "}"
                ),
                python: py!(
                    "class AVLNode:",
                    "    def __init__(self,v): self.val=v; self.h=1",
                    "        self.left=self.right=None",
                    "def avl_insert(node, v):",
                    "    if not node: return AVLNode(v)",
                    "    if v < node.val: node.left = avl_insert(node.left,v)",
                    "    else: node.right = avl_insert(node.right,v)",
                    "    node.h = 1 + max(h(node.left), h(node.right))",
                    "    bal = h(node.left) - h(node.right)",
                    "    if bal > 1 and v < node.left.val:",
                    "        return rotate_right(node)",
                    "    return node"
                ),
                rust: rs!(
                    "struct AVLNode { val: i32, h: i32,",
                    "    left: Option<Box<AVLNode>>,",
                    "    right: Option<Box<AVLNode>> }",
                    "fn avl_insert(n: Option<Box<AVLNode>>, v: i32)",
                    "    -> Option<Box<AVLNode>> {",
                    "    // BST insert + height update +",
                    "    // rotation check (LL,LH,RL,LL)",
                    "    n // stub",
                    "}"
                ),
            },
            line_map: lm!("avl-insert"=>[5],"insert-val"=>[6],"compare"=>[7],"rotate"=>[11],"done"=>[16]),
        },
        AlgorithmEntry {
            algorithm: "bst-preorder".into(),
            meta: meta_simple!("bst-preorder","二叉树前序遍历","Pre-order Traversal",Tree,"根 → 左 → 右 的顺序访问每个节点。",time="O(n)",space="O(h)",use=["复制树结构","前缀表达式"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "void preorder(TreeNode* root) {",
                    "  if (!root) return;",
                    "  visit(root->val);",
                    "  preorder(root->left);",
                    "  preorder(root->right);",
                    "}"
                ),
                python: py!(
                    "def preorder(root):",
                    "    if not root: return",
                    "    visit(root.val)",
                    "    preorder(root.left)",
                    "    preorder(root.right)"
                ),
                rust: rs!(
                    "fn preorder(root: &Option<Box<Node>>) {",
                    "    if let Some(n) = root {",
                    "        visit(n.val);",
                    "        preorder(&n.left);",
                    "        preorder(&n.right);",
                    "    }",
                    "}"
                ),
            },
            line_map: lm!("preorder"=>[1],"visit-pre"=>[3],"done"=>[6]),
        },
        AlgorithmEntry {
            algorithm: "bst-inorder".into(),
            meta: meta_simple!("bst-inorder","二叉树中序遍历","In-order Traversal",Tree,"左 → 根 → 右 的顺序，输出递增序列。",time="O(n)",space="O(h)",use=["BST 有序输出"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "void inorder(TreeNode* root) {",
                    "  if (!root) return;",
                    "  inorder(root->left);",
                    "  visit(root->val);",
                    "  inorder(root->right);",
                    "}"
                ),
                python: py!(
                    "def inorder(root):",
                    "    if not root: return",
                    "    inorder(root.left)",
                    "    visit(root.val)",
                    "    inorder(root.right)"
                ),
                rust: rs!(
                    "fn inorder(root: &Option<Box<Node>>) {",
                    "    if let Some(n) = root {",
                    "        inorder(&n.left);",
                    "        visit(n.val);",
                    "        inorder(&n.right);",
                    "    }",
                    "}"
                ),
            },
            line_map: lm!("inorder"=>[1],"visit-in"=>[4],"done"=>[6]),
        },
        AlgorithmEntry {
            algorithm: "bst-postorder".into(),
            meta: meta_simple!("bst-postorder","二叉树后序遍历","Post-order Traversal",Tree,"左 → 右 → 根 的顺序，用于删除和表达式计算。",time="O(n)",space="O(h)",use=["释放树","后缀表达式"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "void postorder(TreeNode* root) {",
                    "  if (!root) return;",
                    "  postorder(root->left);",
                    "  postorder(root->right);",
                    "  visit(root->val);",
                    "}"
                ),
                python: py!(
                    "def postorder(root):",
                    "    if not root: return",
                    "    postorder(root.left)",
                    "    postorder(root.right)",
                    "    visit(root.val)"
                ),
                rust: rs!(
                    "fn postorder(root: &Option<Box<Node>>) {",
                    "    if let Some(n) = root {",
                    "        postorder(&n.left);",
                    "        postorder(&n.right);",
                    "        visit(n.val);",
                    "    }",
                    "}"
                ),
            },
            line_map: lm!("postorder"=>[1],"visit-post"=>[5],"done"=>[6]),
        },
        AlgorithmEntry {
            algorithm: "bst-levelorder".into(),
            meta: meta_simple!("bst-levelorder","二叉树层序遍历","Level-order Traversal",Tree,"逐层访问，使用队列实现（BFS）。",time="O(n)",space="O(w)",use=["广度优先处理"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "void levelorder(TreeNode* root) {",
                    "  if (!root) return;",
                    "  queue<TreeNode*> q; q.push(root);",
                    "  while (!q.empty()) {",
                    "    TreeNode* n = q.front(); q.pop();",
                    "    visit(n->val);",
                    "    if (n->left) q.push(n->left);",
                    "    if (n->right) q.push(n->right);",
                    "  }",
                    "}"
                ),
                python: py!(
                    "def levelorder(root):",
                    "    if not root: return",
                    "    from collections import deque",
                    "    q = deque([root])",
                    "    while q:",
                    "        n = q.popleft()",
                    "        visit(n.val)",
                    "        if n.left: q.append(n.left)",
                    "        if n.right: q.append(n.right)"
                ),
                rust: rs!(
                    "fn levelorder(root: &Option<Box<Node>>) {",
                    "    let mut q = VecDeque::new();",
                    "    if let Some(r) = root { q.push_back(r); }",
                    "    while let Some(n) = q.pop_front() {",
                    "        visit(n.val);",
                    "        if let Some(l) = &n.left { q.push_back(l); }",
                    "        if let Some(r) = &n.right { q.push_back(r); }",
                    "    }",
                    "}"
                ),
            },
            line_map: lm!("levelorder"=>[1],"visit"=>[6],"done"=>[9]),
        },
    ]
}
