import type { TraceStep } from '../../types';

interface Props {
  step: TraceStep | null;
  totalSteps: number;
  currentStep: number;
}

export default function StatsPanel({ step, totalSteps, currentStep }: Props) {
  return (
    <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
      <StatCard label="当前步数" value={`${currentStep + 1} / ${totalSteps}`} />
      <StatCard label="比较次数" value={step?.comparisons ?? 0} />
      <StatCard label="交换次数" value={step?.swaps ?? 0} />
      <StatCard label="写入次数" value={step?.writes ?? 0} />
    </div>
  );
}

function StatCard({ label, value }: { label: string; value: string | number }) {
  return (
    <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-3 text-center shadow-lg">
      <div className="text-xs text-slate-400">{label}</div>
      <div className="mt-1 text-lg font-bold text-slate-100 tabular-nums">{value}</div>
    </div>
  );
}
