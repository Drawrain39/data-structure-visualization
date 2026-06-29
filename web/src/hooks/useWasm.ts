import { useEffect, useState } from 'react';
import type { TraceStep } from '../types';
import init, { generate_trace_json } from '../../public/wasm/visualizer_wasm';

export function useWasm() {
  const [ready, setReady] = useState(false);
  const [error, setError] = useState<string | null>(null);

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

  const generateTrace = async (algorithm: string, values: number[]): Promise<TraceStep[]> => {
    if (!ready) throw new Error('WASM 尚未加载');
    const json = generate_trace_json(algorithm, JSON.stringify(values));
    return JSON.parse(json) as TraceStep[];
  };

  return { ready, error, generateTrace };
}
