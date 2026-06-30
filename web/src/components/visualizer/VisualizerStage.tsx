import type { TraceStep } from '../../types';
import type { AlgorithmCategory, AlgorithmKey } from '../../data/algorithmMeta';
import { algorithmMetaMap } from '../../data/algorithmMeta';
import ArrayRenderer from './renderers/ArrayRenderer';
import LinkedListRenderer from './renderers/LinkedListRenderer';
import StackQueueRenderer from './renderers/StackQueueRenderer';
import CallStackRenderer from './renderers/CallStackRenderer';
import TreeRenderer from './renderers/TreeRenderer';
import GraphRenderer from './renderers/GraphRenderer';

interface Props {
  step: TraceStep | null;
  algorithm: AlgorithmKey;
}

export default function VisualizerStage({ step, algorithm }: Props) {
  const meta = algorithmMetaMap[algorithm];
  const category: AlgorithmCategory = meta?.category ?? 'sorting';

  const items = step?.items ?? [];
  const total = items.length;

  const renderer = () => {
    switch (category) {
      case 'sorting':
      case 'searching':
        return <ArrayRenderer step={step} />;
      case 'linear':
        if (algorithm === 'linked-list-traverse') {
          return <LinkedListRenderer step={step} />;
        }
        return <ArrayRenderer step={step} />;
      case 'stack-queue':
        return (
          <StackQueueRenderer
            step={step}
            variant={algorithm === 'queue-enqueue-dequeue' ? 'queue' : 'stack'}
          />
        );
      case 'recursive':
        return <CallStackRenderer step={step} />;
      case 'tree':
        return <TreeRenderer step={step} />;
      case 'graph':
        return <GraphRenderer step={step} />;
      case 'dp':
        return <ArrayRenderer step={step} />;
      default:
        return <ArrayRenderer step={step} />;
    }
  };

  return (
    <div className="relative flex h-full min-h-[320px] flex-col rounded-2xl border border-slate-800 bg-slate-900/60 p-4 shadow-2xl">
      <div className="mb-2 flex items-center justify-between text-sm text-slate-400">
        <span>动画舞台</span>
        <span>{total > 0 ? `${total} 个元素` : ''}</span>
      </div>

      <div className="relative flex-1 overflow-hidden rounded-xl bg-slate-950/60">
        {renderer()}
      </div>
    </div>
  );
}
