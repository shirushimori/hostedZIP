import type { SrvResponse } from './types'

const API = ''

export async function getService(): Promise<SrvResponse> {
  const r = await fetch(`${API}/api/service`)
  return r.json()
}

export async function refresh(): Promise<SrvResponse> {
  const r = await fetch(`${API}/api/refresh`, { method: 'POST' })
  return r.json()
}

export async function install(password?: string): Promise<SrvResponse> {
  const r = await fetch(`${API}/api/install`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ password: password || null }),
  })
  return r.json()
}

export async function runService(): Promise<SrvResponse> {
  const r = await fetch(`${API}/api/run`, { method: 'POST' })
  return r.json()
}

export async function runTool(name: string): Promise<SrvResponse> {
  const r = await fetch(`${API}/api/run-tool/${encodeURIComponent(name)}`, { method: 'POST' })
  return r.json()
}
