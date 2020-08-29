use render::{component, html, rsx, Render};

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
