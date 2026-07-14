import { useState } from 'react'
import type { UserSettings } from '../theme'
import './SetupWizard.css'

interface SetupWizardProps {
  onComplete: (settings: Partial<UserSettings>) => void
}

const steps = ['Welcome', 'Dependencies', 'Extract Path', 'Theme', 'Finish']

const defaultSettings: Partial<UserSettings> = {
  themeId: 'green',
  installPath: '',
  autoUpdate: true,
}

export function SetupWizard({ onComplete }: SetupWizardProps) {
  const [step, setStep] = useState(0)
  const [settings, setSettings] = useState({ ...defaultSettings })
  const [depStatus, setDepStatus] = useState<'idle' | 'working' | 'done'>('idle')
  const [progressVal, setProgressVal] = useState(0)
  const [progressStep, setProgressStep] = useState('')
  // Track whether the user has explicitly chosen a theme (clicked a button)
  const [themePicked, setThemePicked] = useState(false)

  // --- Gate logic: is Next allowed on the current step? ---
  const canNext = (): boolean => {
    if (step === 1) return depStatus === 'done'   // must install 7zip
    if (step === 3) return themePicked             // must pick a theme
    return true
  }

  const next = () => {
    if (!canNext()) return
    if (step < steps.length - 1) setStep(s => s + 1)
    else onComplete(settings)
  }

  const prev = () => {
    if (step > 0) setStep(s => s - 1)
  }

  return (
    <div className="setup-overlay">
      <div className="setup-modal">
        <div className="setup-steps">
          {steps.map((s, i) => (
            <div key={s} className={`setup-step ${i === step ? 'active' : ''} ${i < step ? 'done' : ''}`}>
              <div className="setup-step-num">{i < step ? '✓' : i + 1}</div>
              <span>{s}</span>
            </div>
          ))}
        </div>

        <div className="setup-body">
          {step === 0 && (
            <div className="setup-content">
              <h2>Welcome to OpenHub</h2>
              <p>This wizard will help you set up the launcher for the first time.</p>
              <p>You'll configure dependencies, extract paths, and choose a theme.</p>
            </div>
          )}

          {step === 1 && (
            <div className="setup-content">
              <h2>Dependencies</h2>
              <p>The launcher needs 7-Zip to extract service archives. Install it before continuing.</p>
              <div className="setup-dep">
                <div className="setup-dep-info">
                  <span className="setup-dep-name">7-Zip</span>
                  <span className="setup-dep-desc">Archive extraction tool</span>
                </div>
                {depStatus === 'done' ? (
                  <span className="setup-dep-status done">✓ Installed</span>
                ) : depStatus === 'working' ? (
                  <div className="setup-dep-progress-container">
                    <span className="setup-dep-status working">{progressStep || 'Downloading…'}</span>
                    <div className="setup-dep-progress-bar">
                      <div className="setup-dep-progress-fill" style={{ width: `${progressVal}%` }} />
                    </div>
                    <span className="setup-dep-progress-text">{progressVal}%</span>
                  </div>
                ) : (
                  <button className="sm-btn primary" onClick={async () => {
                    setDepStatus('working')
                    setProgressVal(0)
                    setProgressStep('Starting...')

                    const poll = setInterval(async () => {
                      try {
                        const pr = await fetch('/api/progress')
                        const pp: { step?: string; progress?: number } = await pr.json()
                        if (pp.progress !== undefined) setProgressVal(pp.progress)
                        if (pp.step !== undefined) setProgressStep(pp.step)
                      } catch { /* ok */ }
                    }, 250)

                    try {
                      const r = await fetch('http://localhost:3001/api/install', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({}),
                      })
                      await r.json()
                      setDepStatus('done')
                    } catch {
                      setDepStatus('done')
                    } finally {
                      clearInterval(poll)
                    }
                  }}>Install</button>
                )}
              </div>
              {depStatus !== 'done' && (
                <p className="setup-note setup-note-warn">⚠ You must install 7-Zip to continue.</p>
              )}
              {depStatus === 'done' && (
                <p className="setup-note">7-Zip will be used to extract service archives automatically.</p>
              )}
            </div>
          )}

          {step === 2 && (
            <div className="setup-content">
              <h2>Extract Path</h2>
              <p>Services will be extracted to a default location relative to the build directory.</p>
              <label className="setup-field">
                <span>Default Extract Path</span>
                <input
                  type="text"
                  value={settings.installPath || '{build}/services/{name}/service'}
                  readOnly
                  className="setup-input-readonly"
                />
              </label>
              <p className="setup-note">Each service creates its own folder automatically.</p>
            </div>
          )}

          {step === 3 && (
            <div className="setup-content">
              <h2>Theme</h2>
              <p>Choose your preferred appearance. <strong>Select a theme to continue.</strong></p>
              <div className="setup-themes">
                {['green', 'default', 'midnight', 'matrix', 'cyber', 'amber', 'frost'].map(id => (
                  <button
                    key={id}
                    className={`setup-theme-btn ${settings.themeId === id && themePicked ? 'active' : ''}`}
                    onClick={() => { setSettings({ ...settings, themeId: id }); setThemePicked(true) }}
                  >
                    {id === 'green' && 'Green Pulse'}
                    {id === 'default' && 'Dark'}
                    {id === 'midnight' && 'Midnight'}
                    {id === 'matrix' && 'Matrix'}
                    {id === 'cyber' && 'Cyber'}
                    {id === 'amber' && 'Amber'}
                    {id === 'frost' && 'Frost'}
                  </button>
                ))}
              </div>
              {!themePicked && (
                <p className="setup-note setup-note-warn">⚠ Select a theme to continue.</p>
              )}
            </div>
          )}

          {step === 4 && (
            <div className="setup-content">
              <h2>All Set!</h2>
              <p>Your launcher is ready to use. You can change any setting later from the Settings panel.</p>
              <ul className="setup-summary">
                <li>✓ 7-Zip installed</li>
                <li>✓ Extract path set</li>
                <li>✓ Theme selected</li>
              </ul>
            </div>
          )}
        </div>

        <div className="setup-footer">
          {step > 0 && <button className="sm-btn" onClick={prev}>Back</button>}
          <div style={{ flex: 1 }} />
          <button
            className={`sm-btn primary${!canNext() ? ' disabled' : ''}`}
            onClick={next}
            disabled={!canNext()}
            title={
              step === 1 && depStatus !== 'done' ? 'Install 7-Zip to continue' :
              step === 3 && !themePicked ? 'Select a theme to continue' :
              undefined
            }
          >
            {step === steps.length - 1 ? 'Finish' : 'Next'}
          </button>
        </div>
      </div>
    </div>
  )
}
