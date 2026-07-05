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

  const prev = () => setIdx(i => (i - 1 + shots.length) % shots.length)
  const next = () => setIdx(i => (i + 1) % shots.length)

  return (
    <>
      <div className="gallery">
        <div id="carouselExampleCaptions" className="carousel slide">
          <div className="carousel-indicators">
            {shots.map((_, i) => (
              <button
                key={i}
                type="button"
                className={i === idx ? 'active' : ''}
                aria-current={i === idx ? 'true' : undefined}
                aria-label={`Slide ${i + 1}`}
                onClick={() => setIdx(i)}
              />
            ))}
          </div>
          <div className="carousel-inner">
            {shots.map((t, i) => {
              const active = i === idx
              let label = ''
              let desc = ''
              if (i === 0) {
                label = 'Subh X Cheats'
                desc = 'Cheats works and We Have Nice UseerFlow.'
              } else if (i === 1) {
                label = 'BEST PANEL'
                desc = 'All in One Panel'
              } else if (i === 2) {
                label = 'What?'
                desc = 'bro my both subscription should have been working both was permanent.'
              } else {
                label = `Slide ${i + 1}`
                desc = 'Screenshot showcase preview.'
              }

              return (
                <div key={i} className={`carousel-item ${active ? 'active' : ''}`}>
                  <img
                    src={t}
                    className="d-block w-100 gallery-img"
                    alt={label}
                    onClick={() => setPopup(true)}
                    style={{ cursor: 'pointer' }}
                  />
                  <div className="carousel-caption d-none d-md-block">
                    <h5>{label}</h5>
                    <p>{desc}</p>
                  </div>
                </div>
              )
            })}
          </div>
          <button className="carousel-control-prev" type="button" onClick={prev}>
            <span className="carousel-control-prev-icon" aria-hidden="true">‹</span>
            <span className="visually-hidden">Previous</span>
          </button>
          <button className="carousel-control-next" type="button" onClick={next}>
            <span className="carousel-control-next-icon" aria-hidden="true">›</span>
            <span className="visually-hidden">Next</span>
          </button>
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
