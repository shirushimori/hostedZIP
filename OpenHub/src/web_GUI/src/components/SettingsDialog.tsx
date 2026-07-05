import { useState } from 'react'
import type { UserSettings } from '../theme'
import { themes, getTheme } from '../theme'
import './SettingsDialog.css'

interface SettingsDialogProps {
  settings: UserSettings
  onUpdate: (s: UserSettings) => void
  onClose: () => void
  inline?: boolean
}

const tabs = ['General', 'Appearance', 'Downloads', 'Advanced']

export function SettingsDialog({ settings, onUpdate, onClose, inline }: SettingsDialogProps) {
  const [tab, setTab] = useState('General')
  const [draft, setDraft] = useState<UserSettings>({ ...settings })

  const set = (key: keyof UserSettings, val: any) => setDraft({ ...draft, [key]: val })

  const handleSave = () => {
    onUpdate(draft)
    if (!inline) onClose()
  }

  const currentTheme = getTheme(draft.themeId)

  const body = (
    <>
      <div className="sm-tabs">
        {tabs.map(t => (
          <button key={t} className={`sm-tab ${tab === t ? 'active' : ''}`} onClick={() => setTab(t)}>
            {t}
          </button>
        ))}
      </div>
      <div className="sm-body">
        {tab === 'General' && (
          <div className="sm-section">
            <SettingRow label="Dark Theme">
              <Toggle checked={draft.themeId !== 'frost'} onChange={v => set('themeId', v ? 'green' : 'frost')} />
            </SettingRow>
            <SettingRow label="Launch on Startup">
              <Toggle checked={draft.autoStart} onChange={v => set('autoStart', v)} />
            </SettingRow>
            <SettingRow label="Minimize to Tray">
              <Toggle checked={draft.minimizeToTray} onChange={v => set('minimizeToTray', v)} />
            </SettingRow>
            <SettingRow label="Discord Rich Presence">
              <Toggle checked={draft.discordRpc} onChange={v => set('discordRpc', v)} />
            </SettingRow>
          </div>
        )}

        {tab === 'Appearance' && (
          <div className="sm-section">
            <SettingRow label="Accent Color">
              <div className="accent-picker">
                {themes.map(t => (
                  <button
                    key={t.id}
                    className={`accent-swatch ${currentTheme.id === t.id ? 'active' : ''}`}
                    style={{ background: t.colors.accent }}
                    onClick={() => set('themeId', t.id)}
                    title={t.name}
                  />
                ))}
              </div>
            </SettingRow>

            <div className="theme-preview-box" style={{
              background: currentTheme.colors.bg,
              borderRadius: '12px',
              padding: '20px',
              display: 'flex',
              flexDirection: 'column',
              gap: '12px',
              marginTop: '4px',
            }}>
              <div style={{ display: 'flex', gap: '12px', alignItems: 'center' }}>
                <div style={{ width: 40, height: 40, borderRadius: 8, background: currentTheme.colors.bg4 }} />
                <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: 4 }}>
                  <div style={{ height: 8, width: '60%', background: currentTheme.colors.text2, borderRadius: 4, opacity: 0.6 }} />
                  <div style={{ height: 6, width: '40%', background: currentTheme.colors.text3, borderRadius: 4 }} />
                </div>
              </div>
              <div style={{
                height: 100, borderRadius: 8, background: currentTheme.colors.bg3,
                border: `1px solid ${currentTheme.colors.border}`,
                display: 'flex', alignItems: 'center', justifyContent: 'center',
                color: currentTheme.colors.text3, fontSize: 12, gap: 8,
              }}>
                <span style={{ width: 8, height: 8, borderRadius: 2, background: currentTheme.colors.accent }} />
                <span style={{ width: 8, height: 8, borderRadius: 2, background: currentTheme.colors.text3, opacity: 0.5 }} />
                <span style={{ width: 8, height: 8, borderRadius: 2, background: currentTheme.colors.text3, opacity: 0.3 }} />
              </div>
              <div style={{ display: 'flex', gap: 6 }}>
                <div style={{ flex: 1, height: 4, background: currentTheme.colors.accent, borderRadius: 2 }} />
                <div style={{ flex: 3, height: 4, background: currentTheme.colors.bg4, borderRadius: 2 }} />
              </div>
            </div>

            <SettingRow label="Window Transparency">
              <input type="range" min="0" max="100" step="5" value={draft.transparency}
                onChange={e => set('transparency', Number(e.target.value))} className="slider" />
            </SettingRow>
            <SettingRow label="UI Scale">
              <input type="range" min="80" max="150" step="5" value={draft.uiScale}
                onChange={e => set('uiScale', Number(e.target.value))} className="slider" />
            </SettingRow>
            <SettingRow label="Animations">
              <Toggle checked={draft.animation} onChange={v => set('animation', v)} />
            </SettingRow>
          </div>
        )}

        {tab === 'Downloads' && (
          <div className="sm-section">
            <SettingRow label="Default Folder">
              <input type="text" value={draft.installPath}
                onChange={e => set('installPath', e.target.value)}
                className="sm-input" placeholder="/home/Downloads" />
            </SettingRow>
            <SettingRow label="Auto Update">
              <Toggle checked={draft.autoUpdate} onChange={v => set('autoUpdate', v)} />
            </SettingRow>
            <SettingRow label="Clear Cache">
              <button className="sm-btn" onClick={() => alert('Cache cleared')}>Clear</button>
            </SettingRow>
          </div>
        )}

        {tab === 'Advanced' && (
          <div className="sm-section">
            <SettingRow label="Developer Mode">
              <Toggle checked={false} onChange={() => {}} />
            </SettingRow>
            <SettingRow label="Verbose Logs">
              <Toggle checked={false} onChange={() => {}} />
            </SettingRow>
            <SettingRow label="Open Log Folder">
              <button className="sm-btn" onClick={() => alert('Opening logs...')}>Open</button>
            </SettingRow>
            <SettingRow label="Reset Settings">
              <button className="sm-btn danger" onClick={() => {
                if (confirm('Reset all settings to defaults?')) {
                  setDraft({ themeId: 'green', logoUrl: '', installPath: '',
                    autoUpdate: true, autoStart: false, minimizeToTray: false,
                    discordRpc: false, animation: true, transparency: 0, uiScale: 100 })
                }
              }}>Reset</button>
            </SettingRow>
          </div>
        )}
      </div>
    </>
  )

  if (inline) {
    return (
      <div className="sm-inline">
        <div className="sm-header"><h2>Settings</h2></div>
        {body}
        <div className="sm-footer">
          <button className="sm-btn primary" onClick={handleSave}>Save</button>
        </div>
      </div>
    )
  }

  return (
    <div className="settings-overlay" onClick={onClose}>
      <div className="settings-modal" onClick={e => e.stopPropagation()}>
        <div className="sm-header">
          <h2>Settings</h2>
          <button className="sm-close" onClick={onClose}>
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M18 6L6 18M6 6l12 12"/>
            </svg>
          </button>
        </div>
        {body}
        <div className="sm-footer">
          <button className="sm-btn" onClick={onClose}>Cancel</button>
          <button className="sm-btn primary" onClick={handleSave}>Save</button>
        </div>
      </div>
    </div>
  )
}

function Toggle({ checked, onChange }: { checked: boolean; onChange: (v: boolean) => void }) {
  return (
    <label className="toggle">
      <input type="checkbox" checked={checked} onChange={e => onChange(e.target.checked)} />
      <span className="toggle-track" />
    </label>
  )
}

function SettingRow({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div className="setting-row">
      <span className="setting-label">{label}</span>
      <div className="setting-control">{children}</div>
    </div>
  )
}
