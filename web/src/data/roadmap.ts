export interface RoadmapPhase {
  title: string;
  items: string[];
}

export const roadmap: RoadmapPhase[] = [
  {
    title: '第一阶段：基础排序',
    items: ['冒泡排序', '选择排序', '插入排序'],
  },
  {
    title: '第二阶段：高效排序',
    items: ['快速排序', '归并排序', '堆排序'],
  },
  {
    title: '第三阶段：线性排序',
    items: ['计数排序', '基数排序', '桶排序'],
  },
  {
    title: '第四阶段：基础数据结构',
    items: ['数组 / 链表', '栈 / 队列', '哈希表'],
  },
  {
    title: '第五阶段：树与图',
    items: ['二叉搜索树', '堆', '图遍历（BFS / DFS）'],
  },
];
