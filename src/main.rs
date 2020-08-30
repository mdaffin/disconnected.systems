use anyhow::Result;
use log::*;
use std::fmt::Display;

mod layout;
mod ssg;

use ssg::{OutputDirectory, Page, SiteDirectory, SourcePage};

fn main() -> Result<()> {
    env_logger::init();

    let site_dir = SiteDirectory::new("site");
    let out_dir = OutputDirectory::new("dist");
    out_dir.clear()?;

    let pages: Vec<Page> = site_dir
        .pages()
        .filter_map(log_error)
        .map(SourcePage::load)
        .filter_map(log_error)
        .map(|page| {
            info!("Loaded page {}", page.path().display());
            page
        })
        .collect();

    pages
        .iter()
        .map(|page| page.render_layout(&pages, layout::render))
        .filter_map(log_error)
        .for_each(|page| {
            if let Err(err) = out_dir.write(&page) {
                error!("{}", err);
            }
        });

    Ok(())
}

fn log_error<T, R: Display>(result: std::result::Result<T, R>) -> Option<T> {
    match result {
        Err(err) => {
            error!("{}", err);
            None
        }
        Ok(v) => Some(v),
    }
}
