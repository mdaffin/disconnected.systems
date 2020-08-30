use crate::ssg::{HtmlPage, Page};
use render::{component, html, raw, rsx, Render};
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[component]
pub fn Layout<'a, Children: Render>(title: &'a str, children: Children) {
    rsx! {
      <>
        <html::HTML5Doctype />
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

pub fn render(page: &HtmlPage, pages: &[Page]) -> Result<String> {
    match &page.layout {
        Some(l) if l.to_lowercase() == "home" => home(page, pages),
        None => default(page, pages),
        Some(l) => Err(Error::UnknownLayout(page.path.clone(), l.clone())),
    }
}

pub fn home(page: &HtmlPage, pages: &[Page]) -> Result<String> {
    Ok(html! {
        <Layout title={"Disconnected Systems"}>
            {raw!(page.content.as_str())}
        </Layout>
    })
}

pub fn default(page: &HtmlPage, _pages: &[Page]) -> Result<String> {
    Ok(html! {
        <Layout title={"Disconnected Systems"}>
            {raw!(page.content.as_str())}
        </Layout>
    })
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown layout '{1}' in '{0}'")]
    UnknownLayout(PathBuf, String),
}
