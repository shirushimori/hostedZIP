import './RunButton.css'

interface RunButtonProps {
  onClick: () => void
  loading?: boolean
  disabled?: boolean
  label?: string
  forceStop?: boolean
  onForceStop?: () => void
}

export function RunButton({ onClick, loading, disabled, label = 'Run', forceStop, onForceStop }: RunButtonProps) {
  if (forceStop) {
    return (
      <button
        className="run-btn run-btn-stop"
        onClick={onForceStop}
        title="Not Recommended"
        aria-label="Force Stop (Not Recommended)"
      >
        {/* Stop square icon */}
        <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
          <rect x="4" y="4" width="16" height="16" rx="2"/>
        </svg>
        Force Stop
      </button>
    )
  }

  return (
    <button className={`run-btn ${loading ? 'loading' : ''}`} onClick={onClick} disabled={disabled || loading}>
      {loading ? (
        <svg className="spinner" width="20" height="20" viewBox="0 0 24 24" fill="none">
          <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="3" opacity="0.25"/>
          <path d="M12 2a10 10 0 019.95 9" stroke="currentColor" strokeWidth="3" strokeLinecap="round"/>
        </svg>
      ) : (
        <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
          <path d="M6 4l16 8-16 8V4z"/>
        </svg>
      )}
      {loading ? 'Running' : label}
    </button>
  )
}
