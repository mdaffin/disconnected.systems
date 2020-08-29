use anyhow::{Context, Result};
use pulldown_cmark as cmark;
use serde::Deserialize;
use std::fs::{copy, create_dir_all, remove_dir_all, remove_file, write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod frontmatter;
mod layout;

pub struct OutputDirectory {
    path: PathBuf,
}

#[derive(Debug)]
pub struct SiteDirectory {
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct SourcePage {
    source: PathBuf,
    route: PathBuf,
}

pub struct SiteIter<'site> {
    site: &'site SiteDirectory,
    walkdir: walkdir::IntoIter,
}

#[derive(Debug)]
pub struct Page {
    route: PathBuf,
    collection: Option<String>,
    layout: Option<String>,
    content: Content,
}

#[derive(Debug)]
enum Content {
    Html(String),
    File(PathBuf),
}

#[derive(Debug)]
pub struct RenderedPage {
    route: PathBuf,
    content: RenderedContent,
}

#[derive(Debug)]
enum RenderedContent {
    String(String),
    File(PathBuf),
}

impl SiteDirectory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn pages(&self) -> SiteIter<'_> {
        SiteIter {
            site: self,
            walkdir: WalkDir::new(&self.path).into_iter(),
        }
    }
}

impl<'site> Iterator for SiteIter<'site> {
    type Item = Result<SourcePage, walkdir::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.walkdir.next() {
                Some(Ok(entry)) => {
                    if entry.path().is_dir() {
                        continue;
                    }
                    Some(Ok(SourcePage {
                        source: entry.path().into(),
                        route: entry
                            .path()
                            .strip_prefix(&self.site.path)
                            .expect("root path did not match of file and site did not match")
                            .into(),
                    }))
                }
                Some(Err(e)) => Some(Err(e)),
                None => None,
            };
        }
    }
}

impl SourcePage {
    pub fn load(self) -> Result<Page> {
        #[derive(Deserialize)]
        struct Frontmatter {
            layout: Option<String>,
            collection: Option<String>,
        }

        fn normalise_html_route(route: impl AsRef<Path>) -> PathBuf {
            let route = route.as_ref().with_extension("");
            match route.file_name() {
                Some(f) if f == "index" => route.with_extension("html"),
                _ => route.join("index.html"),
            }
        }

        match self.source.extension().unwrap_or_default() {
            ext if ext == "md" || ext == "html" || ext == "htm" => {
                let (frontmatter, content): (Option<Frontmatter>, String) =
                    frontmatter::parse_file(&self.source)?;
                let frontmatter = frontmatter.unwrap_or(Frontmatter {
                    layout: None,
                    collection: None,
                });
                Ok(Page {
                    route: normalise_html_route(self.route),
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
                route: self.route,
                collection: None,
                layout: None,
                content: Content::File(self.source),
            }),
        }
    }
}

impl Page {
    pub fn render_layout(&self, _pages: &[Page]) -> Result<RenderedPage> {
        use layout::Layout;
        use render::html;

        Ok(RenderedPage {
            route: self.route.clone(),
            content: match &self.content {
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
}

impl RenderedPage {
    pub fn write(&self, dest_dir: impl AsRef<Path>) -> std::io::Result<()> {
        let dest = dest_dir.as_ref().join(&self.route);
        if let Some(parent) = dest.parent() {
            create_dir_all(parent)?
        }

        match &self.content {
            RenderedContent::String(content) => write(dest, content)?,
            RenderedContent::File(source) => {
                copy(source, dest)?;
            }
        }
        Ok(())
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

    pub fn write(&self, page: &RenderedPage) -> std::io::Result<()> {
        page.write(&self.path)
    }
}

#[cfg(test)]
mod tests {

    use super::SiteDirectory;
    use std::path::Path;
    use test_case::test_case;

    #[test]
    fn iterating_over_an_empty_directory_produces_no_pages() {
        let temp_dir = tempfile::tempdir().unwrap();
        let site = SiteDirectory::new(temp_dir.path());
        assert!(dbg!(site.pages().next()).is_none());
    }

    #[test_case("index.html" ; "html page in the root")]
    #[test_case("index.md" ; "markdown page in the root")]
    #[test_case("main.css" ; "css file in the root")]
    #[test_case("main.js" ; "javascript file in the root")]
    #[test_case("path/index.html" ; "html page in a directory")]
    #[test_case("path/index.md" ; "markdown page in a directory")]
    #[test_case("path/main.css" ; "css file in a directory")]
    #[test_case("path/main.js" ; "javascript file in a directory")]
    fn iterating_over_a_directory_produces_source_pages_with_source_path(file_path: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let full_path = temp_dir.path().join(file_path);
        std::fs::create_dir_all(full_path.parent().unwrap()).unwrap();
        std::fs::write(&full_path, b"content").unwrap();
        let site = SiteDirectory::new(temp_dir.path());

        assert_eq!(
            site.pages()
                .next()
                .expect("no pages found")
                .expect("error searching for file")
                .source,
            full_path,
        );
    }

    #[test_case("css" ; "css files")]
    #[test_case("js" ; "js files")]
    #[test_case("html" ; "html files")]
    #[test_case("png" ; "png files")]
    #[test_case("jpeg" ; "jpeg files")]
    #[test_case("jpg" ; "jpg files")]
    #[test_case("md" ; "markdown files")]
    fn route_is_unmodified_with(extension: &str) {
        for path in &["main", "section/main", "section/subsection/main"] {
            let temp_dir = tempfile::tempdir().unwrap();
            let route = format!("{}.{}", path, extension);
            let full_path = temp_dir.path().join(&route);

            std::fs::create_dir_all(full_path.parent().unwrap()).unwrap();
            std::fs::write(full_path, b"content").unwrap();
            let site = SiteDirectory::new(temp_dir.path());

            let page = site
                .pages()
                .next()
                .expect("no pages found")
                .expect("error searching for file");
            assert_eq!(&page.route, Path::new(&route));
        }
    }
}
