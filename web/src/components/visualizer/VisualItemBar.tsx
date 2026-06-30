import { motion } from 'framer-motion';
import type { VisualItem, TraceStep } from '../../types';

interface Props {
  step: TraceStep | null;
  maxValue: number;
  item: VisualItem;
  index: number;
  total: number;
}

function getItemState(id: number, step: TraceStep | null) {
  if (!step) return { isSorted: false, isComparing: false, isSwapping: false, isActive: false, isPivot: false };
  return {
    isSorted: step.sorted.includes(id),
    isComparing: step.comparing.includes(id),
    isSwapping: step.swapping.includes(id),
    isActive: step.active.includes(id),
    isPivot: step.pivot_id === id,
    isMin: step.min_id === id,
    isBoundary: step.boundary_id === id,
  };
}

function barColor(state: ReturnType<typeof getItemState>) {
  if (state.isSorted) return '#22c55e';
  if (state.isSwapping) return '#ef4444';
  if (state.isComparing) return '#f59e0b';
  if (state.isPivot) return '#a855f7';
  if (state.isMin || state.isActive) return '#38bdf8';
  return '#64748b';
}

export default function VisualItemBar({ item, step, maxValue, index, total }: Props) {
  const state = getItemState(item.id, step);
  const heightPercent = maxValue > 0 ? (item.value / maxValue) * 100 : 0;
  const widthPercent = 100 / total;

  return (
    <motion.div
      layout
      layoutId={`bar-${item.id}`}
      initial={false}
      animate={{
        y: state.isComparing ? -12 : 0,
        scale: state.isSwapping ? 1.08 : state.isPivot ? 1.05 : 1,
      }}
      transition={{
        layout: { type: 'spring', stiffness: 320, damping: 28 },
        y: { duration: 0.18 },
        scale: { duration: 0.15 },
      }}
      className={`absolute bottom-0 flex flex-col items-center justify-end rounded-t-md bar-shadow ${
        state.isPivot ? 'pivot-ring' : ''
      }`}
      style={{
        left: `${index * widthPercent}%`,
        width: `calc(${widthPercent}% - 6px)`,
        marginLeft: '3px',
        marginRight: '3px',
        height: `${Math.max(heightPercent, 4)}%`,
        backgroundColor: barColor(state),
      }}
    >
      <span className="mb-1 text-[10px] font-semibold text-white/90 drop-shadow">
        {item.value}
      </span>
    </motion.div>
  );
}

export { getItemState };
