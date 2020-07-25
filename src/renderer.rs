use crate::input::SourcePage;
use anyhow::{Context, Result};
use std::fs::read;
use std::path::PathBuf;

#[derive(Debug)]
pub struct RenderedPage {
    pub route: PathBuf,
    pub content: Vec<u8>,
}

pub fn render(page: SourcePage) -> Result<RenderedPage> {
    Ok(RenderedPage {
        route: page.route,
        content: read(&page.source).context(format!(
            "could not read source file: {}",
            page.source.display()
        ))?,
    })
}

#[cfg(test)]
mod tests {
    use super::{render, SourcePage};
    use std::fs::write;
    use std::path::PathBuf;
    use test_case::test_case;

    #[test_case("css" ; "css files")]
    #[test_case("js" ; "js files")]
    #[test_case("html" ; "html files")]
    #[test_case("png" ; "png files")]
    #[test_case("jpeg" ; "jpeg files")]
    #[test_case("jpg" ; "jpg files")]
    fn content_is_unmodified_when_rendering(extension: &str) {
        let temp_dir = tempfile::tempdir().unwrap();

        let in_page = SourcePage {
            source: temp_dir.path().join(format!("main.{}", extension)),
            route: PathBuf::from(""),
        };

        let content = b"content";
        write(&in_page.source, content).unwrap();

        assert_eq!(render(in_page).unwrap().content, content);
    }
}
