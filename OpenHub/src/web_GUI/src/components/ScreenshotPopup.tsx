import { useState, useEffect } from 'react'
import './ScreenshotPopup.css'

interface ScreenshotPopupProps {
  items: string[]
  index: number
  onNavigate: (i: number) => void
  onClose: () => void
}

export function ScreenshotPopup({ items, index, onNavigate, onClose }: ScreenshotPopupProps) {
  const [zoom, setZoom] = useState(1)
  const [pos, setPos] = useState({ x: 0, y: 0 })
  const [dragging, setDragging] = useState(false)
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 })
  const url = items[index]
  const isVideo = url?.match(/\.(mp4|webm|ogg)|youtube\.|youtu\.be|vimeo\./i)

  const prev = () => onNavigate((index - 1 + items.length) % items.length)
  const next = () => onNavigate((index + 1) % items.length)

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
      if (e.key === 'ArrowLeft') prev()
      if (e.key === 'ArrowRight') next()
      if (e.key === '=' || e.key === '+') setZoom(z => Math.min(z + 0.25, 4))
      if (e.key === '-') setZoom(z => Math.max(z - 0.25, 0.25))
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [onClose, index])

  const handleWheel = (e: React.WheelEvent) => {
    e.preventDefault()
    const delta = e.deltaY > 0 ? -0.15 : 0.15
    setZoom(z => Math.max(0.25, Math.min(4, z + delta)))
  }

  const handleMouseDown = (e: React.MouseEvent) => {
    if (zoom <= 1) return
    setDragging(true)
    setDragStart({ x: e.clientX - pos.x, y: e.clientY - pos.y })
  }

  const handleMouseMove = (e: React.MouseEvent) => {
    if (!dragging) return
    setPos({ x: e.clientX - dragStart.x, y: e.clientY - dragStart.y })
  }

  const handleMouseUp = () => setDragging(false)

  return (
    <div className="sp-overlay" onClick={onClose} onMouseUp={handleMouseUp} onMouseMove={handleMouseMove}>
      <div className="sp-controls" onClick={e => e.stopPropagation()}>
        <span className="sp-counter">{index + 1} / {items.length}</span>
        <span className="sp-zoom-label">{Math.round(zoom * 100)}%</span>
        <button onClick={() => setZoom(z => Math.max(z - 0.25, 0.25))}>-</button>
        <button onClick={() => setZoom(z => Math.min(z + 0.25, 4))}>+</button>
        <button onClick={() => { setZoom(1); setPos({ x: 0, y: 0 }) }}>Reset</button>
        <button className="sp-close" onClick={onClose}>
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>

      <button className="sp-nav sp-nav-left" onClick={e => { e.stopPropagation(); prev() }}>&lsaquo;</button>
      <button className="sp-nav sp-nav-right" onClick={e => { e.stopPropagation(); next() }}>&rsaquo;</button>

      <div
        className="sp-content"
        onClick={e => e.stopPropagation()}
        onWheel={handleWheel}
        onMouseDown={handleMouseDown}
        style={{ cursor: zoom > 1 ? (dragging ? 'grabbing' : 'grab') : 'default' }}
      >
        {isVideo ? (
          <video
            src={url}
            controls
            autoPlay
            className="sp-video"
            style={{ transform: `scale(${zoom}) translate(${pos.x / zoom}px, ${pos.y / zoom}px)` }}
          />
        ) : (
          <img
            src={url}
            alt=""
            className="sp-image"
            style={{ transform: `scale(${zoom}) translate(${pos.x / zoom}px, ${pos.y / zoom}px)` }}
            draggable={false}
          />
        )}
      </div>
    </div>
  )
}
