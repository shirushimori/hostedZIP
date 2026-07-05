export interface ThemeColors {
  bg: string
  bg2: string
  bg3: string
  bg4: string
  border: string
  text: string
  text2: string
  text3: string
  accent: string
  accent2: string
  accent3: string
  green: string
  red: string
  yellow: string
}

export interface Theme {
  id: string
  name: string
  colors: ThemeColors
}

export const themes: Theme[] = [
  {
    id: 'green',
    name: 'Green Pulse',
    colors: {
      bg: '#141414', bg2: '#1A1A1A', bg3: '#242424', bg4: '#2E2E2E',
      border: '#333333', text: '#FFFFFF', text2: '#A0A0A0', text3: '#707070',
      accent: '#16C55B', accent2: '#12A84D', accent3: '#3DDB75',
      green: '#16C55B', red: '#EF4444', yellow: '#EAB308',
    },
  },
  {
    id: 'default',
    name: 'Default Dark',
    colors: {
      bg: '#0d0d0f', bg2: '#141418', bg3: '#1c1c22', bg4: '#25252d',
      border: '#2a2a35', text: '#f0f0f5', text2: '#9a9ab0', text3: '#6a6a80',
      accent: '#7c5cfc', accent2: '#6a4ae0', accent3: '#9478ff',
      green: '#4ade80', red: '#f87171', yellow: '#fbbf24',
    },
  },
  {
    id: 'midnight',
    name: 'Midnight Blue',
    colors: {
      bg: '#0a0e1a', bg2: '#111827', bg3: '#1a2235', bg4: '#233050',
      border: '#2d3a5a', text: '#e8edf5', text2: '#94a3b8', text3: '#64748b',
      accent: '#3b82f6', accent2: '#2563eb', accent3: '#60a5fa',
      green: '#22c55e', red: '#ef4444', yellow: '#eab308',
    },
  },
  {
    id: 'matrix',
    name: 'Matrix Green',
    colors: {
      bg: '#0a0f0a', bg2: '#0f1a0f', bg3: '#1a2a1a', bg4: '#243a24',
      border: '#2a4a2a', text: '#e0f0e0', text2: '#8aba8a', text3: '#5a8a5a',
      accent: '#22c55e', accent2: '#16a34a', accent3: '#4ade80',
      green: '#22c55e', red: '#ef4444', yellow: '#eab308',
    },
  },
  {
    id: 'cyber',
    name: 'Cyber Red',
    colors: {
      bg: '#100a0a', bg2: '#1a1010', bg3: '#2a1a1a', bg4: '#3a2424',
      border: '#4a2a2a', text: '#f0e0e0', text2: '#ba8a8a', text3: '#8a5a5a',
      accent: '#ef4444', accent2: '#dc2626', accent3: '#f87171',
      green: '#22c55e', red: '#ef4444', yellow: '#eab308',
    },
  },
  {
    id: 'amber',
    name: 'Amber Glow',
    colors: {
      bg: '#100d08', bg2: '#1a1510', bg3: '#2a2218', bg4: '#3a3020',
      border: '#4a3f28', text: '#f0e8d8', text2: '#baa878', text3: '#8a7858',
      accent: '#f59e0b', accent2: '#d97706', accent3: '#fbbf24',
      green: '#22c55e', red: '#ef4444', yellow: '#f59e0b',
    },
  },
  {
    id: 'frost',
    name: 'Frost Light',
    colors: {
      bg: '#f5f7fa', bg2: '#eef1f5', bg3: '#e2e6ed', bg4: '#d1d6e0',
      border: '#c5cad4', text: '#1a1d23', text2: '#5a6070', text3: '#8a90a0',
      accent: '#6366f1', accent2: '#4f46e5', accent3: '#818cf8',
      green: '#22c55e', red: '#ef4444', yellow: '#eab308',
    },
  },
]

export interface UserSettings {
  themeId: string
  logoUrl: string
  installPath: string
  autoUpdate: boolean
  autoStart: boolean
  minimizeToTray: boolean
  discordRpc: boolean
  animation: boolean
  transparency: number
  uiScale: number
  developerMode: boolean
  // Visual Editor Override Fields
  customAccent?: string
  customBg?: string
  customBg2?: string
  customText?: string
  customBorderRadius?: number
  customSidebarWidth?: number
}

const STORAGE_KEY = 'openhub_settings'

export function loadSettings(): UserSettings {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (raw) return JSON.parse(raw) as UserSettings
  } catch {}
  return {
    themeId: 'green',
    logoUrl: '',
    installPath: '',
    autoUpdate: true,
    autoStart: false,
    minimizeToTray: false,
    discordRpc: false,
    animation: true,
    transparency: 0,
    uiScale: 100,
    developerMode: false,
    customBorderRadius: 8,
    customSidebarWidth: 220,
  }
}

export function saveSettings(s: UserSettings) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(s))
}

export function getTheme(id: string): Theme {
  return themes.find(t => t.id === id) ?? themes[0]
}

export function applyTheme(s: UserSettings) {
  const currentTheme = getTheme(s.themeId)
  const colors = { ...currentTheme.colors }

  if (s.customAccent) {
    colors.accent = s.customAccent
    colors.accent2 = s.customAccent + 'cc'
    colors.accent3 = s.customAccent + 'ff'
  }
  if (s.customBg) {
    colors.bg = s.customBg
    colors.bg3 = s.customBg
  }
  if (s.customBg2) {
    colors.bg2 = s.customBg2
    colors.bg4 = s.customBg2
  }
  if (s.customText) {
    colors.text = s.customText
  }

  const r = document.documentElement
  Object.entries(colors).forEach(([key, val]) => {
    r.style.setProperty(`--${key}`, val)
  })

  // Sizing and layout visual customization
  r.style.setProperty('--radius', `${s.customBorderRadius ?? 8}px`)
  r.style.setProperty('--radius-lg', `${(s.customBorderRadius ?? 8) * 1.5}px`)
  r.style.setProperty('--sidebar-width', `${s.customSidebarWidth ?? 220}px`)
}
