import { motion } from 'framer-motion';
import VisualItemBar from './VisualItemBar';
import type { TraceStep } from '../../types';

interface Props {
  step: TraceStep | null;
}

export default function VisualizerStage({ step }: Props) {
  const items = step?.items ?? [];
  const maxValue = items.length > 0 ? Math.max(...items.map((it) => it.value)) : 0;

  return (
    <div className="relative flex h-full min-h-[320px] flex-col rounded-2xl border border-slate-800 bg-slate-900/60 p-4 shadow-2xl">
      <div className="mb-2 flex items-center justify-between text-sm text-slate-400">
        <span>动画舞台</span>
        <span>{items.length} 个元素</span>
      </div>

      <div className="relative flex-1 overflow-hidden rounded-xl bg-slate-950/60">
        {items.length === 0 && (
          <div className="flex h-full items-center justify-center text-slate-500">
            暂无数据
          </div>
        )}

        <motion.div className="absolute inset-x-0 bottom-4 top-4" layout>
          {items.map((item, index) => (
            <VisualItemBar
              key={item.id}
              item={item}
              index={index}
              step={step}
              maxValue={maxValue}
            />
          ))}
        </motion.div>
      </div>
    </div>
  );
}
