import './RunButton.css'

interface RunButtonProps {
  onClick: () => void
  loading?: boolean
}

export function RunButton({ onClick, loading }: RunButtonProps) {
  return (
    <button className={`run-btn ${loading ? 'loading' : ''}`} onClick={onClick} disabled={loading}>
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
      {loading ? 'Running' : 'Run'}
    </button>
  )
}
