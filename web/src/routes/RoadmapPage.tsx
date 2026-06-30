import { roadmap } from '../data/roadmap';

const statusClass: Record<'done' | 'doing' | 'planned', string> = {
  done: 'bg-green-500/20 text-green-400',
  doing: 'bg-accent/20 text-accent',
  planned: 'bg-slate-800 text-slate-500',
};

const statusText: Record<'done' | 'doing' | 'planned', string> = {
  done: '已完成',
  doing: '进行中',
  planned: '规划中',
};

export default function RoadmapPage() {
  return (
    <div className="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
      <div className="mb-10 text-center">
        <h1 className="text-2xl font-bold text-slate-50">学习路线</h1>
        <p className="mt-2 text-slate-400">从线性表到树与图，循序渐进掌握数据结构与算法</p>
      </div>

      <div className="relative space-y-8 pl-6 before:absolute before:left-2 before:top-2 before:h-full before:w-0.5 before:bg-slate-800">
        {roadmap.map((phase, idx) => (
          <div key={phase.title} className="relative">
            <span className="absolute -left-6 top-1 flex h-4 w-4 items-center justify-center rounded-full bg-slate-800 ring-4 ring-slate-950">
              <span className="h-2 w-2 rounded-full bg-accent" />
            </span>
            <div className="rounded-2xl border border-slate-800 bg-slate-900/60 p-5 shadow-xl">
              <div className="mb-3 flex items-center gap-2">
                <span className="rounded-md bg-accent/10 px-2 py-0.5 text-xs font-medium text-accent">
                  阶段 {idx + 1}
                </span>
                <h3 className="font-bold text-slate-50">{phase.title}</h3>
              </div>
              <ul className="grid grid-cols-1 gap-2 sm:grid-cols-2">
                {phase.items.map((item) => (
                  <li
                    key={item.name}
                    className="flex items-center justify-between rounded-lg bg-slate-950/50 px-3 py-2 text-sm text-slate-300"
                  >
                    <span>{item.name}</span>
                    <span className={`rounded px-1.5 py-0.5 text-xs ${statusClass[item.status]}`}>
                      {statusText[item.status]}
                    </span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
