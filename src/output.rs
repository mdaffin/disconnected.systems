use std::fs::{create_dir_all, remove_dir_all, remove_file, File};
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::{Context, Result};

pub struct RenderedPage {
    route: String,
    content: String,
}

pub struct OutputDirectory {
    path: PathBuf,
}

impl RenderedPage {
    pub fn new(route: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            route: route.into(),
            content: content.into(),
        }
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
        let dest_file = self.path.join(&page.route);
        let dest_file = match dest_file.extension() {
            None => dest_file.join("index.html"),
            _ => dest_file,
        };

        create_dir_all(dest_file.parent().expect("missing parent directory"))
            .context("failed to create output directory")?;

        File::create(dest_file)
            .context("failed to create page")?
            .write_all(page.content.as_ref())
            .context("failed to write to page")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{OutputDirectory, RenderedPage};
    use std::fs::read_to_string;
    use test_case::test_case;

    #[test]
    fn when_the_output_directory_does_not_exist_clear_should_create_it() {
        let temp_dir = tempfile::tempdir().unwrap();
        let out_dir = OutputDirectory::new(temp_dir.path().join("out_dir"));

        out_dir.clear().unwrap();

        assert!(out_dir.path.exists());
        assert!(out_dir.path.is_dir());
    }

    #[test_case("" ; "site root")]
    #[test_case("page" ; "single directory")]
    #[test_case("section/page" ; "nested directory")]
    #[test_case("section/subsection/subsecion/subsection/page" ; "deeply nested directory")]
    fn writing_a_page_with_no_extension_path_writes_to_index_file_in(route: &str) {
        let temp_dir = tempfile::tempdir().unwrap();
        let out_path = temp_dir.path().join("out_dir");
        let out_dir = OutputDirectory::new(out_path.clone());
        let page = RenderedPage::new(route, "content");

        out_dir.write(&page).unwrap();

        assert_eq!(
            &read_to_string(out_path.join(route).join("index.html")).unwrap(),
            &page.content
        );
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
        let page = RenderedPage::new(route, "content");

        out_dir.write(&page).unwrap();

        assert_eq!(
            &read_to_string(out_path.join(route)).unwrap(),
            &page.content
        );
    }

    #[test]
    fn clean_removes_all_files_and_directories_that_have_been_written() {
        let temp_dir = tempfile::tempdir().unwrap();
        let out_dir = OutputDirectory::new(temp_dir.path().join("out_dir"));

        out_dir.write(&RenderedPage::new("", "content")).unwrap();
        out_dir
            .write(&RenderedPage::new("page1", "content"))
            .unwrap();
        out_dir
            .write(&RenderedPage::new("page2", "content"))
            .unwrap();
        out_dir
            .write(&RenderedPage::new("page3/subpage", "content"))
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
}
