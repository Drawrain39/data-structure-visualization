import { algorithmMetaMap, type AlgorithmKey } from '../../data/algorithmMeta';

interface Props {
  algorithm: AlgorithmKey;
}

export default function AlgorithmInfoPanel({ algorithm }: Props) {
  const meta = algorithmMetaMap[algorithm];

  return (
    <div className="rounded-2xl border border-slate-800 bg-slate-900/60 p-5 shadow-xl">
      <div className="mb-4">
        <h3 className="text-lg font-bold text-slate-50">
          {meta.name}
          <span className="ml-2 text-sm font-normal text-slate-400">{meta.nameEn}</span>
        </h3>
        <p className="mt-1 text-sm leading-relaxed text-slate-300">{meta.description}</p>
      </div>

      <div className="grid grid-cols-2 gap-4 text-sm">
        <InfoRow label="时间复杂度" value={meta.complexity.time} />
        {meta.complexity.best && <InfoRow label="最好" value={meta.complexity.best} />}
        {meta.complexity.average && <InfoRow label="平均" value={meta.complexity.average} />}
        {meta.complexity.worst && <InfoRow label="最坏" value={meta.complexity.worst} />}
        <InfoRow label="空间复杂度" value={meta.complexity.space} />
        {meta.stable && <InfoRow label="稳定性" value={meta.stable} />}
      </div>

      <div className="mt-4">
        <span className="text-xs font-medium text-slate-400">适用场景</span>
        <div className="mt-2 flex flex-wrap gap-2">
          {meta.useCases.map((use) => (
            <span
              key={use}
              className="rounded-full bg-slate-800 px-2.5 py-1 text-xs text-slate-300"
            >
              {use}
            </span>
          ))}
        </div>
      </div>
    </div>
  );
}

function InfoRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center justify-between border-b border-slate-800/60 py-1.5">
      <span className="text-slate-400">{label}</span>
      <span className="font-mono font-medium text-slate-100">{value}</span>
    </div>
  );
}
