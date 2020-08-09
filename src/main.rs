use anyhow::Result;

mod frontmatter;
mod input;
mod output;
mod transform;

use crate::transform::Content;

fn main() -> Result<()> {
    let site_dir = input::SiteDirectory::new("site");
    let out_dir = output::OutputDirectory::new("dist");

    out_dir.clear()?;

    let (_html, raw) = site_dir
        .pages()
        .map(|page| -> Result<_> { Ok(page?.read()?) })
        .fold(Ok((Vec::new(), Vec::new())), |acc: Result<_>, content| {
            let (mut html_vec, mut raw_vec) = acc?;
            match content? {
                Content::Html(html) => html_vec.push(html),
                Content::Raw(raw) => raw_vec.push(raw),
            };
            Ok((html_vec, raw_vec))
        })?;

    for content in raw {
        out_dir.write(&content)?;
    }

    Ok(())
}
