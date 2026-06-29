export interface VisualItem {
  id: string;
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
  | 'Done';

export interface TraceStep {
  step_type: StepType;
  line_key: string;
  items: VisualItem[];
  active: string[];
  comparing: string[];
  swapping: string[];
  sorted: string[];
  pivot_id: string | null;
  min_id: string | null;
  boundary_id: string | null;
  range: [number, number] | null;
  comparisons: number;
  swaps: number;
  writes: number;
  note: string;
}
