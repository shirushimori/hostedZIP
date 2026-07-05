import './StatusCard.css'

interface StatusCardProps {
  status: string
}

export function StatusCard({ status }: StatusCardProps) {
  const color = status === 'Success'    ? 'var(--green)'
    : status === 'Cleaned'              ? 'var(--green)'
    : status === 'Running'              ? 'var(--accent)'
    : status === 'Cleaning...'          ? 'var(--yellow)'
    : status === 'Failed'               ? 'var(--red)'
    : status === 'Stopped'              ? 'var(--red)'
    : status === 'Ready'                ? 'var(--text3)'
    : 'var(--accent)'

  const pulse = status === 'Cleaning...'

  return (
    <div className="status-card">
      <span className="status-label">STATUS</span>
      <span className={`status-value${pulse ? ' status-pulse' : ''}`} style={{ color }}>{status}</span>
    </div>
  )
}
