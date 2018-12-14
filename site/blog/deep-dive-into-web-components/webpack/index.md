---
date: '2018-11-27T00:00:00Z'
description: "Implementing a simple client side router with web components"
slug: deep-dive-into-webcomponents/webpack
tags:
- webdev
- web-components
- programming
---

# A Deep Dive into Web Components - To Parcel

So, while writing up the next article I came across a showstopping problem -

```
Unable to load import: No parser for for file type wasm
```

So, polymer-cli does not yet support wasm. This is an issue since one of the
projects I am planning has a nice usecase for including web assembly. And this
rules out basing it off polymer-cli which is a shame since it was so nice to
set up and use.

I brefily tried parcel as well but it seems to lack support for web components
and most of the avaiable plugins I could find where for older html imports
rather than the new js modules. Overall it also lacked a lot of documentation
or decent examples so I decided to go with webpack instead which has lots of
examples and documentation for both webpack and web assembly.

## Adding Webpack

Webpack has three main packages, `webpack` the core libraries, `webpack-cli` a
cli tool for running webpack and building your application and optinally
`webpack-dev-server` which gives you a local dev server with hot reloading.
Update `package.json` with these and remove the old `polymer-cli` dependency as
it will no longer be required.

```diff
diff --git a/package.json b/package.json
index 7e5cbe4..e3afb2a 100644
--- a/package.json
+++ b/package.json
@@ -3,11 +3,14 @@
   },
   "devDependencies": {
-    "polymer-cli": "^1.9.1"
+    "webpack": "^4.27.0",
+    "webpack-cli": "^3.1.2",
+    "webpack-dev-server": "^3.1.10"
   },
   "dependencies": {
     "@polymer/lit-element": "^0.6.3",
```

While we are at it the start and build commands can be replaced with their
webpack equlivents.

```diff
diff --git a/package.json b/package.json
index 7e5cbe4..e3afb2a 100644
--- a/package.json
+++ b/package.json
@@ -3,11 +3,14 @@
   "version": "0.0.1",
   "license": "MIT",
   "scripts": {
-    "start": "polymer serve",
-    "build": "polymer build"
+    "build": "webpack --mode=production",
+    "start": "webpack-dev-server --hot --inline --mode=development"
   },
   "devDependencies": {
```

Run `yarn install` to download the new dependencies. If you then try to start
the dev server `yarn start` you end up with the following error.

```
ERROR ...blah... Module not found: Error: Can't resolve './src' in ...blah...
```

This basically means you are missing `./src/index.js` which webpack uses as its
default input. We have two options here, create this file or tell webpack where
the actual entry point (ie `./src/tl-app.js`) is. I chose to do the latter.

Create `webpack.config.js` with the following to tell it about the application
entry point.

```javascript
module.exports = {
  entry: "./src/tl-app.js"
};
```

Restart the dev server and it should now start successfully. But when you try
to navigate to it in a browser you will get a blank page. But intrestingly our
app in being downloaded:

![image]

But also produces the following error:

![image]

It seems the application is just being served directly from the source without
any transpiling. So lets build the application and see what we have.

```bash
yarn build
```

By default webpack builds our entry point to `dist/main.js`, which it does:

```bash
dist
├── 1.js
├── 2.js
└── main.js
```

Along with two other files. A quick look at these revieles they are
compiled/minified versions of `tl-home` and `tl-qr-code` - the two pages in our
applications. So webpack has correctly bundled our application in the same way
that polymer-cli did. This is good and we should expect to see it only loading
the correct routes on demand later.

## The index page

But the build is currently missing the index page so will not work when
deployed to a static web server. To fix this `html-webpack-plugin` can be used
to generate a html file basied on a template (such as the existing index.html
we have).

```bash
yarn add html-webpack-plugin
```

And enable it in the webpack config.

```javascript
const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: "./src/tl-app.js",
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "index.html")
    })
  ]
};
```

If we now run a `yarn build` we now get index.html in dist.

```bash
dist
├── 1.js
├── 2.js
├── index.html
└── main.js
```

And better yet the site now functions.

[image]

But there is still a problem, if you look at what requests are made you can see
it is loading `webcomponents-loader.js` and `tl-app.js`, neither are inside the
dist and one is no longer required.

[image]

If you take a look at `dist/index.html` you can see it is mostly unmodified
apart from the main script being added to the bottom of the body.

```html
  <body>
    <tl-app></tl-app>
  <script type="text/javascript" src="main.js"></script></body>
</html>
```

This means that we no longer need to include the link to `tl-app.js` ourselves and can let webpack inject that.

```diff
diff --git a/index.html b/index.html
index 94a55c5..981381c 100644
--- a/index.html
+++ b/index.html
@@ -5,7 +5,6 @@
     <meta name="viewport" content="width=device-width, initial-scale=1" />
     <title>Tools</title>
     <script src="/node_modules/@webcomponents/webcomponentsjs/webcomponents-loader.js"></script>
-    <script type="module" src="/src/tl-app.js"></script>
     <style type="text/css">
       body {
         margin: 0;
```

That fixes the extra script, but we do still require the webcompoents
pollyfills.

## Webcomponents pollyfills

The problem here is that it is not linked to any or our javascript
files so webpack does not pull it in or bundle it up in our app - and we don't
want to bundle it up.

This can be done with `copy-webpack-plugin`.

```bash
yarn add copy-webpack-plugin
```

```diff
diff --git a/webpack.config.js b/webpack.config.js
index cb63b0e..0b0380d 100644
--- a/webpack.config.js
+++ b/webpack.config.js
@@ -1,11 +1,15 @@
 const path = require("path");
 const HtmlWebpackPlugin = require("html-webpack-plugin");
+const CopyWebpackPlugin = require("copy-webpack-plugin");

 module.exports = {
   entry: "./src/tl-app.js",
   plugins: [
     new HtmlWebpackPlugin({
       template: path.resolve(__dirname, "index.html")
-    })
+    }),
+    new CopyWebpackPlugin([
+      "node_modules/@webcomponents/webcomponentsjs/webcomponents-loader.js"
+    ])
   ]
 };
```

```diff
diff --git a/index.html b/index.html
index 94a55c5..0d4563b 100644
--- a/index.html
+++ b/index.html
@@ -4,8 +4,7 @@
     <meta charset="utf-8" />
     <meta name="viewport" content="width=device-width, initial-scale=1" />
     <title>Tools</title>
-    <script src="/node_modules/@webcomponents/webcomponentsjs/webcomponents-loader.js"></script>
+    <script src="/webcomponents-loader.js"></script>
     <style type="text/css">
       body {
         margin: 0;
```

```bash
dist
├── 1.js
├── 2.js
├── index.html
├── main.js
└── webcomponents-loader.js
```

## Subpages

The site works as expected, you can navigate to each of the pages and
everything loads. But if you hit refresh while you are not on the index page
you get a 404 error. This is becuase although the spa is working no other pages
exist so cannot be loaded directly, only ever nagivated from the index page.

There are two ways to solve this, tell your webserver to serve the index.html
for all 404 pages or a set of know pages. Or get webpack to create an html file
for each route.

The former varies per webserver but all major ones, including static hosting
providers like netlify and AWS, provide a way to do this. But it is far simpler
to create the extra html files at build time and not have to worry about
special web server config if you can avoid it.

```diff
diff --git a/webpack.config.js b/webpack.config.js
index 0b0380d..dbbcbca 100644
--- a/webpack.config.js
+++ b/webpack.config.js
@@ -8,6 +8,10 @@ module.exports = {
     new HtmlWebpackPlugin({
       template: path.resolve(__dirname, "index.html")
     }),
+    new HtmlWebpackPlugin({
+      template: path.resolve(__dirname, "index.html"),
+      filename: "qrcode/index.html"
+    }),
     new CopyWebpackPlugin([
       "node_modules/@webcomponents/webcomponentsjs/webcomponents-loader.js"
     ])
```

```bash
dist
├── 1.js
├── 2.js
├── index.html
├── main.js
├── qrcode
│  └── index.html
└── webcomponents-loader.js
```

Great, but try reloading the dev server and it is still broken. So for that we
will use the former method and configure it to serve the index.html for all
404s.

```diff
diff --git a/webpack.config.js b/webpack.config.js
index dbbcbca..13d9e63 100644
--- a/webpack.config.js
+++ b/webpack.config.js
@@ -15,5 +15,10 @@ module.exports = {
     new CopyWebpackPlugin([
       "node_modules/@webcomponents/webcomponentsjs/webcomponents-loader.js"
     ])
-  ]
+  ],
+  devServer: {
+    historyApiFallback: true
+  }
 };
```

And there, that is the dev and production builds working for all of our one
extra page. This could get anoying as the site grows as for each page you must
remember to add the new paths or it will break during production, in the future
I might look at ways to automate this a bit better.

## Summary

And that is the minimal ammount of webpack config and plugins you need for a
simple web component project. Now, there are some missing bits here, notabally
no babel so support for older browsers is limited - but I am not so worryied
about these at this point in time and will look to add them when/if it becomes
required.
