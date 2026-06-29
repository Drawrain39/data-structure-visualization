import type { TraceStep } from '../../types';
import { getStepDescription } from '../../data/algorithmMeta';

interface Props {
  step: TraceStep | null;
}

export default function StepExplanation({ step }: Props) {
  if (!step) {
    return (
      <div className="rounded-2xl border border-slate-800 bg-slate-900/60 p-4 shadow-xl">
        <div className="text-sm text-slate-400">点击“开始”运行算法</div>
      </div>
    );
  }

  return (
    <div className="rounded-2xl border border-slate-800 bg-slate-900/60 p-4 shadow-xl">
      <div className="mb-1 text-xs font-medium uppercase tracking-wider text-slate-500">
        {step.step_type}
      </div>
      <p className="text-sm leading-relaxed text-slate-200">{getStepDescription(step)}</p>
      {step.note && step.note !== getStepDescription(step) && (
        <p className="mt-2 text-xs text-slate-400">{step.note}</p>
      )}
    </div>
  );
}
