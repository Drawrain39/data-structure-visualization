import { algorithmMetaMap, categoryLabels, type AlgorithmCategory, type AlgorithmKey } from '../../data/algorithmMeta';

interface Props {
  value: AlgorithmKey;
  onChange: (key: AlgorithmKey) => void;
}

export default function AlgorithmSelect({ value, onChange }: Props) {
  const keys = Object.keys(algorithmMetaMap) as AlgorithmKey[];
  const grouped = keys.reduce(
    (acc, key) => {
      const cat = algorithmMetaMap[key].category;
      if (!acc[cat]) acc[cat] = [];
      acc[cat].push(key);
      return acc;
    },
    {} as Record<AlgorithmCategory, AlgorithmKey[]>
  );

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
        {Object.entries(grouped).map(([category, groupKeys]) => (
          <optgroup key={category} label={categoryLabels[category as AlgorithmCategory]}>
            {groupKeys.map((key) => (
              <option key={key} value={key}>
                {algorithmMetaMap[key].name}
              </option>
            ))}
          </optgroup>
        ))}
      </select>
    </div>
  );
}
