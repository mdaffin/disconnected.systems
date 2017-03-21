+++
date = "2017-03-21T20:20:50Z"
title = "Web Interface To Control The Rover"
draft = true
description = "Looks at how to build a web interface using vue.js and webpack to control the raspberry pi rover"
slug = "rover-web-ui"
tags = ["javascript", "raspberry-pi", "vue.js", "webpack"]
+++

Over the past few weeks I have been building a raspberry pi zero w based rover.
This post follows on from the previous posts which you can checkout below.

* [Pi Zero W Rover Setup]({{< relref "blog/2017-03-08-pi-zero-w-rover-setup.md" >}})
* [Customising Raspberry Pi Images with Github and Travis]({{< relref "blog/2017-03-10-custom-rpi-image-with-github-travis.md" >}})
* [Using Rust to Control a Raspberry Pi Zero W Rover]({{< relref "blog/2017-03-13-rust-powered-rover.md" >}})
* [Small Refactor To Prepare For Writing The Rest API]({{< relref "blog/2017-03-18-rover-refactor.md" >}})
* [Writing A Rest API For The Pi Rover]({{< relref "blog/2017-03-19-rover-rest-api.md" >}})

In this post we will look at creating a web ui for the rover. It will make use
of the rest api we developed in the last post to give us a nicer way to
interactively control the rover. Rather then keeping this as a minimal example,
I am going to make use of most of the front end stack to develop the ui rather
then just creating a simple html page. For this we are going to look at a number
of front end technologies such as vue.js, webpack and concise.css.

Vue.js will be responsible for our ui, it is a rendering library similar to
angular.js or react.js. Webpack is a build tool for browser javascript projects,
it allows us to develop code to run on a browser like we develop for service
side node.js applications.

## Setting Up The Dev Environment And Project

Most modern web development it based off node.js, in particular npm - this now
include browser side code as well as server side. In our project the server side
code is written in rust but we will still make use of npm for installing and
managing our browser side dependencies.

Node.js is the first component we need to install as npm and many other tools
are based off it. Head over to the [node.js install guide] and follow the
instructions for your system. This should give you both `nodejs` and `npm`
commands. Verify both of these are installed with the following.

```shell
node --version
#v7.7.2
npm --version
#4.4.1
```

The only other tool we will need is `vue-cli` which can be install globally with
`npm` by running the following.

```shell
npm install -g vue-cli
```

https://medium.com/codingthesmartway-com-blog/vue-js-2-quickstart-tutorial-2017-246195cfbdd2#.8w94isatl
https://vuejs-templates.github.io/webpack/
https://vuejs.org/v2/guide/index.html

Our front end code is going to live inside the `ui` subdirectory of the root of
our project. `vue-cli` can be used to create this directory with all of the
boiler plate code needed by vue.js and webpack as well as many other niceties.

```shell
vue init webpack ui
#
#  This will install Vue 2.x version of the template.
#
#  For Vue 1.x use: vue init webpack#1.0 ui
#
#? Project name ui
#? Project description User interface for a raspberry pi based rover
#? Author Michael Daffin <michael@daffin.io>
#? Vue build standalone
#? Install vue-router? Yes
#? Use ESLint to lint your code? Yes
#? Pick an ESLint preset AirBNB
#? Setup unit tests with Karma + Mocha? Yes
#? Setup e2e tests with Nightwatch? Yes
#
#   vue-cli · Generated "ui".
#
#   To get started:
#   
#     cd ui
#     npm install
#     npm run dev
#   
#   Documentation can be found at https://vuejs-templates.github.io/webpack

```

Now do what the output tells us

```shell
cd ui
npm install
npm run dev
```

This will start a web server and launch a web browser to display our
application. This is an incredibly useful tool during development as it will
automatically build and reload the web browser whenever we change anything. You
can leave this running in the background as we develop the site.

## Adding Concise CSS

```shell
npm install --save concise.css
```

```diff
diff --git a/ui/src/main.js b/ui/src/main.js
index 0a6f825..c972cac 100644
--- a/ui/src/main.js
+++ b/ui/src/main.js
@@ -4,6 +4,8 @@ import Vue from 'vue';
 import App from './App';
 import router from './router';

+import '../node_modules/concise.css/dist/concise.css';
+
 Vue.config.productionTip = false;

 /* eslint-disable no-new */
```
