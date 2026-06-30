import { motion } from 'framer-motion';
import type { TraceStep, VisualItem } from '../../../types';

interface Props {
  step: TraceStep | null;
}

interface GraphNode {
  id: number;
  value: number;
  x: number;
  y: number;
}

function layoutCircular(items: VisualItem[]): GraphNode[] {
  if (items.length === 0) return [];
  return items.map((item, i) => {
    const angle = (2 * Math.PI * i) / items.length - Math.PI / 2;
    const radius = 0.35;
    return {
      id: item.id,
      value: item.value,
      x: 0.5 + radius * Math.cos(angle),
      y: 0.5 + radius * Math.sin(angle),
    };
  });
}

function isConnected(a: number, b: number, total: number): boolean {
  // Simple adjacency: each node connected to next, plus cross edges for demo
  if ((a + 1) % total === b) return true;
  if (a === b + 1) return true;
  return false;
}

export default function GraphRenderer({ step }: Props) {
  const items = step?.items ?? [];

  if (items.length === 0) {
    return (
      <div className="flex h-full items-center justify-center text-slate-500">
        图为空
      </div>
    );
  }

  const nodes = layoutCircular(items);
  const visitedIds = new Set(step?.sorted ?? []);
  const activeIds = new Set(step?.active ?? []);
  const comparingIds = new Set(step?.comparing ?? []);

  const edges: { from: GraphNode; to: GraphNode }[] = [];
  for (const a of nodes) {
    for (const b of nodes) {
      if (a.id !== b.id && isConnected(a.id, b.id, nodes.length)) {
        // Avoid duplicates
        if (!edges.some((e) => (e.from.id === b.id && e.to.id === a.id))) {
          edges.push({ from: a, to: b });
        }
      }
    }
  }

  return (
    <div className="relative h-full w-full">
      <svg className="absolute inset-0 h-full w-full" viewBox="0 0 1 1">
        {edges.map(({ from, to }) => (
          <line
            key={`edge-${from.id}-${to.id}`}
            x1={from.x}
            y1={from.y}
            x2={to.x}
            y2={to.y}
            stroke="#475569"
            strokeWidth="0.003"
          />
        ))}
      </svg>

      {nodes.map((node) => {
        const isVisited = visitedIds.has(node.id);
        const isActive = activeIds.has(node.id);
        const isComparing = comparingIds.has(node.id);

        const bg = isActive
          ? 'rgba(56,189,248,0.35)'
          : isComparing
            ? 'rgba(245,158,11,0.3)'
            : isVisited
              ? 'rgba(34,197,94,0.25)'
              : 'rgba(30,41,59,0.9)';

        const border = isActive ? '#38bdf8' : isComparing ? '#f59e0b' : isVisited ? '#22c55e' : '#64748b';
        const scale = isActive ? 1.15 : isComparing ? 1.1 : 1;

        return (
          <motion.div
            key={node.id}
            layout
            animate={{ scale, backgroundColor: bg, borderColor: border }}
            className="absolute flex h-12 w-12 -translate-x-1/2 -translate-y-1/2 flex-col items-center justify-center rounded-full border-2 shadow-lg"
            style={{
              left: `${node.x * 100}%`,
              top: `${node.y * 100}%`,
            }}
          >
            <span className="text-xs font-bold text-slate-100">{node.value}</span>
            <span className="text-[10px] text-slate-400">#{node.id}</span>
          </motion.div>
        );
      })}
    </div>
  );
}
