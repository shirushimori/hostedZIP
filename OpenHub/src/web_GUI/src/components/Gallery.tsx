import { useState } from 'react'
import { ScreenshotPopup } from './ScreenshotPopup'
import './Gallery.css'

interface GalleryProps {
  items?: string[]
}

const fallbackShots = [
  'https://picsum.photos/seed/1/800/450',
  'https://picsum.photos/seed/2/800/450',
  'https://picsum.photos/seed/3/800/450',
]

export function Gallery({ items }: GalleryProps) {
  const shots = items && items.length > 0 ? items : fallbackShots
  const [idx, setIdx] = useState(0)
  const [popup, setPopup] = useState(false)

  const isVideo = (url: string) => /\.(mp4|webm|ogg)|youtube\.|youtu\.be|vimeo\./i.test(url)

  const prev = () => setIdx(i => (i - 1 + shots.length) % shots.length)
  const next = () => setIdx(i => (i + 1) % shots.length)

  return (
    <>
      <div className="gallery">
        <div className="gallery-main">
          <div className="gallery-img-wrap" onClick={() => setPopup(true)} style={{ cursor: 'pointer' }}>
            {isVideo(shots[idx]) ? (
              <video src={shots[idx]} className="gallery-img" controls />
            ) : (
              <img src={shots[idx]} alt="" className="gallery-img" />
            )}
          </div>
          <button className="gallery-nav gallery-nav-left" onClick={prev}>&lsaquo;</button>
          <button className="gallery-nav gallery-nav-right" onClick={next}>&rsaquo;</button>
          <div className="gallery-dots">
            {shots.map((_, i) => (
              <button
                key={i}
                className={`gallery-dot ${i === idx ? 'active' : ''}`}
                onClick={e => { e.stopPropagation(); setIdx(i) }}
              />
            ))}
          </div>
        </div>
      </div>
      {popup && (
        <ScreenshotPopup
          items={shots}
          index={idx}
          onNavigate={setIdx}
          onClose={() => setPopup(false)}
        />
      )}
    </>
  )
}
