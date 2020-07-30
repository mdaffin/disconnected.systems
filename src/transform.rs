use anyhow::Result;
use std::fs::{create_dir_all, read, read_to_string, write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SourcePage {
    pub source: PathBuf,
    pub route: PathBuf,
}

#[derive(Debug)]
pub enum Content {
    /// Html content that can be wrapped by a layout, need further processing before it can be
    /// written.
    Html(HtmlContent),
    /// Raw content that is ready to be written.
    Raw(WritableContent),
}

#[derive(Debug)]
pub struct HtmlContent {
    route: PathBuf,
    collection: Option<String>,
    layout: Option<String>,
    content: String,
}

#[derive(Debug)]
pub struct WritableContent {
    route: PathBuf,
    content: Vec<u8>,
}

impl SourcePage {
    pub fn read(self) -> Result<Content> {
        Ok(match self.source.extension() {
            Some(extension) if extension == "html" || extension == "md" => {
                Content::Html(HtmlContent {
                    route: self.route,
                    collection: None,
                    layout: None,
                    content: read_to_string(self.source)?,
                })
            }
            _ => Content::Raw(WritableContent {
                route: self.route,
                content: read(self.source)?,
            }),
        })
    }
}

impl WritableContent {
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
    use std::convert::TryInto;
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
    #[test_case("main.md" ; "md file")]
    #[test_case("section/main.md" ; "md file in directory")]
    #[test_case("section/subsection/main.md" ; "md file in nested directory")]
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
}
