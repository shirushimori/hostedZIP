import { useState } from 'react'
import type { Source } from '../types'
import './TopBar.css'

interface TopBarProps {
  appName: string
  onSettingsOpen: () => void
  sources: Source[]
  activeSource: string | null
  onSelectSource: (url: string, name: string) => void
  loading: boolean
}

export function TopBar({ appName, onSettingsOpen, sources, activeSource, onSelectSource, loading }: TopBarProps) {
  const [open, setOpen] = useState(false)

  return (
    <header className="topbar">
      <div className="topbar-left">
        <div className="app-selector" onClick={() => !loading && setOpen(!open)} tabIndex={0}>
          {loading ? (
            <span className="app-selector-label" style={{ opacity: 0.5 }}>Loading...</span>
          ) : (
            <span className="app-selector-label">{activeSource || appName}</span>
          )}
          <svg className={`chevron ${open ? 'up' : ''}`} width="12" height="12" viewBox="0 0 12 12" fill="none">
            <path d="M3 5l3 3 3-3" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
          </svg>
          {open && (
            <>
              <div className="app-dropdown-overlay" onClick={() => setOpen(false)} />
              <div className="app-dropdown">
                {sources.length === 0 && (
                  <div className="app-dropdown-item disabled">No sources available</div>
                )}
                {sources.map(s => (
                  <div
                    key={s.name}
                    className={`app-dropdown-item ${activeSource === s.name ? 'active' : ''}`}
                    onClick={() => { onSelectSource(s.url, s.name); setOpen(false) }}
                  >
                    {s.name}
                  </div>
                ))}
              </div>
            </>
          )}
        </div>
      </div>

      <button className="settings-gear" onClick={onSettingsOpen} title="Settings">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
        </svg>
      </button>
    </header>
  )
}
