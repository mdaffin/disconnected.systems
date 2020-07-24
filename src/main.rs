use anyhow::Result;

use render::html::HTML5Doctype;
use render::{component, html, rsx, Render};

mod input;
mod output;

use input::SiteDirectory;
use output::{OutputDirectory, RenderedPage};

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
    RenderedPage::new(
        "",
        html! {
          <Layout title={"Disconnected Systems"}>
            <h1>{"Hello"}</h1>
            {"Welcome!"}
          </Layout>
        },
    )
}

fn main() -> Result<()> {
    let out_dir = OutputDirectory::new("dist");
    let site_dir = SiteDirectory::new("site");

    out_dir.clear()?;

    for page in site_dir.pages() {
        let page = page?;
        dbg!(page);
    }

    out_dir.write(&index())?;

    Ok(())
}
