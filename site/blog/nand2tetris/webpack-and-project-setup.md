Look at setting up a webpack based project from scratch.

There are a lot of outdated webpack tutorials out there that use patterns from webpack 3. Some things you might see but not require anymore as well as some other things I wish I had known eariler:

* separate prod and dev configs - 4 supports mode=development and mode=production.
* input and output have defaults for src/index.js and dist/main.js.
* webpack does not process other files by default, only js files and will not transpile them - only bundle them. But does support plugins and loaders to do all of this if required.

Start by initing the project using yarn or npm.

```bash
yarn init -y
```

```json
{
  "name": "site",
  "version": "1.0.0",
  "main": "index.js",
  "license": "MIT"
}
```

Add webpack and webpack-cli as dependencies.

Webpack-cli allows building and stuff.

```bash
yarn add webpack webpack-cli
```

```diff
diff --git a/site/package.json b/site/package.json
index 3371e53..62f4028 100644
--- a/site/package.json
+++ b/site/package.json
@@ -2,5 +2,9 @@
   "name": "site",
   "version": "1.0.0",
   "main": "index.js",
-  "license": "MIT"
+  "license": "MIT",
+  "dependencies": {
+    "webpack": "^4.27.1",
+    "webpack-cli": "^3.1.2"
+  }
 }
```

Now add a script to package.json to build the application.

```diff
diff --git a/site/package.json b/site/package.json
index 62f4028..2759bf5 100644
--- a/site/package.json
+++ b/site/package.json
@@ -3,6 +3,9 @@
   "version": "1.0.0",
   "main": "index.js",
   "license": "MIT",
+  "scripts": {
+    "build": "webpack --mode=production"
+  },
   "dependencies": {
     "webpack": "^4.27.1",
     "webpack-cli": "^3.1.2"
```

* talk about webpack modes

webpack takes src/index.js and produces dist/main.js.

In `src/index.js`

```jacascript
console.log("Hello, World!");
```

Now build

```bash
yarn build
```

Produces

```bash
dist
└── main.js
```

If you look at the source it will be minified and contain a lot of boiler plate
that we did not include - while a waste for what we have curently it provides
isolation/namespacing for the different modules that will be imported later.

Now for a index file so we have something to view on the web.

src/index.html
