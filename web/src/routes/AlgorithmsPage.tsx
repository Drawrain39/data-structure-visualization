import { algorithmKeys, algorithmMetaMap } from '../data/algorithmMeta';
import { Link } from 'react-router-dom';

export default function AlgorithmsPage() {
  return (
    <div className="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-slate-50">算法库</h1>
        <p className="mt-2 text-slate-400">已实现的排序算法</p>
      </div>

      <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
        {algorithmKeys.map((key) => {
          const meta = algorithmMetaMap[key];
          return (
            <Link
              key={key}
              to={`/visualizer?algorithm=${key}`}
              className="group rounded-2xl border border-slate-800 bg-slate-900/60 p-5 shadow-xl transition-colors hover:border-accent/30"
            >
              <div className="mb-3 flex items-center justify-between">
                <h3 className="text-lg font-bold text-slate-50">{meta.name}</h3>
                <span className="rounded-full bg-slate-800 px-2 py-0.5 text-xs text-slate-400">
                  {meta.complexity.time}
                </span>
              </div>
              <p className="mb-4 text-sm text-slate-400">{meta.description}</p>
              <div className="flex flex-wrap gap-2">
                {meta.useCases.slice(0, 2).map((u) => (
                  <span
                    key={u}
                    className="rounded-md bg-slate-800/80 px-2 py-1 text-xs text-slate-300"
                  >
                    {u}
                  </span>
                ))}
              </div>
              <div className="mt-4 text-sm font-medium text-accent opacity-0 transition-opacity group-hover:opacity-100">
                去可视化 →
              </div>
            </Link>
          );
        })}
      </div>
    </div>
  );
}
