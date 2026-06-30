import { algorithmMetaMap, categoryLabels, type AlgorithmKey } from '../../data/algorithmMeta';
import type { CatalogData } from '../../hooks/useWasm';

interface Props {
  value: AlgorithmKey;
  onChange: (key: AlgorithmKey) => void;
  catalog?: CatalogData | null;
}

export default function AlgorithmSelect({ value, onChange, catalog }: Props) {
  // Prefer Rust/WASM catalog when available, fall back to TS data
  const keys = catalog
    ? catalog.algorithms.map((a) => a.algorithm)
    : (Object.keys(algorithmMetaMap) as AlgorithmKey[]);

  const grouped: Record<string, string[]> = {};
  if (catalog) {
    for (const a of catalog.algorithms) {
      const cat = a.meta.category;
      if (!grouped[cat]) grouped[cat] = [];
      grouped[cat].push(a.algorithm);
    }
  } else {
    for (const key of keys) {
      const cat = algorithmMetaMap[key as AlgorithmKey].category;
      if (!grouped[cat]) grouped[cat] = [];
      grouped[cat].push(key);
    }
  }

  const catLabels: Record<string, string> = catalog
    ? Object.fromEntries(catalog.categories.map((c) => [c.key, c.label]))
    : categoryLabels;

  const getName = (algo: string) =>
    catalog?.algoMap.get(algo)?.meta.name ?? algorithmMetaMap[algo as AlgorithmKey]?.name ?? algo;

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
          <optgroup key={category} label={catLabels[category] ?? category}>
            {groupKeys.map((key) => (
              <option key={key} value={key}>
                {getName(key)}
              </option>
            ))}
          </optgroup>
        ))}
      </select>
    </div>
  );
}
