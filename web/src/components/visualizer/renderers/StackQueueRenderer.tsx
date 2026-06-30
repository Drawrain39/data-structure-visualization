import { motion } from 'framer-motion';
import type { TraceStep, VisualItem } from '../../../types';

interface Props {
  step: TraceStep | null;
  variant: 'stack' | 'queue';
}

function orderItems(items: VisualItem[], active: number[]): VisualItem[] {
  if (active.length === 0) return [];
  // active is already in push/enqueue order
  return active.map((id) => items.find((it) => it.id === id)!).filter(Boolean);
}

export default function StackQueueRenderer({ step, variant }: Props) {
  const allItems = step?.items ?? [];
  const active = step?.active ?? [];
  const visible = orderItems(allItems, active);

  if (visible.length === 0 && allItems.length === 0) {
    return (
      <div className="flex h-full items-center justify-center text-slate-500">
        {variant === 'stack' ? '栈为空' : '队列为空'}
      </div>
    );
  }

  const isStack = variant === 'stack';
  const highlighted = new Set(step?.comparing ?? []);
  highlighted.add(step?.pivot_id ?? -1);

  const containerClass = isStack
    ? 'flex-col items-center justify-end gap-2'
    : 'flex-row items-center justify-center gap-2';

  return (
    <div className="flex h-full items-center justify-center">
      <div className={`flex ${containerClass} max-h-full overflow-auto p-4`}>
        {visible.map((item, idx) => {
          const isActive = highlighted.has(item.id);
          const sideLabel = isStack
            ? (idx === visible.length - 1 ? 'Top' : '')
            : (idx === 0 ? 'Front' : idx === visible.length - 1 ? 'Rear' : '');

          return (
            <motion.div
              key={`${item.id}-${idx}`}
              layout
              initial={{ opacity: 0, scale: 0.8 }}
              animate={{
                opacity: 1,
                scale: isActive ? 1.08 : 1,
                backgroundColor: isActive ? 'rgba(56,189,248,0.25)' : 'rgba(30,41,59,0.9)',
                borderColor: isActive ? '#38bdf8' : '#475569',
              }}
              exit={{ opacity: 0, scale: 0.8 }}
              className={`flex items-center justify-between rounded-lg border-2 px-4 py-2.5 shadow-lg ${isStack ? 'w-36' : ''}`}
            >
              <span className="text-sm font-bold text-slate-100">{item.value}</span>
              {sideLabel && (
                <span className={`ml-2 rounded px-1.5 py-0.5 text-[10px] font-medium ${
                  sideLabel === 'Top' || sideLabel === 'Front'
                    ? 'bg-accent/20 text-accent'
                    : 'bg-fuchsia-500/20 text-fuchsia-400'
                }`}>
                  {sideLabel}
                </span>
              )}
            </motion.div>
          );
        })}
      </div>
    </div>
  );
}
