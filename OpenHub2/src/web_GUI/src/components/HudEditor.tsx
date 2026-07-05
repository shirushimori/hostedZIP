import { useState } from 'react'
import './HudEditor.css'

interface HudEditorProps {
  onClose: () => void
}

interface HudElement {
  name: string
  label: string
  x: number // left offset in %
  y: number // top offset in %
  w: number // width in %
  h: number // height in %
  bg: string // hex color
  radius: number // border radius in px
}

const defaultElements: HudElement[] = [
  { name: 'header', label: 'Header Banner & Logo', x: 0, y: 0, w: 60, h: 32, bg: '#1A1A1A', radius: 12 },
  { name: 'tools', label: 'Tools Elevation Cards', x: 63, y: 0, w: 37, h: 32, bg: '#1A1A1A', radius: 12 },
  { name: 'gallery', label: 'Gallery Showcase Panel', x: 0, y: 36, w: 100, h: 50, bg: '#1A1A1A', radius: 12 },
  { name: 'controls', label: 'Bottom Bar Controls', x: 0, y: 89, w: 100, h: 11, bg: '#1A1A1A', radius: 12 },
]

export function HudEditor({ onClose }: HudEditorProps) {
  const [elements, setElements] = useState<HudElement[]>(defaultElements)
  const [selected, setSelected] = useState<string>('header')
  const [msg, setMsg] = useState<{ type: 'success' | 'error'; text: string } | null>(null)

  const active = elements.find(e => e.name === selected) || elements[0]

  const updateActive = (key: keyof HudElement, val: any) => {
    setElements(elements.map(e => (e.name === selected ? { ...e, [key]: val } : e)))
  }

  const handleSave = async () => {
    const serialized = JSON.stringify(elements, null, 2)
    try {
      const r = await fetch('/api/save-hud', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ layout: serialized }),
      })
      const res: { ok: boolean; path: string; error?: string } = await r.json()
      if (res.ok) {
        setMsg({ type: 'success', text: `HUD configuration saved to: ${res.path}` })
      } else {
        setMsg({ type: 'error', text: res.error || 'Failed to save layout configuration.' })
      }
    } catch (e) {
      setMsg({ type: 'error', text: `Connection failed: ${e instanceof Error ? e.message : String(e)}` })
    }
  }

  return (
    <div className="hud-editor-overlay">
      <div className="hud-editor-window">
        <div className="hud-editor-header">
          <h2>HUD Visual Editor</h2>
          <button className="sm-btn" onClick={onClose}>Close Editor</button>
        </div>

        {msg && (
          <div className={`hud-message ${msg.type}`}>
            <span>{msg.text}</span>
            <button onClick={() => setMsg(null)}>&times;</button>
          </div>
        )}

        <div className="hud-editor-main">
          {/* Visual Workspace Canvas */}
          <div className="hud-canvas">
            <div className="hud-grid-bg" />
            {elements.map(e => (
              <div
                key={e.name}
                className={`hud-element-box ${e.name === selected ? 'selected' : ''}`}
                style={{
                  left: `${e.x}%`,
                  top: `${e.y}%`,
                  width: `${e.w}%`,
                  height: `${e.h}%`,
                  background: e.bg,
                  borderRadius: `${e.radius}px`,
                }}
                onClick={() => setSelected(e.name)}
              >
                <div className="hud-element-label">
                  <strong>{e.label}</strong>
                  <span>{e.w}% &times; {e.h}%</span>
                </div>
              </div>
            ))}
          </div>

          {/* Properties Editor Sidebar */}
          <div className="hud-editor-sidebar">
            <div className="hud-editor-section">
              <label className="hud-field-label">Select Element to Edit</label>
              <select
                value={selected}
                onChange={e => setSelected(e.target.value)}
                className="hud-select"
              >
                {elements.map(e => (
                  <option key={e.name} value={e.name}>{e.label}</option>
                ))}
              </select>
            </div>

            <div className="hud-editor-section">
              <div className="hud-slider-row">
                <span className="hud-slider-label">Horizontal Position (X)</span>
                <input
                  type="range"
                  min="0"
                  max="100"
                  value={active.x}
                  onChange={e => updateActive('x', Number(e.target.value))}
                  className="hud-slider"
                />
                <span className="hud-slider-val">{active.x}%</span>
              </div>

              <div className="hud-slider-row">
                <span className="hud-slider-label">Vertical Position (Y)</span>
                <input
                  type="range"
                  min="0"
                  max="100"
                  value={active.y}
                  onChange={e => updateActive('y', Number(e.target.value))}
                  className="hud-slider"
                />
                <span className="hud-slider-val">{active.y}%</span>
              </div>

              <div className="hud-slider-row">
                <span className="hud-slider-label">Width Size (W)</span>
                <input
                  type="range"
                  min="5"
                  max="100"
                  value={active.w}
                  onChange={e => updateActive('w', Number(e.target.value))}
                  className="hud-slider"
                />
                <span className="hud-slider-val">{active.w}%</span>
              </div>

              <div className="hud-slider-row">
                <span className="hud-slider-label">Height Size (H)</span>
                <input
                  type="range"
                  min="5"
                  max="100"
                  value={active.h}
                  onChange={e => updateActive('h', Number(e.target.value))}
                  className="hud-slider"
                />
                <span className="hud-slider-val">{active.h}%</span>
              </div>

              <div className="hud-slider-row">
                <span className="hud-slider-label">Corner Border Radius</span>
                <input
                  type="range"
                  min="0"
                  max="28"
                  value={active.radius}
                  onChange={e => updateActive('radius', Number(e.target.value))}
                  className="hud-slider"
                />
                <span className="hud-slider-val">{active.radius}px</span>
              </div>

              <div className="hud-slider-row">
                <span className="hud-slider-label">Color Swatch</span>
                <input
                  type="color"
                  value={active.bg}
                  onChange={e => updateActive('bg', e.target.value)}
                  className="hud-color-picker"
                />
              </div>
            </div>

            <div className="hud-sidebar-footer">
              <button className="sm-btn primary full-width" onClick={handleSave}>
                Save & Export Layout (.txt)
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
