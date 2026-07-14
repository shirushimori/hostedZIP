import './Sidebar.css'

interface SidebarProps {
  active: string
  onNavigate: (page: string) => void
}

const items = [
  { id: 'home', label: 'Home', icon: 'M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6' },
  { id: 'downloads', label: 'Downloads', icon: 'M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' },
  { id: 'announcement', label: 'Announcement', icon: 'M11 5L6 9H2v6h4l5 4V5zM19 8a6 6 0 0 1 0 8' },
]

export function Sidebar({ active, onNavigate }: SidebarProps) {
  return (
    <nav className="sidebar">
      {items.map(item => (
        <button
          key={item.id}
          className={`sidebar-icon ${active === item.id ? 'active' : ''}`}
          onClick={() => onNavigate(item.id)}
          title={item.label}
        >
          <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
            <path d={item.icon} />
          </svg>
        </button>
      ))}
      <div className="sidebar-spacer" />
    </nav>
  )
}
