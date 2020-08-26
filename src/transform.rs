use crate::frontmatter;
use anyhow::Result;
use serde::Deserialize;
use std::fs::{create_dir_all, read, write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SourcePage {
    pub source: PathBuf,
    pub route: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Content {
    /// Html content that can be wrapped by a layout, need further processing before it can be
    /// written.
    Html(HtmlContent),
    /// Raw content that is ready to be written.
    Raw(WritableContent),
}

#[derive(Debug, Clone)]
pub struct HtmlContent {
    route: PathBuf,
    collection: Option<String>,
    layout: Option<String>,
    content: String,
}

#[derive(Debug, Clone)]
pub struct WritableContent {
    route: PathBuf,
    content: Vec<u8>,
}

impl SourcePage {
    pub fn read(self) -> Result<Content> {
        #[derive(Deserialize)]
        struct Frontmatter {
            layout: Option<String>,
            collection: Option<String>,
        }

        Ok(match self.source.extension() {
            Some(extension) if extension == "html" || extension == "md" => {
                let (frontmatter, content): (Option<Frontmatter>, String) =
                    frontmatter::parse_file(&self.source)?;
                let frontmatter = frontmatter.unwrap_or(Frontmatter {
                    layout: None,
                    collection: None,
                });
                Content::Html(HtmlContent {
                    route: self.route.with_extension("html").into(),
                    collection: frontmatter.collection,
                    layout: frontmatter.layout,
                    content,
                })
            }
            _ => Content::Raw(WritableContent::new(self.route, read(self.source)?)),
        })
    }
}

impl WritableContent {
    pub fn new(route: PathBuf, content: Vec<u8>) -> Self {
        Self { route, content }
    }

    pub fn write(&self, out_dir: &Path) -> std::io::Result<()> {
        let path = out_dir.join(&self.route);
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        write(path, &self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{read, write};
    use std::path::PathBuf;
    use test_case::test_case;

    #[test_case("css" ; "css files")]
    #[test_case("js" ; "js files")]
    #[test_case("png" ; "png files")]
    #[test_case("jpeg" ; "jpeg files")]
    #[test_case("jpg" ; "jpg files")]
    fn content_is_unmodified_when_source_is_read_and_produces_raw_content(extension: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let content = b"content";

        let page = SourcePage {
            source: temp_dir.path().join(format!("main.{}", extension)),
            route: PathBuf::from(""),
        };
        write(&page.source, content).unwrap();

        match page.read().unwrap() {
            Content::Html(_) => panic!("html content found"),
            Content::Raw(r) => assert_eq!(r.content, content),
        }
    }

    #[test_case("main.jpg" ; "jpg file")]
    #[test_case("section/main.jpg" ; "jpg file in directory")]
    #[test_case("section/subsection/main.jpg" ; "jpg file in nested directory")]
    fn writing_content_that_is_a_file_writes_to_the_route_inside_the_given_dir(route: &str) {
        let temp_dir = tempfile::tempdir().unwrap();

        let page = WritableContent {
            route: route.into(),
            content: b"content".to_vec(),
        };

        page.write(temp_dir.path()).unwrap();

        let dest_path = temp_dir.path().join(page.route);
        assert_eq!(read(dest_path).unwrap(), page.content);
    }

    #[test_case("html" ; "html files")]
    #[test_case("md" ; "markdown files")]
    fn content_is_unmodified_when_source_is_read_and_produces_html_content(extension: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let content = "<h1>content</h1>";

        let page = SourcePage {
            source: temp_dir.path().join(format!("main.{}", extension)),
            route: PathBuf::from(""),
        };
        write(&page.source, content).unwrap();

        match page.read().unwrap() {
            Content::Html(html) => assert_eq!(html.content, content),
            Content::Raw(_) => panic!("raw content returned"),
        }
    }

    #[test_case("main.css" ; "css file")]
    #[test_case("section/main.css" ; "css file in directory")]
    #[test_case("section/subsection/main.css" ; "css file in nested directory")]
    #[test_case("main.jpg" ; "jpg file")]
    #[test_case("section/main.jpg" ; "jpg file in directory")]
    #[test_case("section/subsection/main.jpg" ; "jpg file in nested directory")]
    #[test_case("main.html" ; "html file")]
    #[test_case("section/main.html" ; "html file in directory")]
    #[test_case("section/subsection/main.html" ; "html file in nested directory")]
    fn route_from_sorce_is_unmodified_when_converted_to_content(route: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let content = b"content";
        let route = PathBuf::from(route);

        let page = SourcePage {
            source: temp_dir.path().join(route.file_name().unwrap()),
            route: route.clone(),
        };
        write(&page.source, content).unwrap();

        assert_eq!(
            match page.read().unwrap() {
                Content::Html(html) => html.route,
                Content::Raw(raw) => raw.route,
            },
            route
        );
    }

    #[test_case( r#"---
layout: "some-layout"
---
content
"#; "yaml file")]
    #[test_case( r#"+++
layout = "some-layout"
+++
content
"#; "toml file")]
    #[test_case( r#"{ "layout": "some-layout" }
content
"#; "json file")]
    fn layout_is_loaded_from_frontmatter(content: &str) {
        for route in &["main.html", "main.md"] {
            let temp_dir = tempfile::tempdir().unwrap();

            let page = SourcePage {
                source: temp_dir.path().join(route),
                route: PathBuf::from(route),
            };
            write(&page.source, content).unwrap();

            match page.read().unwrap() {
                Content::Html(html) => assert_eq!(html.layout, Some("some-layout".to_string())),
                Content::Raw(_) => panic!("raw content returned"),
            }
        }
    }
}
