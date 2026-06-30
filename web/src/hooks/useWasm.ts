import { useCallback, useEffect, useRef, useState } from 'react';
import type { TraceStep } from '../types';
import init, { generate_trace_json } from '../../public/wasm/visualizer_wasm';

export function useWasm() {
  const [ready, setReady] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const generatingRef = useRef(false);

  useEffect(() => {
    let mounted = true;
    init()
      .then(() => {
        if (!mounted) return;
        setReady(true);
      })
      .catch((err: unknown) => {
        if (!mounted) return;
        setError(err instanceof Error ? err.message : String(err));
      });
    return () => {
      mounted = false;
    };
  }, []);

  const generateTrace = useCallback(
    async (algorithm: string, values: number[]): Promise<TraceStep[]> => {
      if (!ready) throw new Error('WASM 尚未加载');
      if (generatingRef.current) throw new Error('正在生成 trace，请稍候');
      generatingRef.current = true;
      try {
        const json = generate_trace_json(algorithm, JSON.stringify(values));
        return JSON.parse(json) as TraceStep[];
      } finally {
        generatingRef.current = false;
      }
    },
    [ready]
  );

  return { ready, error, generateTrace };
}
