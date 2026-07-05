import { useState, useEffect, useCallback } from 'react'
import type { ServiceData, Source, ProgressResponse } from './types'
import { getService } from './api'
import { loadSettings, saveSettings, applyTheme, getTheme, type UserSettings } from './theme'
import { TopBar } from './components/TopBar'
import { Sidebar } from './components/Sidebar'
import { Header } from './components/Header'
import { Gallery } from './components/Gallery'
import { PreviewCard } from './components/PreviewCard'
import { ToolList } from './components/ToolList'
import { ProgressBar } from './components/ProgressBar'
import { StatusCard } from './components/StatusCard'
import { RunButton } from './components/RunButton'
import { SettingsDialog } from './components/SettingsDialog'
import { InstallDialog } from './components/InstallDialog'
import { SetupWizard } from './components/SetupWizard'
import { ContextMenu, type CtxItem } from './components/ContextMenu'
import './App.css'

export default function App() {
  const [settings, setSettings] = useState<UserSettings>(loadSettings)
  const [showSetup, setShowSetup] = useState(() => !localStorage.getItem('openhub_setup_done'))
  const [settingsOpen, setSettingsOpen] = useState(false)
  const [data, setData] = useState<ServiceData | null>(null)
  const [localVer, setLocalVer] = useState('')
  const [navPage, setNavPage] = useState('home')
  const [progress, setProgress] = useState(0)
  const [status, setStatus] = useState('Ready')
  const [running, setRunning] = useState(false)
  const [sources, setSources] = useState<Source[]>([])
  const [activeSourceName, setActiveSourceName] = useState<string | null>(null)
  const [loadingSource, setLoadingSource] = useState(false)
  const [installDialog, setInstallDialog] = useState(false)
  const [installing, setInstalling] = useState(false)
  const [installProgress, setInstallProgress] = useState(0)
  const [installStep, setInstallStep] = useState('')
  const [passwordError, setPasswordError] = useState(false)
  const [errorDialog, setErrorDialog] = useState<{ title: string; message: string } | null>(null)
  const [ctxMenu, setCtxMenu] = useState<{ x: number; y: number; items: CtxItem[] } | null>(null)


  useEffect(() => {
    const t = getTheme(settings.themeId)
    applyTheme(t.colors)
  }, [settings.themeId])

  useEffect(() => {
    fetch('/data/sources.json')
      .then(r => r.json())
      .then((list: Source[]) => {
        setSources(list)
        if (list.length > 0) {
          setActiveSourceName(list[0].name)
          fetchSource(list[0].url, list[0].name)
        }
      })
      .catch(() => {
        getService().then(r => {
          if (r.data) setData(r.data)
          setLocalVer(r.local_version)
        })
      })
  }, [])

  const fetchSource = useCallback(async (url: string, name: string) => {
    setLoadingSource(true)
    setActiveSourceName(name)
    try {
      const r = await fetch(url)
      const svc: ServiceData = await r.json()
      setData(svc)
      setProgress(0)
      setStatus('Ready')
      setRunning(false)
      setInstallDialog(false)
      setInstalling(false)
      setInstallProgress(0)
      setLocalVer('')
    } catch (e) {
      setStatus('Failed')
      setErrorDialog({ title: 'Load Failed', message: `Could not load service source "${name}": ${e instanceof Error ? e.message : String(e)}` })
    } finally {
      setLoadingSource(false)
    }
  }, [])

  const updateSettings = (s: UserSettings) => { setSettings(s); saveSettings(s) }

  const handleSetupComplete = (partial: Partial<UserSettings>) => {
    const merged = { ...settings, ...partial }
    updateSettings(merged)
    localStorage.setItem('openhub_setup_done', '1')
    setShowSetup(false)
  }

  const handleInstall = async (password?: string) => {
    setInstalling(true)
    setInstallProgress(0)
    setInstallStep('Starting...')
    setPasswordError(false)
    setStatus('Installing')

    const poll = setInterval(async () => {
      try {
        const r = await fetch('/api/progress')
        const p: ProgressResponse = await r.json()
        setInstallProgress(p.progress)
        setInstallStep(p.step)
        if (p.error) {
          setInstallStep(p.error)
        }
        if (p.done) {
          clearInterval(poll)
        }
      } catch { /* ignore poll errors */ }
    }, 200)

    try {
      const r = await fetch('/api/install', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ password: password || null }),
      })
      const res: { installed?: boolean; local_version?: string; error?: string } = await r.json()
      clearInterval(poll)

      // Final progress read
      try {
        const pr = await fetch('/api/progress')
        const pp: ProgressResponse = await pr.json()
        if (pp.progress > 0) setInstallProgress(pp.progress)
        if (pp.step) setInstallStep(pp.step)
      } catch { /* ok */ }

      if (res.error === 'WRONG_PASSWORD' || res.error === 'PASSWORD_REQUIRED') {
        setPasswordError(true)
        setInstalling(false)
        setStatus('Password required')
        setInstallProgress(0)
        setInstallStep('')
        return
      }

      setInstallProgress(100)
      setInstallStep('Complete')
      setTimeout(() => {
        setInstalling(false)
        setInstallDialog(false)
        if (res.installed) {
          setLocalVer(res.local_version || (data ? data.ServiceVersion : ''))
          setStatus('Ready')
          setProgress(0)
        } else {
          setStatus('Failed')
          setErrorDialog({ title: 'Install Failed', message: res.error || 'Installation completed but service is not installed' })
        }
      }, 400)
    } catch (e) {
      clearInterval(poll)
      setInstalling(false)
      setStatus('Failed')
      setErrorDialog({ title: 'Install Failed', message: `Install request failed: ${e instanceof Error ? e.message : String(e)}` })
    }
  }

  const handleRun = () => {
    if (running || !data) return
    setRunning(true)
    setStatus('Running')
    setProgress(0)

    fetch('/api/run', { method: 'POST' })
      .then(r => r.json() as Promise<{ error?: string }>)
      .then(res => {
        if (res.error) {
          setStatus('Failed')
          setErrorDialog({ title: 'Run Failed', message: res.error })
        } else {
          setStatus('Success')
        }
        setRunning(false)
        setProgress(100)
      })
      .catch((e) => {
        setStatus('Failed')
        setRunning(false)
        setErrorDialog({ title: 'Run Failed', message: `Run request failed: ${e instanceof Error ? e.message : String(e)}` })
      })
  }

  const doRunCheck = async () => {
    if (!data) return
    setStatus('Checking...')

    try {
      const r = await fetch('/api/service')
      const svc: { version_match?: boolean } = await r.json()

      if (svc.version_match) {
        handleRun()
      } else {
        setInstallDialog(true)
      }
    } catch {
      setInstallDialog(true)
      setPasswordError(false)
    }
  }

  const handleRunTool = (name: string) => {
    fetch(`/api/run-tool/${encodeURIComponent(name)}`, { method: 'POST' })
      .catch(() => {})
  }

  // right-click handler
  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault()
    const items: CtxItem[] = [
      {
        label: 'Reload',
        action: () => {
          if (activeSourceName && sources.length > 0) {
            const s = sources.find(x => x.name === activeSourceName)
            if (s) fetchSource(s.url, s.name)
          } else {
            window.location.reload()
          }
        },
      },
      { divider: true, label: '', action: () => {} },
      { label: 'Home', action: () => setNavPage('home') },
      { label: 'Downloads', action: () => setNavPage('downloads') },
      { label: 'Announcement', action: () => setNavPage('announcement') },
    ]

    if (sources.length > 0) {
      items.push({ divider: true, label: '', action: () => {} })
      sources.forEach(s => {
        items.push({
          label: s.name,
          action: () => fetchSource(s.url, s.name),
        })
      })
    }

    setCtxMenu({ x: e.clientX, y: e.clientY, items })
  }

  return (
    <div onContextMenu={handleContextMenu}>
      <TopBar
        appName={data?.ServiceName || 'OpenHub'}
        onSettingsOpen={() => setSettingsOpen(true)}
        sources={sources}
        activeSource={activeSourceName}
        onSelectSource={fetchSource}
        loading={loadingSource}
      />

      <div className="dashboard">
        <Sidebar active={navPage} onNavigate={setNavPage} />

        <div className="content">
          {navPage === 'announcement' ? (
            <AnnouncementPage />
          ) : navPage === 'downloads' ? (
            <DownloadsPage
              sources={sources}
              activeSource={activeSourceName}
              onSelectSource={fetchSource}
            />
          ) : (
            <>
              <div className="content-top">
                <Header
                  logoUrl={settings.logoUrl || data?.ServiceLogo?.value}
                  bannerUrl={data?.ServiceBanner?.value}
                  version={data?.ServiceVersion || localVer || '1.0.0'}
                  name={data?.ServiceName || 'OpenHub'}
                  description={data?.ServiceDescription || 'Select a service from the top-left dropdown'}
                  userType={data?.ServiceUserType?.value}
                  osSupport={data?.ServiceOSSupport?.value}
                />
                <ToolList
                  tools={data?.Tools || []}
                  onRunTool={handleRunTool}
                />
              </div>

              <div className="content-mid">
                <Gallery items={data?.ServiceScreenshotsVideosURLs?.value} />
                <PreviewCard items={data?.ServiceScreenshotsVideosURLs?.value} />
              </div>

              <div className="bottom-bar">
                <ProgressBar value={progress} />
                <div className="bottom-row">
                  <StatusCard status={status} />
                  <RunButton onClick={doRunCheck} loading={running || loadingSource} />
                </div>
              </div>
            </>
          )}
        </div>
      </div>

      {showSetup && <SetupWizard onComplete={handleSetupComplete} />}

      {settingsOpen && navPage !== 'settings' && (
        <SettingsDialog
          settings={settings}
          onUpdate={updateSettings}
          onClose={() => setSettingsOpen(false)}
        />
      )}

      {installDialog && data && (
        <InstallDialog
          defaultPath={`{build}/services/${data.ServiceName}/service`}
          onInstall={handleInstall}
          onCancel={() => { setInstallDialog(false); setInstalling(false); setPasswordError(false) }}
          installing={installing}
          progress={installProgress}
          step={installStep}
          passwordError={passwordError}
        />
      )}

      {errorDialog && (
        <div className="error-overlay" onClick={() => setErrorDialog(null)}>
          <div className="error-modal" onClick={e => e.stopPropagation()}>
            <h2 className="error-title">{errorDialog.title}</h2>
            <div className="error-body">
              <pre className="error-message">{errorDialog.message}</pre>
            </div>
            <div className="error-footer">
              <button className="sm-btn primary" onClick={() => setErrorDialog(null)}>OK</button>
            </div>
          </div>
        </div>
      )}

      {ctxMenu && (
        <ContextMenu
          x={ctxMenu.x}
          y={ctxMenu.y}
          items={ctxMenu.items}
          onClose={() => setCtxMenu(null)}
        />
      )}
    </div>
  )
}

function AnnouncementPage() {
  return (
    <div className="announcement-page">
      <h2>Get support or contact us</h2>
      <p className="ap-desc">
        Join our Discord server for support, updates, and community discussions:
      </p>
      <a
        className="ap-discord"
        href="https://discord.gg/mJZwjqSCtJ"
        target="_blank"
        rel="noopener noreferrer"
      >
        <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
          <path d="M20.317 4.37a19.8 19.8 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.5 18.5 0 0 0-5.487 0 12 12 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.7 19.7 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14 14 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13 13 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10 10 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12 12 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.8 19.8 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.06.06 0 0 0-.031-.03M8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419s.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418m7.975 0c-1.183 0-2.157-1.085-2.157-2.419s.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418"/>
        </svg>
        <span>Join the Discord Server</span>
      </a>
      <p className="ap-footer">
        Or else we will add.
      </p>
    </div>
  )
}

function DownloadsPage({
  sources,
  activeSource,
  onSelectSource,
}: {
  sources: Source[]
  activeSource: string | null
  onSelectSource: (url: string, name: string) => void
}) {
  return (
    <div className="downloads-page">
      <h2>Service Sources</h2>
      <div className="sg-desc">Available service manifests. Select one to load.</div>

      <div className="ds-list">
        {sources.map(s => (
          <div
            key={s.name}
            className={`ds-item ${activeSource === s.name ? 'active' : ''}`}
            onClick={() => onSelectSource(s.url, s.name)}
          >
            <div className="ds-item-info">
              <span className="ds-name">{s.name}</span>
              <span className="ds-url">{s.url}</span>
            </div>
            <span className="ds-badge">{activeSource === s.name ? 'Active' : ''}</span>
          </div>
        ))}
      </div>
    </div>
  )
}
