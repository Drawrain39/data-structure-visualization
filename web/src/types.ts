export interface VisualItem {
  id: number;
  value: number;
}

export type StepType =
  | 'Start'
  | 'Compare'
  | 'Swap'
  | 'Overwrite'
  | 'Select'
  | 'Pivot'
  | 'Partition'
  | 'Merge'
  | 'Sorted'
  | 'Done'
  | 'Highlight'
  | 'Visit'
  | 'Push'
  | 'Pop'
  | 'Enqueue'
  | 'Dequeue'
  | 'Relax'
  | 'Rotate';

export interface TraceStep {
  step_type: StepType;
  line_key: string;
  items: VisualItem[];
  active: number[];
  comparing: number[];
  swapping: number[];
  sorted: number[];
  pivot_id: number | null;
  min_id: number | null;
  boundary_id: number | null;
  range: [number, number] | null;
  comparisons: number;
  swaps: number;
  writes: number;
  note: string;
  extra: Record<string, unknown>;
}
