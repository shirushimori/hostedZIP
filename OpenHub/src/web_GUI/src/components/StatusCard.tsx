import './StatusCard.css'

interface StatusCardProps {
  status: string
}

export function StatusCard({ status }: StatusCardProps) {
  const color = status === 'Success' ? 'var(--green)'
    : status === 'Running' ? 'var(--accent)'
    : status === 'Failed' ? 'var(--red)'
    : status === 'Ready' ? 'var(--text3)'
    : 'var(--accent)'

  return (
    <div className="status-card">
      <span className="status-label">STATUS</span>
      <span className="status-value" style={{ color }}>{status}</span>
    </div>
  )
}
