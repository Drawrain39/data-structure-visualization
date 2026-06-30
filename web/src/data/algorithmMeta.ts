import type { TraceStep } from '../types';

export type AlgorithmCategory =
  | 'sorting'
  | 'searching'
  | 'linear'
  | 'stack-queue'
  | 'recursive'
  | 'tree'
  | 'graph';

export type AlgorithmKey =
  // Sorting
  | 'selection-sort'
  | 'bubble-sort'
  | 'insertion-sort'
  | 'merge-sort'
  | 'quick-sort'
  | 'heap-sort'
  // Searching
  | 'linear-search'
  | 'binary-search'
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
  // Graph
  | 'bfs'
  | 'dfs';

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
    default:
      return '';
  }
}
