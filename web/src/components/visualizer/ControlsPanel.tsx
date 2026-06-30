import {
  Play,
  Pause,
  StepForward,
  StepBack,
  RotateCcw,
  SkipForward,
  SkipBack,
} from 'lucide-react';

interface Props {
  isPlaying: boolean;
  canStepForward: boolean;
  canStepBack: boolean;
  speed: number;
  onPlay: () => void;
  onPause: () => void;
  onNext: () => void;
  onPrev: () => void;
  onReset: () => void;
  onFirst: () => void;
  onLast: () => void;
  onSpeedChange: (speed: number) => void;
}

export default function ControlsPanel({
  isPlaying,
  canStepForward,
  canStepBack,
  speed,
  onPlay,
  onPause,
  onNext,
  onPrev,
  onReset,
  onFirst,
  onLast,
  onSpeedChange,
}: Props) {
  return (
    <div className="rounded-2xl border border-slate-800 bg-slate-900/60 p-4 shadow-xl">
      <div className="flex flex-wrap items-center justify-between gap-4">
        <div className="flex items-center gap-2">
          <ControlButton onClick={onFirst} disabled={!canStepBack} title="跳到开始">
            <SkipBack className="h-5 w-5" />
          </ControlButton>
          <ControlButton onClick={onPrev} disabled={!canStepBack} title="上一步">
            <StepBack className="h-5 w-5" />
          </ControlButton>

          {isPlaying ? (
            <ControlButton onClick={onPause} variant="primary" title="暂停">
              <Pause className="h-5 w-5" />
            </ControlButton>
          ) : (
            <ControlButton onClick={onPlay} variant="primary" title="开始">
              <Play className="h-5 w-5" />
            </ControlButton>
          )}

          <ControlButton onClick={onNext} disabled={!canStepForward} title="下一步">
            <StepForward className="h-5 w-5" />
          </ControlButton>
          <ControlButton onClick={onLast} disabled={!canStepForward} title="跳到结尾">
            <SkipForward className="h-5 w-5" />
          </ControlButton>

          <ControlButton onClick={onReset} title="重置" type="button">
            <RotateCcw className="h-5 w-5" />
          </ControlButton>
        </div>

        <div className="flex items-center gap-3">
          <span className="text-sm text-slate-400">速度</span>
          <input
            id="speed"
            type="range"
            min={0.2}
            max={3}
            step={0.1}
            value={speed}
            onChange={(e) => onSpeedChange(parseFloat(e.target.value))}
            className="w-32 accent-accent"
            aria-label="播放速度"
          />
          <label htmlFor="speed" className="w-12 text-right text-sm tabular-nums text-slate-300">
            {speed.toFixed(1)}x
          </label>
        </div>
      </div>
    </div>
  );
}

interface ButtonProps {
  children: React.ReactNode;
  onClick: () => void;
  disabled?: boolean;
  variant?: 'default' | 'primary';
  title?: string;
  type?: 'button' | 'submit' | 'reset';
}

function ControlButton({
  children,
  onClick,
  disabled,
  variant = 'default',
  title,
  type = 'button',
}: ButtonProps) {
  const base =
    'inline-flex h-10 w-10 items-center justify-center rounded-full transition-all';
  const styles =
    variant === 'primary'
      ? 'bg-accent text-slate-950 hover:bg-sky-300 shadow-lg shadow-accent/20'
      : 'bg-slate-800 text-slate-200 hover:bg-slate-700';
  const disabledStyle = 'opacity-40 cursor-not-allowed hover:bg-slate-800';

  return (
    <button
      type={type}
      onClick={onClick}
      disabled={disabled}
      title={title}
      className={`${base} ${styles} ${disabled ? disabledStyle : ''}`}
    >
      {children}
    </button>
  );
}
