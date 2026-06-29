import { algorithmMetaMap, type AlgorithmKey } from '../../data/algorithmMeta';

interface Props {
  value: AlgorithmKey;
  onChange: (key: AlgorithmKey) => void;
}

export default function AlgorithmSelect({ value, onChange }: Props) {
  return (
    <div className="flex items-center gap-3">
      <label htmlFor="algorithm" className="text-sm font-medium text-slate-300">
        算法
      </label>
      <select
        id="algorithm"
        value={value}
        onChange={(e) => onChange(e.target.value as AlgorithmKey)}
        className="rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-100 outline-none focus:border-accent focus:ring-1 focus:ring-accent"
      >
        {(Object.keys(algorithmMetaMap) as AlgorithmKey[]).map((key) => (
          <option key={key} value={key}>
            {algorithmMetaMap[key].name}
          </option>
        ))}
      </select>
    </div>
  );
}
