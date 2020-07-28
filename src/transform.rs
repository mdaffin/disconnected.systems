use anyhow::Result;
use std::fs::{create_dir_all, read, write};
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
    layout: String,
    content: String,
}

#[derive(Debug)]
pub struct WritableContent {
    route: PathBuf,
    content: Vec<u8>,
}

impl SourcePage {
    pub fn read(self) -> Result<Content> {
        Ok(Content::Raw(WritableContent {
            route: self.route,
            content: read(self.source)?,
        }))
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
    fn content_is_unmodified_when_source_is_read(extension: &str) {
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
}
