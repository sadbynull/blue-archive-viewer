use super::api::jp::AddressableCatalog;
use crate::{api::en::catalog::Catalog, info};
use anyhow::Result;

pub async fn run_jp(catalog: &AddressableCatalog) -> Result<()> {
    info!("Downloading CGs");
    let media_catalog = catalog.get_media_catalog().await?;
    media_catalog
        .save_media("./public/data/jp", |media| media.path.contains(".jpg"))
        .await?;
    info!("Finished downloading CGs");
    Ok(())
}

pub async fn run_en(catalog: &Catalog) -> Result<()> {
    info!("Downloading CGs");
    catalog
        .save_resource("./public/data/en", |r| r.resource_path.ends_with(".jpg"))
        .await?;
    info!("Finished downloading CGs");
    Ok(())
}
