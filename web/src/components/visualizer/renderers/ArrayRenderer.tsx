import { motion } from 'framer-motion';
import VisualItemBar from '../VisualItemBar';
import type { TraceStep } from '../../../types';

interface Props {
  step: TraceStep | null;
}

export default function ArrayRenderer({ step }: Props) {
  const items = step?.items ?? [];
  const total = items.length;
  const maxValue = total > 0 ? Math.max(...items.map((it) => it.value)) : 0;

  if (total === 0) {
    return (
      <div className="flex h-full items-center justify-center text-slate-500">
        暂无数据
      </div>
    );
  }

  return (
    <motion.div className="absolute inset-x-0 bottom-4 top-4" layout>
      {items.map((item, index) => (
        <VisualItemBar
          key={item.id}
          item={item}
          index={index}
          total={total}
          step={step}
          maxValue={maxValue}
        />
      ))}
    </motion.div>
  );
}
