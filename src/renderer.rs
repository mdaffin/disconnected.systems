use anyhow::Result;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RenderedPage {
    pub route: PathBuf,
    pub content: String,
}

#[derive(Debug)]
pub struct SourcePage {
    pub path: PathBuf,
}

impl SourcePage {
    pub fn route(&self) -> &Path {
        self.path.as_path()
    }
}

pub fn render(page: SourcePage) -> Result<RenderedPage> {
    Ok(RenderedPage {
        route: page.route().into(),
        content: "content".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::SourcePage;
    use std::path::{Path, PathBuf};
    use test_case::test_case;

    #[test_case("css" ; "css files")]
    fn source_path_matches_its_route(extension: &str) {
        for path in &["main", "section/main", "section/subsection/main"] {
            let path = format!("{}.{}", path, extension);
            let page = SourcePage {
                path: PathBuf::from(&path),
            };
            assert_eq!(page.route(), Path::new(&path));
        }
    }
}
