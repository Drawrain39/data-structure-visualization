export interface RoadmapPhase {
  title: string;
  items: { name: string; status: 'done' | 'doing' | 'planned' }[];
}

export const roadmap: RoadmapPhase[] = [
  {
    title: '线性表',
    items: [
      { name: '数组', status: 'done' },
      { name: '链表', status: 'done' },
      { name: '栈', status: 'done' },
      { name: '队列', status: 'done' },
    ],
  },
  {
    title: '排序',
    items: [
      { name: '选择 / 冒泡 / 插入', status: 'done' },
      { name: '归并 / 快速 / 堆', status: 'done' },
    ],
  },
  {
    title: '查找',
    items: [
      { name: '顺序查找', status: 'done' },
      { name: '二分查找', status: 'done' },
      { name: '哈希表', status: 'planned' },
    ],
  },
  {
    title: '递归',
    items: [
      { name: '阶乘 / 斐波那契', status: 'done' },
      { name: '汉诺塔', status: 'done' },
    ],
  },
  {
    title: '树与图',
    items: [
      { name: '二叉搜索树', status: 'done' },
      { name: '堆', status: 'done' },
      { name: 'BFS / DFS', status: 'done' },
    ],
  },
];
