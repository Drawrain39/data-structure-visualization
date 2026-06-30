import { motion } from 'framer-motion';
import type { TraceStep } from '../../../types';

interface Props {
  step: TraceStep | null;
}

export default function CallStackRenderer({ step }: Props) {
  if (!step) {
    return (
      <div className="flex h-full items-center justify-center text-slate-500">
        等待执行
      </div>
    );
  }

  const note = step.note;
  const extra = step.extra as Record<string, unknown> | undefined;
  const result = extra?.result as number | undefined;

  if (step.step_type === 'Done') {
    return (
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        className="flex h-full flex-col items-center justify-center gap-3"
      >
        <div className="rounded-2xl border-2 border-green-500/40 bg-green-500/10 px-6 py-4 shadow-lg">
          <span className="text-xs text-green-400">结果</span>
          <div className="mt-1 text-2xl font-bold text-green-300">{result ?? '?'}</div>
        </div>
        <span className="text-sm text-slate-400">{note}</span>
      </motion.div>
    );
  }

  return (
    <div className="flex h-full flex-col items-center justify-center gap-4 p-4">
      {/* Call stack */}
      <div className="flex flex-col-reverse gap-2">
        {(step.active ?? []).map((id, idx) => {
          const item = step.items.find((it) => it.id === id);
          return (
            <motion.div
              key={`${id}-${idx}`}
              layout
              initial={{ opacity: 0, x: -40 }}
              animate={{ opacity: 1, x: 0 }}
              className="rounded-lg border border-slate-700 bg-slate-800/80 px-5 py-2 shadow-lg"
            >
              <span className="text-sm font-medium text-slate-200">
                {item ? `n = ${item.value}` : `frame ${idx}`}
              </span>
            </motion.div>
          );
        })}
      </div>

      {/* Current step info */}
      <motion.div
        layout
        animate={{ opacity: 1 }}
        className="rounded-xl border border-slate-700 bg-slate-900/80 px-4 py-2 shadow-lg"
      >
        <span className="text-sm text-slate-300">{note}</span>
      </motion.div>
    </div>
  );
}
