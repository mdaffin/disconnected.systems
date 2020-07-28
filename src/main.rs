use anyhow::Result;

mod frontmatter;
mod input;
mod output;
mod renderer;
mod transform;

use renderer::render;

fn main() -> Result<()> {
    let site_dir = input::SiteDirectory::new("site");
    let out_dir = output::OutputDirectory::new("dist");

    out_dir.clear()?;

    for page in site_dir.pages() {
        out_dir.write(&render(page?)?)?;
    }

    Ok(())
}
