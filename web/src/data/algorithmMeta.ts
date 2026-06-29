import type { TraceStep } from '../types';

export type AlgorithmKey =
  | 'selection-sort'
  | 'bubble-sort'
  | 'insertion-sort'
  | 'quick-sort'
  | 'merge-sort';

export interface AlgorithmMeta {
  key: AlgorithmKey;
  name: string;
  nameEn: string;
  description: string;
  complexity: {
    time: string;
    best: string;
    average: string;
    worst: string;
    space: string;
  };
  stable: string;
  useCases: string[];
  category: 'sorting';
}

export const algorithmMetaMap: Record<AlgorithmKey, AlgorithmMeta> = {
  'selection-sort': {
    key: 'selection-sort',
    name: '选择排序',
    nameEn: 'Selection Sort',
    description:
      '每一轮从未排序区间选出最小元素，放到已排序区间的末尾。实现简单，但时间复杂度较高。',
    complexity: {
      time: 'O(n²)',
      best: 'O(n²)',
      average: 'O(n²)',
      worst: 'O(n²)',
      space: 'O(1)',
    },
    stable: '不稳定',
    useCases: ['数据量较小', '内存受限', '对稳定性无要求'],
    category: 'sorting',
  },
  'bubble-sort': {
    key: 'bubble-sort',
    name: '冒泡排序',
    nameEn: 'Bubble Sort',
    description:
      '重复走访未排序序列，比较相邻元素并交换，使较大元素逐渐"冒泡"到末尾。',
    complexity: {
      time: 'O(n²)',
      best: 'O(n)',
      average: 'O(n²)',
      worst: 'O(n²)',
      space: 'O(1)',
    },
    stable: '稳定',
    useCases: ['教学演示', '数据基本有序', '小规模数据'],
    category: 'sorting',
  },
  'insertion-sort': {
    key: 'insertion-sort',
    name: '插入排序',
    nameEn: 'Insertion Sort',
    description:
      '将未排序元素逐个插入到已排序序列的合适位置，类似整理扑克牌。',
    complexity: {
      time: 'O(n²)',
      best: 'O(n)',
      average: 'O(n²)',
      worst: 'O(n²)',
      space: 'O(1)',
    },
    stable: '稳定',
    useCases: ['小规模数据', '部分有序', '在线排序'],
    category: 'sorting',
  },
  'quick-sort': {
    key: 'quick-sort',
    name: '快速排序',
    nameEn: 'Quick Sort',
    description:
      '选择枢轴，将数组划分为左右两部分，再递归排序。平均性能优异。',
    complexity: {
      time: 'O(n log n)',
      best: 'O(n log n)',
      average: 'O(n log n)',
      worst: 'O(n²)',
      space: 'O(log n)',
    },
    stable: '不稳定',
    useCases: ['大规模数据', '通用排序', '内存允许递归'],
    category: 'sorting',
  },
  'merge-sort': {
    key: 'merge-sort',
    name: '归并排序',
    nameEn: 'Merge Sort',
    description:
      '采用分治法将数组拆分为小片段排序，再合并为完整有序数组。稳定且可预测。',
    complexity: {
      time: 'O(n log n)',
      best: 'O(n log n)',
      average: 'O(n log n)',
      worst: 'O(n log n)',
      space: 'O(n)',
    },
    stable: '稳定',
    useCases: ['链表排序', '外部排序', '需要稳定排序'],
    category: 'sorting',
  },
};

export const algorithmKeys: AlgorithmKey[] = [
  'selection-sort',
  'bubble-sort',
  'insertion-sort',
  'quick-sort',
  'merge-sort',
];

export function getAlgorithmName(key: AlgorithmKey): string {
  return algorithmMetaMap[key].name;
}

export function getStepDescription(step: TraceStep): string {
  if (step.note) return step.note;
  switch (step.step_type) {
    case 'Start':
      return '开始排序';
    case 'Compare':
      return '正在比较元素';
    case 'Swap':
      return '交换元素位置';
    case 'Overwrite':
      return '写入元素';
    case 'Select':
      return '选择关键元素';
    case 'Pivot':
      return '确定枢轴';
    case 'Partition':
      return '划分子区间';
    case 'Merge':
      return '合并区间';
    case 'Sorted':
      return '元素已排序';
    case 'Done':
      return '排序完成';
    default:
      return '';
  }
}
