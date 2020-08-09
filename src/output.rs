use crate::transform::WritableContent;
use std::fs::{create_dir_all, remove_dir_all, remove_file};
use std::path::PathBuf;

use anyhow::{Context, Result};

pub struct OutputDirectory {
    path: PathBuf,
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

    pub fn write(&self, content: &WritableContent) -> Result<()> {
        Ok(content.write(&self.path)?)
    }
}

#[cfg(test)]
mod tests {
    use super::OutputDirectory;
    use crate::transform::WritableContent;
    use std::fs::read;
    use std::path::PathBuf;
    use test_case::test_case;

    #[test]
    fn when_the_output_directory_does_not_exist_clear_should_create_it() {
        let temp_dir = tempfile::tempdir().unwrap();
        let out_dir = OutputDirectory::new(temp_dir.path().join("out_dir"));

        out_dir.clear().unwrap();

        assert!(out_dir.path.exists());
        assert!(out_dir.path.is_dir());
    }

    #[test_case("main.css" ; "site root with css")]
    #[test_case("section/main.css" ; "single directory with css")]
    #[test_case("section/subsection/main.css" ; "nested directory with css")]
    #[test_case("section/subsection/subsecion/subsection/main.css" ; "deeply nested directory with css")]
    #[test_case("main.js" ; "site root with js")]
    #[test_case("section/main.js" ; "single directory with js")]
    #[test_case("section/subsection/main.js" ; "nested directory with js")]
    #[test_case("section/subsection/subsecion/subsection/main.js" ; "deeply nested directory with js")]
    fn writing_a_page_with_an_extension_path_writes_directly_to_that_path(route: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let out_path = temp_dir.path().join("out_dir");
        let out_dir = OutputDirectory::new(out_path.clone());
        let content = "content";
        let page = rendered_page(route, content);

        out_dir.write(&page).unwrap();

        assert_eq!(
            read(out_path.join(route)).unwrap().as_slice(),
            content.as_bytes()
        );
    }

    #[test]
    fn clean_removes_all_files_and_directories_that_have_been_written() {
        let temp_dir = tempfile::tempdir().unwrap();
        let out_dir = OutputDirectory::new(temp_dir.path().join("out_dir"));

        out_dir
            .write(&rendered_page("index.html", "content"))
            .unwrap();
        out_dir.write(&rendered_page("page1", "content")).unwrap();
        out_dir.write(&rendered_page("page2", "content")).unwrap();
        out_dir
            .write(&rendered_page("page3/subpage", "content"))
            .unwrap();

        out_dir.clear().unwrap();

        assert!(
            out_dir
                .path
                .read_dir()
                .expect("could not read output directory")
                .next()
                .is_none(),
            "output_directory is not empty"
        );
    }

    fn rendered_page(route: impl Into<PathBuf>, content: impl Into<Vec<u8>>) -> WritableContent {
        WritableContent::new(route.into(), content.into())
    }
}
