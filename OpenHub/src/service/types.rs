use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceData {
    pub ServiceName: String,
    pub ServiceVersion: String,
    pub ServiceDescription: String,
    pub ServiceOnlineVersionURL: String,
    pub ServiceDownloadZipSourceURL: String,
    pub ServiceDefaultExtractPath: String,
    pub ServiceOSSupport: Field<Vec<String>>,
    pub ServiceLogo: Field<String>,
    pub ServiceBanner: Field<String>,
    pub ServiceScreenshotsVideosURLs: Field<Vec<String>>,
    pub ServiceUserType: Field<String>,
    pub Tools: Vec<ToolEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field<T> {
    pub value: T,
    pub tooltip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEntry {
    pub displayName: String,
    pub path: String,
    pub tooltip: String,
}
