import { useMemo } from 'react';
import type { TraceStep } from '../../types';
import { getCodeSample, getLineNumbers, type Language } from '../../data/codeSamples';

interface Props {
  algorithm: string;
  language: Language;
  step: TraceStep | null;
  onLanguageChange: (lang: Language) => void;
}

const languages: { key: Language; label: string }[] = [
  { key: 'cpp', label: 'C++' },
  { key: 'python', label: 'Python' },
  { key: 'rust', label: 'Rust' },
];

export default function CodePanel({ algorithm, language, step, onLanguageChange }: Props) {
  const sample = useMemo(() => getCodeSample(algorithm), [algorithm]);
  const current = sample?.samples[language];
  const activeLines = useMemo(() => {
    if (!sample || !step) return new Set<number>();
    return new Set(getLineNumbers(sample, step.line_key));
  }, [sample, step]);

  return (
    <div className="flex h-full flex-col rounded-2xl border border-slate-800 bg-slate-900/80 shadow-2xl overflow-hidden">
      <div className="flex items-center justify-between border-b border-slate-800 px-4 py-3">
        <span className="text-sm font-medium text-slate-200">代码</span>
        <div className="flex items-center gap-1 rounded-lg bg-slate-950 p-1">
          {languages.map((lang) => (
            <button
              key={lang.key}
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
            {current.lines.map((line, idx) => {
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
          <div className="text-slate-500">暂无代码</div>
        )}
      </div>
    </div>
  );
}
