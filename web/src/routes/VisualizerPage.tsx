import { useCallback, useEffect, useRef, useState } from 'react';
import { motion } from 'framer-motion';
import VisualizerStage from '../components/visualizer/VisualizerStage';
import CodePanel from '../components/visualizer/CodePanel';
import ControlsPanel from '../components/visualizer/ControlsPanel';
import StatsPanel from '../components/visualizer/StatsPanel';
import AlgorithmInfoPanel from '../components/visualizer/AlgorithmInfoPanel';
import StepExplanation from '../components/visualizer/StepExplanation';
import AlgorithmSelect from '../components/visualizer/AlgorithmSelect';
import { useWasm } from '../hooks/useWasm';
import type { AlgorithmKey } from '../data/algorithmMeta';
import type { TraceStep } from '../types';
import type { Language } from '../data/codeSamples';

const defaultValues: Record<AlgorithmKey, number[]> = {
  'selection-sort': [64, 25, 12, 22, 11, 89, 34, 55],
  'bubble-sort': [64, 25, 12, 22, 11, 89, 34, 55],
  'insertion-sort': [64, 25, 12, 22, 11, 89, 34, 55],
  'merge-sort': [38, 27, 43, 3, 9, 82, 10],
  'quick-sort': [10, 7, 8, 9, 1, 5],
  'heap-sort': [12, 11, 13, 5, 6, 7],
  'linear-search': [3, 1, 4, 1, 5, 9, 2, 6],
  'binary-search': [1, 2, 3, 4, 5, 6, 7, 8, 9],
  'array-insert': [1, 2, 3, 4, 5, 99],
  'array-delete': [10, 20, 30, 40, 50],
  'linked-list-traverse': [10, 20, 30, 40, 50],
  'stack-push-pop': [1, 2, 3, 4, 5],
  'queue-enqueue-dequeue': [1, 2, 3, 4, 5],
  'factorial': [5],
  'fibonacci': [8],
  'tower-of-hanoi': [3],
  'bst-insert': [50, 30, 70, 20, 40, 60, 80],
  'bst-search': [50, 30, 70, 20, 40, 60, 80],
  'heap-insert': [10, 20, 5, 30, 15],
  'bfs': [1, 2, 3, 4, 5, 6],
  'dfs': [1, 2, 3, 4, 5, 6],
};

export default function VisualizerPage() {
  const { ready, error, generateTrace } = useWasm();
  const [algorithm, setAlgorithm] = useState<AlgorithmKey>('quick-sort');
  const [values, setValues] = useState<number[]>(defaultValues['quick-sort']);
  const [steps, setSteps] = useState<TraceStep[]>([]);
  const [current, setCurrent] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [speed, setSpeed] = useState(1.5);
  const [language, setLanguage] = useState<Language>('rust');
  const [loading, setLoading] = useState(false);
  const playRef = useRef(isPlaying);
  playRef.current = isPlaying;

  const step = steps[current] ?? null;

  const loadTrace = useCallback(async () => {
    if (!ready) return;
    setLoading(true);
    setIsPlaying(false);
    setCurrent(0);
    try {
      const trace = await generateTrace(algorithm, values);
      setSteps(trace);
    } catch (e) {
      // eslint-disable-next-line no-console
      console.error(e);
    } finally {
      setLoading(false);
    }
  }, [ready, algorithm, values, generateTrace]);

  useEffect(() => {
    setValues(defaultValues[algorithm]);
  }, [algorithm]);

  useEffect(() => {
    loadTrace();
  }, [loadTrace]);

  useEffect(() => {
    if (!isPlaying) return;
    let raf = 0;
    let last = performance.now();
    const interval = 700 / speed;

    const tick = (now: number) => {
      if (!playRef.current) return;
      if (now - last >= interval) {
        last = now;
        setCurrent((prev) => {
          if (prev >= steps.length - 1) {
            setIsPlaying(false);
            return prev;
          }
          return prev + 1;
        });
      }
      raf = requestAnimationFrame(tick);
    };

    raf = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(raf);
  }, [isPlaying, steps.length, speed]);

  const handlePlay = () => {
    if (current >= steps.length - 1) {
      setCurrent(0);
    }
    setIsPlaying(true);
  };

  const handlePause = () => setIsPlaying(false);
  const handleNext = () => setCurrent((c) => Math.min(c + 1, steps.length - 1));
  const handlePrev = () => setCurrent((c) => Math.max(c - 1, 0));
  const handleReset = () => {
    setIsPlaying(false);
    setCurrent(0);
  };
  const handleFirst = () => setCurrent(0);
  const handleLast = () => setCurrent(steps.length - 1);

  const handleRandomize = () => {
    const next = Array.from({ length: 10 }, () => Math.floor(Math.random() * 90) + 10);
    setValues(next);
  };

  const handleCustomInput = (raw: string) => {
    const parsed = raw
      .split(/[,，\s]+/)
      .map((s) => parseInt(s.trim(), 10))
      .filter((n) => !Number.isNaN(n))
      .slice(0, 30);
    if (parsed.length > 0) {
      setValues(parsed);
    }
  };

  return (
    <div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
      <div className="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-2xl font-bold text-slate-50">算法可视化</h1>
          <p className="text-sm text-slate-400">排序核心运行在 Rust / WebAssembly 中</p>
        </div>

        <div className="flex flex-wrap items-center gap-4">
          <AlgorithmSelect value={algorithm} onChange={setAlgorithm} />
          <input
            type="text"
            key={algorithm}
            defaultValue={values.join(', ')}
            onBlur={(e) => handleCustomInput(e.target.value)}
            placeholder="输入数字，逗号分隔"
            className="w-48 rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-100 outline-none focus:border-accent"
          />
          <button
            onClick={handleRandomize}
            className="rounded-lg bg-slate-800 px-3 py-2 text-sm font-medium text-slate-200 hover:bg-slate-700"
          >
            随机数据
          </button>
        </div>
      </div>

      {!ready && (
        <div className="mb-4 rounded-xl border border-slate-800 bg-slate-900/60 p-4 text-sm text-slate-400">
          正在加载 WebAssembly 模块...
        </div>
      )}
      {error && (
        <div className="mb-4 rounded-xl border border-red-900/40 bg-red-950/30 p-4 text-sm text-red-200">
          WASM 加载失败：{error}
        </div>
      )}

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
        <div className="flex flex-col gap-4 lg:col-span-2">
          <VisualizerStage step={step} />
          <ControlsPanel
            isPlaying={isPlaying}
            canStepForward={current < steps.length - 1}
            canStepBack={current > 0}
            speed={speed}
            onPlay={handlePlay}
            onPause={handlePause}
            onNext={handleNext}
            onPrev={handlePrev}
            onReset={handleReset}
            onFirst={handleFirst}
            onLast={handleLast}
            onSpeedChange={setSpeed}
          />
          <StatsPanel
            step={step}
            totalSteps={steps.length}
            currentStep={current}
          />
          <StepExplanation step={step} />
        </div>

        <div className="flex flex-col gap-4">
          <div className="h-[420px] lg:h-[520px]">
            <CodePanel
              algorithm={algorithm}
              language={language}
              step={step}
              onLanguageChange={setLanguage}
            />
          </div>
          <AlgorithmInfoPanel algorithm={algorithm} />
        </div>
      </div>

      {loading && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="pointer-events-none fixed inset-0 z-40 flex items-center justify-center bg-slate-950/30"
        >
          <div className="rounded-xl bg-slate-900 px-4 py-2 text-sm text-slate-200 shadow-xl">
            生成 trace 中...
          </div>
        </motion.div>
      )}
    </div>
  );
}
