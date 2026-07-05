import { useEffect, useRef } from 'react'
import './ContextMenu.css'

export interface CtxItem {
  label?: string
  action: () => void
  divider?: boolean
}

interface ContextMenuProps {
  x: number
  y: number
  items: CtxItem[]
  onClose: () => void
}

export function ContextMenu({ x, y, items, onClose }: ContextMenuProps) {
  const ref = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        onClose()
      }
    }
    const keyHandler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
    }
    window.addEventListener('mousedown', handler)
    window.addEventListener('keydown', keyHandler)
    return () => {
      window.removeEventListener('mousedown', handler)
      window.removeEventListener('keydown', keyHandler)
    }
  }, [onClose])

  const maxX = window.innerWidth - 200
  const maxY = window.innerHeight - 36 * items.length
  const posX = Math.min(x, maxX)
  const posY = Math.min(y, maxY)

  return (
    <div className="ctx-menu" ref={ref} style={{ left: posX, top: posY }}>
      {items.map((item, i) => (
        <div key={i}>
          {item.divider && <div className="ctx-divider" />}
            {!item.divider && (
              <button
                className="ctx-item"
                onClick={() => { item.action(); onClose() }}
              >
                {item.label}
              </button>
            )}
        </div>
      ))}
    </div>
  )
}
