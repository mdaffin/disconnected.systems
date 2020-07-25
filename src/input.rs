use crate::renderer::SourcePage;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct SiteDirectory {
    path: PathBuf,
}

pub struct SiteIter<'site> {
    site: &'site SiteDirectory,
    walkdir: walkdir::IntoIter,
}

impl SiteDirectory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn pages<'site>(&'site self) -> SiteIter<'site> {
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
                        path: entry
                            .path()
                            .strip_prefix(&self.site.path)
                            .expect("root path did not match of file and site did not match")
                            .to_path_buf(),
                    }))
                }
                Some(Err(e)) => Some(Err(e)),
                None => None,
            };
        }
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
    fn iterating_over_a_directory_produces_source_pages_with_paths_relative_to_the_site_dir(
        file_path: &str,
    ) {
        let temp_dir = tempfile::tempdir().unwrap();
        let full_path = temp_dir.path().join(file_path);
        std::fs::create_dir_all(full_path.parent().unwrap()).unwrap();
        std::fs::write(full_path, b"content").unwrap();
        let site = SiteDirectory::new(temp_dir.path());

        assert_eq!(
            &site
                .pages()
                .next()
                .expect("no pages found")
                .expect("error searching for file")
                .path,
            Path::new(file_path),
        );
    }
}
