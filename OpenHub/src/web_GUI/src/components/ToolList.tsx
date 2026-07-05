import { useState, useRef, useEffect } from 'react'
import type { ToolEntry } from '../types'
import './ToolList.css'

interface ToolListProps {
  tools: ToolEntry[]
  onRunTool: (name: string, asAdmin?: boolean) => void
  disabled?: boolean
}

interface ToolCtx {
  name: string
  x: number
  y: number
}

export function ToolList({ tools, onRunTool, disabled }: ToolListProps) {
  const [search, setSearch] = useState('')
  const [selected, setSelected] = useState('')
  const [collapsed, setCollapsed] = useState(false)
  const [ctx, setCtx] = useState<ToolCtx | null>(null)
  const ctxRef = useRef<HTMLDivElement>(null)

  const filtered = tools.filter(t =>
    t.displayName.toLowerCase().includes(search.toLowerCase())
  )

  const categories = [
    {
      name: 'Utilities',
      items: filtered.filter(t => /clean|batch|sys|tool|bat/i.test(t.displayName) || /clean|utility/i.test(t.tooltip)),
    },
    {
      name: 'Mods & Cheats',
      items: filtered.filter(t => /esp|aim|cheat|hack|bypass/i.test(t.displayName) || /esp|aim/i.test(t.tooltip)),
    },
    {
      name: 'Other Utilities',
      items: filtered.filter(t => 
        !/clean|batch|sys|tool|bat/i.test(t.displayName) && !/clean|utility/i.test(t.tooltip) &&
        !/esp|aim|cheat|hack|bypass/i.test(t.displayName) && !/esp|aim/i.test(t.tooltip)
      ),
    }
  ]

  // Close context menu on outside click or Escape
  useEffect(() => {
    if (!ctx) return
    const onDown = (e: MouseEvent) => {
      if (ctxRef.current && !ctxRef.current.contains(e.target as Node)) {
        setCtx(null)
      }
    }
    const onKey = (e: KeyboardEvent) => { if (e.key === 'Escape') setCtx(null) }
    document.addEventListener('mousedown', onDown)
    document.addEventListener('keydown', onKey)
    return () => { document.removeEventListener('mousedown', onDown); document.removeEventListener('keydown', onKey) }
  }, [ctx])

  const openCtx = (e: React.MouseEvent, name: string) => {
    e.preventDefault()
    e.stopPropagation()
    setCtx({ name, x: e.clientX, y: e.clientY })
  }

  // Always run as admin — plain run is not exposed
  const handleRun = (name: string) => {
    if (disabled) return
    setSelected(name)
    setCtx(null)
    onRunTool(name, true)
  }

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
          {categories.map(cat => {
            if (cat.items.length === 0) return null;
            return (
              <div key={cat.name} className="tl-cat-section">
                <div className="tl-cat-title">{cat.name}</div>
                <div className="tl-cat-divider" />
                <div className="tl-cat-list">
                  {cat.items.map(t => (
                    <button
                      key={t.displayName}
                      className={`tl-item ${selected === t.displayName ? 'active' : ''} ${disabled ? 'tl-disabled' : ''}`}
                      title={`${t.tooltip} — Run as Admin`}
                      onClick={() => handleRun(t.displayName)}
                      onDoubleClick={e => { e.preventDefault(); handleRun(t.displayName) }}
                      onContextMenu={e => { if (!disabled) openCtx(e, t.displayName) }}
                    >
                      <div className="tl-item-content">
                        <span className="tl-checkmark">✓</span>
                        <div className="tl-item-meta">
                          <span className="tl-name">{t.displayName}</span>
                          <span className="tl-desc">{t.tooltip}</span>
                        </div>
                      </div>
                      <span className="tl-hint">Run as Admin</span>
                    </button>
                  ))}
                </div>
              </div>
            );
          })}
          {filtered.length === 0 && <div className="tl-empty">No tools found</div>}
        </div>
      </div>

      {/* Tool context menu — admin only */}
      {ctx && (
        <div
          ref={ctxRef}
          className="tool-ctx-menu"
          style={{ left: ctx.x, top: ctx.y }}
        >
          <div className="tool-ctx-label">SUPER CLEANER</div>
          <button className="tool-ctx-item tool-ctx-admin" onClick={() => handleRun(ctx.name)}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
            </svg>
            Run as Admin
          </button>
        </div>
      )}
    </div>
  )
}
