import './Header.css'

interface HeaderProps {
  logoUrl?: string
  bannerUrl?: string
  version: string
  name: string
  description: string
  userType?: string
  osSupport?: string[]
}

export function Header({ logoUrl, bannerUrl, version, name, description, osSupport }: HeaderProps) {
  return (
    <div className="header-wrap">
      {bannerUrl && (
        <div className="header-banner">
          <img src={bannerUrl} alt="" className="header-banner-img" />
        </div>
      )}
      <div className="header">
        <div className="header-logo">
          {logoUrl ? (
            <img src={logoUrl} alt="" className="header-logo-img" />
          ) : (
            <div className="header-logo-placeholder">
              <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
                <rect x="3" y="3" width="18" height="18" rx="3"/>
                <path d="M9 12l2 2 4-4"/>
              </svg>
            </div>
          )}
        </div>
        <div className="header-info">
          <div className="header-top-row">
            <h1 className="header-name">{name}</h1>
            <div className="header-stars">★★★★★</div>
          </div>
          <p className="header-desc">{description}</p>
          
          <div className="header-meta-grid">
            <div className="header-meta-item">
              <span className="h-meta-label">Version</span>
              <span className="h-meta-val">{version}</span>
            </div>
            <div className="header-meta-item">
              <span className="h-meta-label">Platform</span>
              <span className="h-meta-val">{osSupport?.join(', ') || 'Windows'}</span>
            </div>
            <div className="header-meta-item">
              <span className="h-meta-label">Developer</span>
              <span className="h-meta-val">Subh</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
