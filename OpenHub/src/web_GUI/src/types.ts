export interface SrvResponse {
  data: ServiceData | null
  installed: boolean
  version_match: boolean
  local_version: string
  remote_version: string
  error: string | null
  download_size?: string | null
}

export interface ServiceData {
  ServiceName: string
  ServiceVersion: string
  ServiceDescription: string
  ServiceOnlineVersionURL: string
  ServiceDownloadZipSourceURL: string
  ServiceDefaultExtractPath: string
  ServiceOSSupport: Field<string[]>
  ServiceLogo: Field<string>
  ServiceBanner: Field<string>
  ServiceScreenshotsVideosURLs: Field<string[]>
  ServiceUserType: Field<string>
  Tools: ToolEntry[]
}

export interface Field<T> {
  value: T
  tooltip: string
}

export interface ToolEntry {
  displayName: string
  path: string
  tooltip: string
}

export interface Source {
  name: string
  url: string
}

export interface InstallOptions {
  extractPath: string
  createStartMenuShortcut: boolean
  createDesktopShortcut: boolean
}

export interface ProgressResponse {
  step: string
  progress: number
  error: string | null
  done: boolean
}
