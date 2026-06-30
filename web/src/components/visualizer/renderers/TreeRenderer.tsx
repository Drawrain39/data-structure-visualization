import { motion } from 'framer-motion';
import type { TraceStep, VisualItem } from '../../../types';

interface Props {
  step: TraceStep | null;
}

interface TreeNode {
  id: number;
  value: number;
  x: number;
  y: number;
}

function buildTree(items: VisualItem[]): TreeNode[] {
  if (items.length === 0) return [];
  const nodes: TreeNode[] = [];

  // Place root at center-top, then BFS layout
  function place(idx: number, l: number, r: number, y: number) {
    if (idx >= items.length) return;
    const x = (l + r) / 2;
    nodes.push({ id: items[idx].id, value: items[idx].value, x, y });

    const leftIdx = 2 * idx + 1;
    const rightIdx = 2 * idx + 2;
    const nextY = y + 1;

    if (leftIdx < items.length) {
      place(leftIdx, l, x, nextY);
    }
    if (rightIdx < items.length) {
      place(rightIdx, x, r, nextY);
    }
  }

  place(0, 0, 1, 0);
  return nodes;
}

function getNodeColor(
  id: number,
  step: TraceStep | null
): { bg: string; border: string; scale: number } {
  if (!step) return { bg: 'rgba(30,41,59,0.9)', border: '#475569', scale: 1 };

  const isActive = step.active.includes(id);
  const isComparing = step.comparing.includes(id);
  const isHighlighted = step.pivot_id === id;

  if (isComparing) return { bg: 'rgba(245,158,11,0.25)', border: '#f59e0b', scale: 1.1 };
  if (isActive) return { bg: 'rgba(56,189,248,0.25)', border: '#38bdf8', scale: 1.08 };
  if (isHighlighted) return { bg: 'rgba(168,85,247,0.25)', border: '#a855f7', scale: 1.06 };

  return { bg: 'rgba(30,41,59,0.9)', border: '#475569', scale: 1 };
}

export default function TreeRenderer({ step }: Props) {
  const items = step?.items ?? [];

  if (items.length === 0) {
    return (
      <div className="flex h-full items-center justify-center text-slate-500">
        树为空
      </div>
    );
  }

  const nodes = buildTree(items);
  const nodeMap = new Map(nodes.map((n) => [n.id, n]));

  return (
    <div className="relative h-full w-full">
      <svg className="absolute inset-0 h-full w-full" viewBox="0 0 1 1" preserveAspectRatio="xMidYMid meet">
        {/* Draw edges */}
        {items.map((item, idx) => {
          const from = nodeMap.get(item.id);
          if (!from) return null;

          const edges: { to: TreeNode }[] = [];
          const leftChild = items[2 * idx + 1];
          const rightChild = items[2 * idx + 2];
          if (leftChild) {
            const to = nodeMap.get(leftChild.id);
            if (to) edges.push({ to });
          }
          if (rightChild) {
            const to = nodeMap.get(rightChild.id);
            if (to) edges.push({ to });
          }

          return edges.map(({ to }) => (
            <line
              key={`edge-${from.id}-${to.id}`}
              x1={from.x}
              y1={from.y + 0.04}
              x2={to.x}
              y2={to.y - 0.04}
              stroke="#475569"
              strokeWidth="0.004"
            />
          ));
        })}
      </svg>

      {/* Draw nodes */}
      {nodes.map((node) => {
        const { bg, border, scale } = getNodeColor(node.id, step);
        return (
          <motion.div
            key={node.id}
            layout
            animate={{ scale, backgroundColor: bg, borderColor: border }}
            className="absolute flex h-10 w-10 -translate-x-1/2 -translate-y-1/2 flex-col items-center justify-center rounded-full border-2 shadow-lg"
            style={{
              left: `${node.x * 100}%`,
              top: `${Math.max(8, node.y * 50 + 15)}%`,
            }}
          >
            <span className="text-xs font-bold text-slate-100">{node.value}</span>
          </motion.div>
        );
      })}
    </div>
  );
}
