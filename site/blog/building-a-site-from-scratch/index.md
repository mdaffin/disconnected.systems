---
date: '2019-02-01T09:00:00Z'
description: Build a web site from scratch with web components
slug: building-a-web-site-from-scratch
tags: []
sidebar: []
---

# Building a site from scratch Part 1: Webpack and Project Setup

https://www.valentinog.com/blog/webpack-tutorial/

## Yarn

```bash
yarn init -y
```

Which produces:

```json
{
  "name": "demo",
  "version": "1.0.0",
  "main": "index.js",
  "license": "MIT"
}
```

## Adding Webpack

```bash
yarn add --dev webpack webpack-cli
```

```diff
diff --git a/package.json b/package.json
index d1a3ffd..c56a62e 100644
--- a/package.json
+++ b/package.json
@@ -2,5 +2,9 @@
   "name": "site-from-scratch",
   "version": "1.0.0",
   "main": "index.js",
-  "license": "MIT"
+  "license": "MIT",
+  "devDependencies": {
+    "webpack": "^4.29.0",
+    "webpack-cli": "^3.2.1"
+  }
 }
```

`src/index.js`

```javascript
console.log("Hello, World!");
```

```diff
diff --git a/package.json b/package.json
index c56a62e..ddd1cfe 100644
--- a/package.json
+++ b/package.json
@@ -3,6 +3,10 @@
   "version": "1.0.0",
   "main": "index.js",
   "license": "MIT",
+  "scripts": {
+    "dev": "webpack --mode development",
+    "build": "webpack --mode production"
+  },
   "devDependencies": {
     "webpack": "^4.29.0",
     "webpack-cli": "^3.2.1"
```

## Index Page

```bash
yarn add --dev html-webpack-plugin html-loader
```

`webpack.config.js`

```javascript
const HtmlWebPackPlugin = require("html-webpack-plugin");

module.exports = {
  module: {
    rules: [
      {
        test: /\.html$/,
        use: [{ loader: "html-loader", options: { minimize: true } }]
      }
    ]
  },
  plugins: [
    new HtmlWebPackPlugin({
      template: "./src/index.html",
      filename: "./index.html"
    })
  ]
};
```

`src/index.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>title</title>
  </head>
  <body></body>
</html>
```

```bash
yarn build
```

```bash
dist
├── index.html
└── main.js
```

```bash
dist
├── index.html
└── main.js
package.json
src
├── index.html
└── index.js
webpack.config.js
yarn.lock
```

```html
<!doctype html> <html lang=en> <head> <meta charset=utf-8 /> <title>title</title> </head> <body><script type="text/javascript" src="main.js"></script></body> </html>
```

It has both minified the html but also injected the main script.

## Development server

```bash
yarn add --dev webpack-dev-server
```

```diff
diff --git a/package.json b/package.json
index 5b8ef58..8625790 100644
--- a/package.json
+++ b/package.json
@@ -4,7 +4,7 @@
   "main": "index.js",
   "license": "MIT",
   "scripts": {
-    "dev": "webpack --mode development",
+    "dev": "webpack-dev-server --mode development --open",
     "build": "webpack --mode production"
   },
   "devDependencies": {
```

```bash
yarn dev
```

It opens a browser and prints `Hello, World!` to the console.
