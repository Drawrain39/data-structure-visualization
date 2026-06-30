import { useCallback, useEffect, useRef, useState } from 'react';
import type { TraceStep } from '../types';
import init, {
  generate_trace_json,
  get_algorithm_catalog_json,
  get_single_code_sample,
  get_default_values_json,
} from '../../public/wasm/visualizer_wasm';

// ── Types matching Rust catalog shape ─────────────────────────────────────
export interface CatalogAlgorithm {
  algorithm: string;
  meta: CatalogMeta;
  samples: { cpp: CatalogCodeSample; python: CatalogCodeSample; rust: CatalogCodeSample };
  line_map: Record<string, number[]>;
}

export interface CatalogMeta {
  key: string;
  name: string;
  nameEn: string;
  category: string;
  description: string;
  complexity: {
    time: string;
    best?: string;
    average?: string;
    worst?: string;
    space: string;
  };
  stable?: string;
  use_cases: string[];
}

export interface CatalogCodeSample {
  language: string;
  label: string;
  lines: string[];
}

export interface CatalogData {
  categories: { key: string; label: string }[];
  algorithms: CatalogAlgorithm[];
  algoMap: Map<string, CatalogAlgorithm>;
  catMap: Map<string, CatalogAlgorithm[]>;
}

function parseCatalog(json: string): CatalogData {
  const raw = JSON.parse(json);
  const algorithms: CatalogAlgorithm[] = raw.algorithms ?? [];
  const algoMap = new Map<string, CatalogAlgorithm>();
  const catMap = new Map<string, CatalogAlgorithm[]>();
  for (const a of algorithms) {
    algoMap.set(a.algorithm, a);
    const cat = a.meta.category;
    if (!catMap.has(cat)) catMap.set(cat, []);
    catMap.get(cat)!.push(a);
  }
  return { categories: raw.categories ?? [], algorithms, algoMap, catMap };
}

export function useWasm() {
  const [ready, setReady] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [catalog, setCatalog] = useState<CatalogData | null>(null);
  const generatingRef = useRef(false);

  useEffect(() => {
    let mounted = true;
    init()
      .then(() => {
        if (!mounted) return;
        // Load catalog from Rust/WASM
        try {
          const catalogJson = get_algorithm_catalog_json();
          setCatalog(parseCatalog(catalogJson));
        } catch {
          // Catalog load failure is non-fatal
        }
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
    [ready],
  );

  // ── Catalog helpers ────────────────────────────────────────────────────
  const getCatalog = useCallback((): CatalogData | null => catalog, [catalog]);

  const getMeta = useCallback(
    (algorithm: string): CatalogMeta | undefined => catalog?.algoMap.get(algorithm)?.meta,
    [catalog],
  );

  const getCodeSample = useCallback(
    (algorithm: string): CatalogAlgorithm | undefined => catalog?.algoMap.get(algorithm),
    [catalog],
  );

  const getSingleCodeSample = useCallback(
    (algorithm: string, lang: string): string | null => {
      if (!ready) return null;
      try {
        return get_single_code_sample(algorithm, lang);
      } catch {
        return null;
      }
    },
    [ready],
  );

  const getDefaultValues = useCallback(
    (algorithm: string): number[] => {
      if (!ready) return [];
      try {
        const json = get_default_values_json(algorithm);
        return JSON.parse(json) as number[];
      } catch {
        return [];
      }
    },
    [ready],
  );

  return {
    ready,
    error,
    catalog,
    generateTrace,
    getCatalog,
    getMeta,
    getCodeSample,
    getSingleCodeSample,
    getDefaultValues,
  };
}
