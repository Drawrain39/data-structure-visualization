import { motion } from 'framer-motion';
import type { TraceStep } from '../../../types';

interface Props {
  step: TraceStep | null;
}

export default function LinkedListRenderer({ step }: Props) {
  const items = step?.items ?? [];

  if (items.length === 0) {
    return (
      <div className="flex h-full items-center justify-center text-slate-500">
        链表为空
      </div>
    );
  }

  return (
    <div className="flex h-full items-center justify-center gap-0 overflow-x-auto px-4">
      {items.map((item) => {
        const isActive = step?.active.includes(item.id);
        const isVisited = step?.sorted.includes(item.id);

        return (
          <motion.div key={item.id} className="flex items-center" layout>
            {/* Node box */}
            <motion.div
              layout
              animate={{
                scale: isActive ? 1.12 : 1,
                backgroundColor: isActive ? 'rgba(56,189,248,0.25)' : isVisited ? 'rgba(34,197,94,0.2)' : 'rgba(30,41,59,0.9)',
                borderColor: isActive ? '#38bdf8' : isVisited ? '#22c55e' : '#475569',
              }}
              className="flex h-14 w-14 flex-col items-center justify-center rounded-xl border-2 shadow-lg"
            >
              <span className="text-sm font-bold text-slate-100">{item.value}</span>
              <span className="text-[10px] text-slate-400">#{item.id}</span>
            </motion.div>

            {/* Arrow to next */}
            {item.id < items.length - 1 && (
              <div className="flex items-center px-1">
                <div className="h-0.5 w-6 bg-slate-600" />
                <div className="h-0 w-0 border-[5px] border-l-slate-600 border-r-transparent border-t-transparent border-b-transparent" />
              </div>
            )}
          </motion.div>
        );
      })}
    </div>
  );
}
