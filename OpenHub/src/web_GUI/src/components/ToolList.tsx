import { useState } from 'react'
import type { ToolEntry } from '../types'
import './ToolList.css'

interface ToolListProps {
  tools: ToolEntry[]
  onRunTool: (name: string) => void
}

export function ToolList({ tools, onRunTool }: ToolListProps) {
  const [search, setSearch] = useState('')
  const [selected, setSelected] = useState('')
  const [collapsed, setCollapsed] = useState(false)

  const filtered = tools.filter(t =>
    t.displayName.toLowerCase().includes(search.toLowerCase())
  )

  return (
    <div className="tool-panel">
      <button className="tl-header" onClick={() => setCollapsed(!collapsed)} title={collapsed ? 'Expand tools' : 'Collapse tools'}>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"
          style={{ transform: collapsed ? 'rotate(-90deg)' : 'rotate(0deg)', transition: 'transform 0.2s' }}>
          <path d="M6 9l6 6 6-6"/>
        </svg>
        <span>Tools</span>
        <span className="tl-count">{tools.length}</span>
      </button>

      <div className={`tl-body ${collapsed ? 'collapsed' : ''}`}>
        <div className="tl-search">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/>
          </svg>
          <input
            type="text"
            placeholder="Search..."
            value={search}
            onChange={e => setSearch(e.target.value)}
          />
        </div>

        <div className="tl-items">
          {filtered.map(t => (
            <button
              key={t.displayName}
              className={`tl-item ${selected === t.displayName ? 'active' : ''}`}
              onClick={() => { setSelected(t.displayName); onRunTool(t.displayName) }}
              title={t.tooltip}
            >
              <span className="tl-name">{t.displayName}</span>
              <span className="tl-desc">{t.tooltip}</span>
            </button>
          ))}
          {filtered.length === 0 && <div className="tl-empty">No tools</div>}
        </div>
      </div>
    </div>
  )
}
