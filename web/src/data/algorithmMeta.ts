import type { TraceStep } from '../types';

export type AlgorithmCategory =
  | 'sorting'
  | 'searching'
  | 'linear'
  | 'stack-queue'
  | 'recursive'
  | 'tree'
  | 'graph'
  | 'dp';

export type AlgorithmKey =
  // Sorting
  | 'selection-sort'
  | 'bubble-sort'
  | 'insertion-sort'
  | 'merge-sort'
  | 'quick-sort'
  | 'heap-sort'
  | 'shell-sort'
  | 'counting-sort'
  | 'bucket-sort'
  | 'radix-sort'
  // Searching
  | 'linear-search'
  | 'binary-search'
  | 'interpolation-search'
  | 'hash-search'
  // Linear
  | 'array-insert'
  | 'array-delete'
  | 'linked-list-traverse'
  // Stack/Queue
  | 'stack-push-pop'
  | 'queue-enqueue-dequeue'
  // Recursive
  | 'factorial'
  | 'fibonacci'
  | 'tower-of-hanoi'
  // Tree
  | 'bst-insert'
  | 'bst-search'
  | 'heap-insert'
  | 'avl-insert'
  | 'bst-preorder'
  | 'bst-inorder'
  | 'bst-postorder'
  | 'bst-levelorder'
  // Graph
  | 'bfs'
  | 'dfs'
  | 'dijkstra'
  | 'topological-sort'
  | 'kruskal'
  | 'prim'
  // DP
  | 'fibonacci-dp'
  | 'knapsack'
  | 'lcs'
  | 'lis';

export interface AlgorithmMeta {
  key: AlgorithmKey;
  name: string;
  nameEn: string;
  category: AlgorithmCategory;
  description: string;
  complexity: {
    time: string;
    best?: string;
    average?: string;
    worst?: string;
    space: string;
  };
  stable?: string;
  useCases: string[];
}

export const categoryLabels: Record<AlgorithmCategory, string> = {
  sorting: '排序',
  searching: '查找',
  linear: '线性表',
  'stack-queue': '栈与队列',
  recursive: '递归',
  tree: '树',
  graph: '图',
  dp: '动态规划',
};

export const algorithmMetaMap: Record<AlgorithmKey, AlgorithmMeta> = {
  // Sorting
  'selection-sort': {
    key: 'selection-sort',
    name: '选择排序',
    nameEn: 'Selection Sort',
    category: 'sorting',
    description: '每轮从未排序区间选出最小元素放到已排序区间末尾。',
    complexity: { time: 'O(n²)', best: 'O(n²)', average: 'O(n²)', worst: 'O(n²)', space: 'O(1)' },
    stable: '不稳定',
    useCases: ['数据量小', '内存受限'],
  },
  'bubble-sort': {
    key: 'bubble-sort',
    name: '冒泡排序',
    nameEn: 'Bubble Sort',
    category: 'sorting',
    description: '相邻元素比较并交换，使较大元素逐渐冒泡到末尾。',
    complexity: { time: 'O(n²)', best: 'O(n)', average: 'O(n²)', worst: 'O(n²)', space: 'O(1)' },
    stable: '稳定',
    useCases: ['教学演示', '基本有序'],
  },
  'insertion-sort': {
    key: 'insertion-sort',
    name: '插入排序',
    nameEn: 'Insertion Sort',
    category: 'sorting',
    description: '将元素逐个插入到已排序序列的合适位置。',
    complexity: { time: 'O(n²)', best: 'O(n)', average: 'O(n²)', worst: 'O(n²)', space: 'O(1)' },
    stable: '稳定',
    useCases: ['小规模', '部分有序'],
  },
  'merge-sort': {
    key: 'merge-sort',
    name: '归并排序',
    nameEn: 'Merge Sort',
    category: 'sorting',
    description: '分治法拆分排序后合并，稳定且可预测。',
    complexity: { time: 'O(n log n)', best: 'O(n log n)', average: 'O(n log n)', worst: 'O(n log n)', space: 'O(n)' },
    stable: '稳定',
    useCases: ['链表排序', '外部排序', '需稳定排序'],
  },
  'quick-sort': {
    key: 'quick-sort',
    name: '快速排序',
    nameEn: 'Quick Sort',
    category: 'sorting',
    description: '选择枢轴划分子区间并递归排序。',
    complexity: { time: 'O(n log n)', best: 'O(n log n)', average: 'O(n log n)', worst: 'O(n²)', space: 'O(log n)' },
    stable: '不稳定',
    useCases: ['大规模', '通用排序'],
  },
  'heap-sort': {
    key: 'heap-sort',
    name: '堆排序',
    nameEn: 'Heap Sort',
    category: 'sorting',
    description: '构建最大堆后不断取出堆顶完成排序。',
    complexity: { time: 'O(n log n)', best: 'O(n log n)', average: 'O(n log n)', worst: 'O(n log n)', space: 'O(1)' },
    stable: '不稳定',
    useCases: ['Top K', '内存受限'],
  },
  'shell-sort': {
    key: 'shell-sort',
    name: '希尔排序',
    nameEn: 'Shell Sort',
    category: 'sorting',
    description: '改进的插入排序，通过递减增量分组排序逐步逼近有序。',
    complexity: { time: 'O(n log² n)', best: 'O(n log n)', average: 'O(n log² n)', worst: 'O(n²)', space: 'O(1)' },
    stable: '不稳定',
    useCases: ['中等规模', '插入排序的改进替代'],
  },
  // Searching
  'linear-search': {
    key: 'linear-search',
    name: '顺序查找',
    nameEn: 'Linear Search',
    category: 'searching',
    description: '从头到尾逐个比较查找目标值。',
    complexity: { time: 'O(n)', best: 'O(1)', average: 'O(n)', worst: 'O(n)', space: 'O(1)' },
    useCases: ['无序数据', '小数据集'],
  },
  'binary-search': {
    key: 'binary-search',
    name: '二分查找',
    nameEn: 'Binary Search',
    category: 'searching',
    description: '在有序数组中通过折半快速定位目标。',
    complexity: { time: 'O(log n)', best: 'O(1)', average: 'O(log n)', worst: 'O(log n)', space: 'O(1)' },
    useCases: ['有序数组', '频繁查询'],
  },
  // Linear
  'array-insert': {
    key: 'array-insert',
    name: '数组插入',
    nameEn: 'Array Insert',
    category: 'linear',
    description: '在数组末尾追加新元素。',
    complexity: { time: 'O(1)', space: 'O(1)' },
    useCases: ['动态扩容数组'],
  },
  'array-delete': {
    key: 'array-delete',
    name: '数组删除',
    nameEn: 'Array Delete',
    category: 'linear',
    description: '删除数组指定位置元素。',
    complexity: { time: 'O(n)', space: 'O(1)' },
    useCases: ['顺序表维护'],
  },
  'linked-list-traverse': {
    key: 'linked-list-traverse',
    name: '链表遍历',
    nameEn: 'Linked List Traverse',
    category: 'linear',
    description: '从头节点依次访问每个节点。',
    complexity: { time: 'O(n)', space: 'O(1)' },
    useCases: ['链表查询'],
  },
  // Stack/Queue
  'stack-push-pop': {
    key: 'stack-push-pop',
    name: '栈操作',
    nameEn: 'Stack Push/Pop',
    category: 'stack-queue',
    description: '先入后出：依次入栈再出栈。',
    complexity: { time: 'O(1)', space: 'O(n)' },
    useCases: ['表达式求值', '回溯'],
  },
  'queue-enqueue-dequeue': {
    key: 'queue-enqueue-dequeue',
    name: '队列操作',
    nameEn: 'Queue Enqueue/Dequeue',
    category: 'stack-queue',
    description: '先入先出：依次入队再出队。',
    complexity: { time: 'O(1)', space: 'O(n)' },
    useCases: ['BFS', '任务调度'],
  },
  // Recursive
  'factorial': {
    key: 'factorial',
    name: '阶乘',
    nameEn: 'Factorial',
    category: 'recursive',
    description: 'n! = n × (n-1) × ... × 1。',
    complexity: { time: 'O(n)', space: 'O(1)' },
    useCases: ['递归入门'],
  },
  'fibonacci': {
    key: 'fibonacci',
    name: '斐波那契',
    nameEn: 'Fibonacci',
    category: 'recursive',
    description: 'F(n) = F(n-1) + F(n-2)。',
    complexity: { time: 'O(n)', space: 'O(n)' },
    useCases: ['递归与动态规划'],
  },
  'tower-of-hanoi': {
    key: 'tower-of-hanoi',
    name: '汉诺塔',
    nameEn: 'Tower of Hanoi',
    category: 'recursive',
    description: '经典递归问题，将盘子从 A 柱移到 C 柱。',
    complexity: { time: 'O(2ⁿ)', space: 'O(n)' },
    useCases: ['递归思维训练'],
  },
  // Tree
  'bst-insert': {
    key: 'bst-insert',
    name: '二叉搜索树插入',
    nameEn: 'BST Insert',
    category: 'tree',
    description: '按大小关系插入到左子树或右子树。',
    complexity: { time: 'O(h)', space: 'O(1)' },
    useCases: ['动态有序集合'],
  },
  'bst-search': {
    key: 'bst-search',
    name: '二叉搜索树查找',
    nameEn: 'BST Search',
    category: 'tree',
    description: '在二叉搜索树中按大小关系查找目标。',
    complexity: { time: 'O(h)', space: 'O(1)' },
    useCases: ['有序数据查找'],
  },
  'heap-insert': {
    key: 'heap-insert',
    name: '堆插入',
    nameEn: 'Heap Insert',
    category: 'tree',
    description: '将元素插入堆末尾并上浮调整。',
    complexity: { time: 'O(log n)', space: 'O(1)' },
    useCases: ['优先队列'],
  },
  // Graph
  'bfs': {
    key: 'bfs',
    name: '广度优先搜索',
    nameEn: 'BFS',
    category: 'graph',
    description: '从起点出发逐层访问相邻节点。',
    complexity: { time: 'O(V+E)', space: 'O(V)' },
    useCases: ['最短路径', '连通性'],
  },
  'dfs': {
    key: 'dfs',
    name: '深度优先搜索',
    nameEn: 'DFS',
    category: 'graph',
    description: '从起点出发沿一条路径深入访问。',
    complexity: { time: 'O(V+E)', space: 'O(V)' },
    useCases: ['拓扑排序', '连通分量'],
  },
  'counting-sort': {
    key: 'counting-sort', name: '计数排序', nameEn: 'Counting Sort', category: 'sorting',
    description: '统计每个值出现的次数，通过前缀和确定位置。',
    complexity: { time: 'O(n+k)', space: 'O(k)' }, stable: '稳定',
    useCases: ['整数排序', '值域较小'],
  },
  'bucket-sort': {
    key: 'bucket-sort', name: '桶排序', nameEn: 'Bucket Sort', category: 'sorting',
    description: '将元素分配到多个桶中，桶内排序后合并。',
    complexity: { time: 'O(n+k)', space: 'O(n+k)' }, stable: '稳定',
    useCases: ['均匀分布数据', '外部排序'],
  },
  'radix-sort': {
    key: 'radix-sort', name: '基数排序', nameEn: 'Radix Sort', category: 'sorting',
    description: '按位排序，从低位到高位依次进行稳定计数排序。',
    complexity: { time: 'O(d·(n+k))', space: 'O(n+k)' }, stable: '稳定',
    useCases: ['整数排序', '字符串排序'],
  },
  'interpolation-search': {
    key: 'interpolation-search', name: '插值查找', nameEn: 'Interpolation Search', category: 'searching',
    description: '根据目标值估算其在有序数组中的位置。',
    complexity: { time: 'O(log log n)', best: 'O(1)', worst: 'O(n)', space: 'O(1)' },
    useCases: ['均匀分布有序数据'],
  },
  'hash-search': {
    key: 'hash-search', name: '哈希查找', nameEn: 'Hash Search', category: 'searching',
    description: '通过哈希函数直接映射到存储位置，O(1) 查询。',
    complexity: { time: 'O(1)', worst: 'O(n)', space: 'O(n)' },
    useCases: ['大量数据快速查找', '去重'],
  },
  'avl-insert': {
    key: 'avl-insert', name: 'AVL 插入', nameEn: 'AVL Insert', category: 'tree',
    description: '插入后通过旋转保持平衡，确保高度差不超过 1。',
    complexity: { time: 'O(log n)', space: 'O(1)' },
    useCases: ['频繁增删的有序集合'],
  },
  'bst-preorder': {
    key: 'bst-preorder', name: '二叉树前序遍历', nameEn: 'Pre-order Traversal', category: 'tree',
    description: '根 → 左 → 右 的顺序访问每个节点。',
    complexity: { time: 'O(n)', space: 'O(h)' },
    useCases: ['复制树结构', '前缀表达式'],
  },
  'bst-inorder': {
    key: 'bst-inorder', name: '二叉树中序遍历', nameEn: 'In-order Traversal', category: 'tree',
    description: '左 → 根 → 右 的顺序，输出递增序列。',
    complexity: { time: 'O(n)', space: 'O(h)' },
    useCases: ['BST 有序输出'],
  },
  'bst-postorder': {
    key: 'bst-postorder', name: '二叉树后序遍历', nameEn: 'Post-order Traversal', category: 'tree',
    description: '左 → 右 → 根 的顺序，用于删除和表达式计算。',
    complexity: { time: 'O(n)', space: 'O(h)' },
    useCases: ['释放树', '后缀表达式'],
  },
  'bst-levelorder': {
    key: 'bst-levelorder', name: '二叉树层序遍历', nameEn: 'Level-order Traversal', category: 'tree',
    description: '逐层访问，使用队列实现（BFS）。',
    complexity: { time: 'O(n)', space: 'O(w)' },
    useCases: ['广度优先处理'],
  },
  'dijkstra': {
    key: 'dijkstra', name: 'Dijkstra 最短路径', nameEn: 'Dijkstra', category: 'graph',
    description: '贪心扩展，逐步确定每个节点的最短距离。',
    complexity: { time: 'O((V+E)logV)', space: 'O(V)' },
    useCases: ['路由算法', '地图导航'],
  },
  'topological-sort': {
    key: 'topological-sort', name: '拓扑排序', nameEn: 'Topological Sort', category: 'graph',
    description: 'Kahn 算法，按入度逐步输出 DAG 的有序序列。',
    complexity: { time: 'O(V+E)', space: 'O(V)' },
    useCases: ['任务调度', '依赖解析'],
  },
  'kruskal': {
    key: 'kruskal', name: 'Kruskal 最小生成树', nameEn: 'Kruskal MST', category: 'graph',
    description: '按边权升序，用并查集逐步加入不构成环的边。',
    complexity: { time: 'O(E log E)', space: 'O(V)' },
    useCases: ['网络布线', '聚类'],
  },
  'prim': {
    key: 'prim', name: 'Prim 最小生成树', nameEn: 'Prim MST', category: 'graph',
    description: '从起点出发，每次都选择连接已选集合的最小边。',
    complexity: { time: 'O((V+E)logV)', space: 'O(V)' },
    useCases: ['网络布线', '连通图'],
  },
  'fibonacci-dp': {
    key: 'fibonacci-dp', name: '斐波那契 DP', nameEn: 'Fibonacci DP', category: 'dp',
    description: '自底向上递推，避免递归重复计算。',
    complexity: { time: 'O(n)', space: 'O(n)' },
    useCases: ['DP 入门'],
  },
  'knapsack': {
    key: 'knapsack', name: '0/1 背包', nameEn: 'Knapsack', category: 'dp',
    description: '选择物品使总价值最大且不超容量。',
    complexity: { time: 'O(n·W)', space: 'O(n·W)' },
    useCases: ['资源分配', '投资组合'],
  },
  'lcs': {
    key: 'lcs', name: '最长公共子序列', nameEn: 'LCS', category: 'dp',
    description: '求两个序列的最长公共子序列长度。',
    complexity: { time: 'O(m·n)', space: 'O(m·n)' },
    useCases: ['文本对比', '版本控制'],
  },
  'lis': {
    key: 'lis', name: '最长递增子序列', nameEn: 'LIS', category: 'dp',
    description: '求数组中最长严格递增子序列的长度。',
    complexity: { time: 'O(n²)', space: 'O(n)' },
    useCases: ['序列分析', '套娃问题'],
  },
};

export const algorithmKeys: AlgorithmKey[] = Object.keys(
  algorithmMetaMap
) as AlgorithmKey[];

export function getAlgorithmName(key: AlgorithmKey): string {
  return algorithmMetaMap[key].name;
}

export function getAlgorithmsByCategory(category: AlgorithmCategory): AlgorithmKey[] {
  return algorithmKeys.filter((k) => algorithmMetaMap[k].category === category);
}

export function getStepDescription(step: TraceStep): string {
  if (step.note) return step.note;
  switch (step.step_type) {
    case 'Start':
      return '开始';
    case 'Compare':
      return '比较';
    case 'Swap':
      return '交换';
    case 'Overwrite':
      return '写入';
    case 'Select':
      return '选中';
    case 'Pivot':
      return '枢轴';
    case 'Partition':
      return '划分';
    case 'Merge':
      return '合并';
    case 'Sorted':
      return '已排序';
    case 'Done':
      return '完成';
    case 'Highlight':
      return '高亮';
    case 'Visit':
      return '访问';
    case 'Push':
      return '入栈';
    case 'Pop':
      return '出栈';
    case 'Enqueue':
      return '入队';
    case 'Dequeue':
      return '出队';
    case 'Relax':
      return '松弛';
    case 'Rotate':
      return '旋转';
    default:
      return '';
  }
}
