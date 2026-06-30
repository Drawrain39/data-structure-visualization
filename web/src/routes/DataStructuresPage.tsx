import { Layers } from 'lucide-react';

const structures = [
  { name: '数组', nameEn: 'Array', status: '已实现', category: 'linear' },
  { name: '链表', nameEn: 'Linked List', status: '已实现', category: 'linear' },
  { name: '栈', nameEn: 'Stack', status: '已实现', category: 'stack-queue' },
  { name: '队列', nameEn: 'Queue', status: '已实现', category: 'stack-queue' },
  { name: '哈希表', nameEn: 'Hash Table', status: '规划中', category: 'searching' },
  { name: '二叉搜索树', nameEn: 'BST', status: '已实现', category: 'tree' },
  { name: '堆', nameEn: 'Heap', status: '已实现', category: 'tree' },
  { name: '图', nameEn: 'Graph', status: '已实现', category: 'graph' },
];

export default function DataStructuresPage() {
  return (
    <div className="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-slate-50">数据结构</h1>
        <p className="mt-2 text-slate-400">支持的数据结构与对应操作入口</p>
      </div>

      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {structures.map((s) => (
          <div
            key={s.name}
            className="flex items-center gap-4 rounded-2xl border border-slate-800 bg-slate-900/60 p-5 shadow-lg"
          >
            <div className="rounded-xl bg-accent/10 p-3 text-accent">
              <Layers className="h-6 w-6" />
            </div>
            <div>
              <div className="font-bold text-slate-50">{s.name}</div>
              <div className="text-xs text-slate-500">{s.nameEn}</div>
              <div className="mt-1 text-xs text-slate-400">{s.status}</div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
