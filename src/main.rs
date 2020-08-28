use anyhow::{Context, Result};
use log::*;
use pulldown_cmark as cmark;
use serde::Deserialize;
use std::fmt::Display;
use std::fs::{copy, create_dir_all, remove_dir_all, remove_file, write};
use std::path::{Path, PathBuf};

mod frontmatter;
mod input;
mod layout;

use input::SourcePage;

fn main() -> Result<()> {
    env_logger::init();

    let site_dir = input::SiteDirectory::new("site");
    let out_dir = OutputDirectory::new("dist");
    out_dir.clear()?;

    let pages: Vec<Page> = site_dir
        .pages()
        .filter_map(log_error)
        .map(read_page)
        .filter_map(log_error)
        .collect();

    for page in pages
        .iter()
        .map(|page| render_layout(&pages, page))
        .filter_map(log_error)
    {
        if let Err(err) = out_dir.write(&page) {
            error!("{}", err);
        }
    }

    Ok(())
}

pub struct OutputDirectory {
    path: PathBuf,
}

#[derive(Debug)]
pub struct RenderedPage {
    route: PathBuf,
    content: RenderedContent,
}

#[derive(Debug)]
pub enum RenderedContent {
    String(String),
    File(PathBuf),
}

#[derive(Debug)]
pub struct Page {
    route: PathBuf,
    collection: Option<String>,
    layout: Option<String>,
    content: Content,
}

#[derive(Debug)]
pub enum Content {
    Html(String),
    File(PathBuf),
}

fn render_layout(_pages: &[Page], page: &Page) -> Result<RenderedPage> {
    use layout::*;
    use render::html;

    Ok(RenderedPage {
        route: page.route.clone(),
        content: match &page.content {
            Content::File(p) => RenderedContent::File(p.clone()),
            Content::Html(c) => RenderedContent::String(html! {
              <Layout title={"Disconnected Systems"}>
                <h1>{"Hello"}</h1>
                {c.clone()}
              </Layout>
            }),
        },
    })
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

pub fn read_page(source: SourcePage) -> Result<Page> {
    #[derive(Deserialize)]
    struct Frontmatter {
        layout: Option<String>,
        collection: Option<String>,
    }

    match source.source.extension().unwrap_or_default() {
        ext if ext == "md" || ext == "html" || ext == "htm" => {
            let (frontmatter, content): (Option<Frontmatter>, String) =
                frontmatter::parse_file(&source.source)?;
            let frontmatter = frontmatter.unwrap_or(Frontmatter {
                layout: None,
                collection: None,
            });
            Ok(Page {
                route: normalise_html_route(source.route),
                collection: frontmatter.collection,
                layout: frontmatter.layout,
                content: {
                    if ext == "md" {
                        let parser = cmark::Parser::new_ext(&content, cmark::Options::all());
                        let mut html = String::new();
                        cmark::html::push_html(&mut html, parser);
                        Content::Html(html)
                    } else {
                        Content::Html(content)
                    }
                },
            })
        }
        _ => Ok(Page {
            route: source.route,
            collection: None,
            layout: None,
            content: Content::File(source.source),
        }),
    }
}

pub fn normalise_html_route(route: impl AsRef<Path>) -> PathBuf {
    let route = route.as_ref().with_extension("");
    match route.file_name() {
        Some(f) if f == "index" => route.with_extension("html"),
        _ => route.join("index.html"),
    }
}

impl OutputDirectory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn clear(&self) -> Result<()> {
        create_dir_all(&self.path).context("failed to create output directory")?;
        for entry in self.path.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                remove_dir_all(path)
            } else {
                remove_file(path)
            }
            .context("failed to clear output directory")?
        }
        Ok(())
    }

    pub fn write(&self, page: &RenderedPage) -> Result<()> {
        let dest = self.path.join(&page.route);
        if let Some(parent) = dest.parent() {
            create_dir_all(parent)?
        }

        match &page.content {
            RenderedContent::String(content) => write(dest, content)?,
            RenderedContent::File(source) => {
                copy(source, dest)?;
            }
        }
        Ok(())
    }
}
