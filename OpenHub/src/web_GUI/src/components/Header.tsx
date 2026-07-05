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

export function Header({ logoUrl, bannerUrl, version, name, description, userType, osSupport }: HeaderProps) {
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
          <div className="header-meta">
            <span className="header-version">{version}</span>
            {userType && <span className="header-usertype">{userType}</span>}
          </div>
          <h1 className="header-name">{name}</h1>
          <p className="header-desc">{description}</p>
          {osSupport && osSupport.length > 0 && (
            <div className="header-os">
              {osSupport.map(os => <span key={os} className="header-os-tag">{os}</span>)}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
