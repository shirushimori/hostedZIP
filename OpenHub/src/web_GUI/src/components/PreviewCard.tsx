import { useState } from 'react'
import { ScreenshotPopup } from './ScreenshotPopup'
import './PreviewCard.css'

interface PreviewCardProps {
  items?: string[]
}

const fallback = [
  'https://picsum.photos/seed/a/400/300',
  'https://picsum.photos/seed/b/400/300',
  'https://picsum.photos/seed/c/400/300',
]

export function PreviewCard({ items }: PreviewCardProps) {
  const thumbs = items && items.length > 0 ? items : fallback
  const [idx, setIdx] = useState(0)
  const [popup, setPopup] = useState(false)

  return (
    <>
      <div className="preview-card">
        <div className="preview-img-wrap" onClick={() => setPopup(true)} style={{ cursor: 'pointer' }}>
          <img src={thumbs[idx]} alt="" className="preview-img" />
        </div>
        <div className="preview-thumbs">
          {thumbs.map((t, i) => (
            <button
              key={i}
              className={`preview-thumb ${i === idx ? 'active' : ''}`}
              onClick={() => setIdx(i)}
            >
              <img src={t} alt="" />
            </button>
          ))}
        </div>
      </div>
      {popup && (
        <ScreenshotPopup
          items={thumbs}
          index={idx}
          onNavigate={setIdx}
          onClose={() => setPopup(false)}
        />
      )}
    </>
  )
}
