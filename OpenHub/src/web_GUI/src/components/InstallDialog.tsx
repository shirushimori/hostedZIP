import { useState } from 'react'
import './InstallDialog.css'

interface InstallDialogProps {
  defaultPath: string
  onInstall: (password?: string) => void
  onCancel: () => void
  installing: boolean
  progress: number
  step?: string
  passwordError?: boolean
}

export function InstallDialog({ defaultPath, onInstall, onCancel, installing, progress, step, passwordError }: InstallDialogProps) {
  const [password, setPassword] = useState('')

  return (
    <div className="id-overlay" onClick={onCancel}>
      <div className="id-modal" onClick={e => e.stopPropagation()}>
        <h2 className="id-title">
          {installing ? 'Installing...' : 'Install Options'}
        </h2>

        <div className="id-body">
          <label className="id-field">
            <span>Install Path (auto)</span>
            <div className="id-input-row">
              <input
                type="text"
                value={defaultPath}
                readOnly
                className="id-path-readonly"
              />
            </div>
          </label>

          <label className="id-field">
            <span>Archive Password {passwordError ? <span className="id-pw-error">(wrong password!)</span> : '(if encrypted)'}</span>
            <div className="id-input-row">
              <input
                type="password"
                value={password}
                onChange={e => setPassword(e.target.value)}
                placeholder="Leave blank if not encrypted"
                disabled={installing}
                className={`id-pw-input ${passwordError ? 'id-pw-error-input' : ''}`}
              />
            </div>
          </label>

          {installing && (
            <div className="id-progress">
              <div className="progress-bar">
                <div className="progress-fill" style={{ width: `${progress}%` }} />
              </div>
              <span className="id-progress-text">{Math.round(progress)}%</span>
            </div>
          )}
          {installing && step && (
            <div className="id-step">{step}</div>
          )}
        </div>

        <div className="id-footer">
          <button className="sm-btn" onClick={onCancel} disabled={installing}>
            Cancel
          </button>
          <button
            className="sm-btn primary"
            onClick={() => onInstall(password || undefined)}
            disabled={installing}
          >
            {installing ? 'Installing...' : 'Install'}
          </button>
        </div>
      </div>
    </div>
  )
}
