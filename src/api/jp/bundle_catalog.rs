use crate::util;
use crate::util::save_json;
use anyhow::{Context, Result};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
use trauma::download::Download;
use trauma::downloader::{DownloaderBuilder, ProgressBarOpts, StyleOptions};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Bundle {
    pub name: String,
    pub size: i64,
    pub is_prologue: bool,
    pub crc: i64,
    pub is_split_download: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BundleCatalog {
    bundle_files: HashMap<String, Bundle>,

    #[serde(skip)]
    _base_url: String,
}

impl BundleCatalog {
    /// Save the bundle catalog to a file
    /// # Arguments
    /// * `path` - The path to save the bundle catalog to
    /// # Returns
    /// * `Result<()>` - The result of the operation
    /// # Errors
    /// * If the file cannot be saved
    /// # Example
    /// ```no_run
    /// let catalog = api::jp::get_addressable_catalog().await.unwrap();
    /// let bundle_catalog = catalog.get_bundle_catalog().await.unwrap();
    /// // ./test/MediaResources/MediaCatalog.json
    /// bundle_catalog.save(std::path::PathBuf::from("./test"));
    /// ```
    pub async fn save(&self, path: std::path::PathBuf) -> Result<()> {
        save_json(path.join("Android/bundleDownloadInfo.json"), self).await
    }
    pub async fn save_bundle(
        &self,
        path: std::path::PathBuf,
        filter: impl Fn(&Bundle) -> bool,
    ) -> Result<()> {
        let root_dir = path.join("Android");
        let downloader = DownloaderBuilder::new().directory(root_dir).build();
        let downloads = self
            .bundle_files
            .iter()
            .filter(|(_, v)| filter(v))
            .map(|(_, v)| Download {
                url: Url::parse(format!("{}/Android/{}", self._base_url, v.name).as_str()).unwrap(),
                filename: v.name.clone(),
            })
            .collect::<Vec<Download>>();
        downloader.download(&downloads).await;
        Ok(())
    }
}
