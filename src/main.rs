use std::fs::{create_dir_all, remove_dir_all, remove_file, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use render::html::HTML5Doctype;
use render::{component, html, rsx, Render};

#[component]
fn Layout<'a, Children: Render>(title: &'a str, children: Children) {
    rsx! {
      <>
        <HTML5Doctype />
        <html lang={"en"}>
            <head>
                <meta charset={"utf-8"} />
                <meta name={"viewport"} content={"width=device-width, initial-scale=1"} />
                <title>{title}</title>
                <link rel={"stylesheet"} href={"/css/normalize.css"} />
                <link rel={"stylesheet"} href={"/css/main.css"} />
            </head>
            <body>
                <header></header>
                <main>
                    {children}
                </main>
            </body>
        </html>
      </>
    }
}

pub fn index() -> RenderedPage {
    RenderedPage {
        route: "".to_string(),
        content: html! {
          <Layout title={"Disconnected Systems"}>
            <h1>{"Hello"}</h1>
            {"Welcome!"}
          </Layout>
        },
    }
}

fn main() -> Result<()> {
    let out_dir = OutputDirectory::new("dist");
    let _site_dir = Path::new("site");

    out_dir.clear()?;
    out_dir.write(&index())?;

    Ok(())
}

pub struct RenderedPage {
    route: String,
    content: String,
}

pub struct OutputDirectory {
    path: PathBuf,
}

impl OutputDirectory {
    fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn clear(&self) -> Result<()> {
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

    fn write(&self, page: &RenderedPage) -> Result<()> {
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

    mod output_directory {
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
            let page = RenderedPage {
                route: route.to_string(),
                content: "content".to_string(),
            };

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
            let page = RenderedPage {
                route: route.to_string(),
                content: "content".to_string(),
            };

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

            out_dir
                .write(&RenderedPage {
                    route: "".to_string(),
                    content: "content".to_string(),
                })
                .unwrap();
            out_dir
                .write(&RenderedPage {
                    route: "page1".to_string(),
                    content: "content".to_string(),
                })
                .unwrap();
            out_dir
                .write(&RenderedPage {
                    route: "page2".to_string(),
                    content: "content".to_string(),
                })
                .unwrap();
            out_dir
                .write(&RenderedPage {
                    route: "page3/subpage".to_string(),
                    content: "content".to_string(),
                })
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
}
