import { algorithmMetaMap, type AlgorithmKey } from '../../data/algorithmMeta';
import { getCodeSample, type Language } from '../../data/codeSamples';
import type { TraceStep } from '../../types';
import type { CatalogData } from '../../hooks/useWasm';

interface Props {
  algorithm: AlgorithmKey;
  language: Language;
  step: TraceStep | null;
  onLanguageChange: (lang: Language) => void;
  catalog?: CatalogData | null;
}

const languages: { key: Language; label: string }[] = [
  { key: 'cpp', label: 'C++' },
  { key: 'python', label: 'Python' },
  { key: 'rust', label: 'Rust' },
];

export default function CodePanel({ algorithm, language, step, onLanguageChange, catalog }: Props) {
  // Prefer Rust/WASM catalog for code samples
  const rustAlgo = catalog?.algoMap.get(algorithm);
  const tsSample = getCodeSample(algorithm);

  const current = rustAlgo
    ? rustAlgo.samples[language]
    : tsSample?.samples[language];

  const lineMap: Record<string, number[]> = rustAlgo
    ? rustAlgo.line_map
    : tsSample?.lineMap ?? {};

  const activeLines = (() => {
    if (!step) return new Set<number>();
    return new Set(lineMap[step.line_key] ?? []);
  })();

  const algoName = rustAlgo?.meta.name ?? algorithmMetaMap[algorithm]?.name ?? algorithm;

  return (
    <div className="flex h-full flex-col rounded-2xl border border-slate-800 bg-slate-900/80 shadow-2xl overflow-hidden">
      <div className="flex items-center justify-between border-b border-slate-800 px-4 py-3">
        <span className="text-sm font-medium text-slate-200">代码</span>
        <div className="flex items-center gap-1 rounded-lg bg-slate-950 p-1">
          {languages.map((lang) => (
            <button
              key={lang.key}
              type="button"
              onClick={() => onLanguageChange(lang.key)}
              className={`rounded-md px-3 py-1 text-xs font-medium transition-colors ${
                language === lang.key
                  ? 'bg-accent/15 text-accent'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              {lang.label}
            </button>
          ))}
        </div>
      </div>

      <div className="code-panel flex-1 overflow-auto bg-slate-950 p-4">
        {current ? (
          <pre className="m-0">
            {current.lines.map((line: string, idx: number) => {
              const lineNumber = idx + 1;
              const isActive = activeLines.has(lineNumber);
              return (
                <div
                  key={idx}
                  className={`code-line ${isActive ? 'active' : ''}`}
                >
                  <code className={isActive ? 'text-slate-50' : 'text-slate-400'}>
                    {line || ' '}
                  </code>
                </div>
              );
            })}
          </pre>
        ) : (
          <div className="text-slate-500">暂无 {algoName} 的代码示例</div>
        )}
      </div>
    </div>
  );
}
