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
of the rest api we developed in the last post to give us a nice way to
interactively control the rover. Rather then keeping this to a bare bones
example we will be building the start of a modern frontend web application that
will act as a base we can build upon later. For this we are going to look at and
tie together a few different frontend technologies such as
[vue.js](https://vuejs.org/), [webpack](https://webpack.js.org/) and
[concise.css](http://concisecss.com/) and others.

## Setting Up The Dev Environment And Project

Modern web development makes heavy use of node.js, especially npm, nodes package
manager. This now include browser side code/dependencies in addition to server
side node was originally written for. In our project the server side code is
written in rust but we will still make use of npm for installing and managing
our browser side dependencies in addition to running some tools useful in
development.

Node.js is the first component we need to install as npm and many other tools
are based off it. Head over to the [node.js install
guide](https://nodejs.org/en/download/) and follow the instructions for your
system. Once done you should have both `nodejs` and `npm` installed, verify this
with the following.

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

Our front end code is going to live inside the `ui` subdirectory of the root of
our project. `vue-cli` can be used to create this directory with all of the
boiler plate code needed by vue.js and webpack as well as many other niceties we
can make use of later.

```shell
vue init webpack ui
#
#  This will install Vue 2.x version of the template.
#
#  For Vue 1.x use: vue init webpack#1.0 ui
#
#? Project name ui
#? Project description Web application for a raspberry pi based rover
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

You can read more about vue-cli
[here](https://medium.com/codingthesmartway-com-blog/vue-js-2-quickstart-tutorial-2017-246195cfbdd2#.8w94isatl)
and vue in general [here](https://vuejs.org/v2/guide/index.html).

We can start the dev server by running the following.

```shell
cd ui
npm install
npm run dev
```

This will start a web server and launch a web browser to display our
application. This is an incredibly useful tool during development as it will
automatically build run/reload a web browser whenever we change anything
allowing you to instantly see the change you have made. You can leave this
running in the background as we develop the site.

## Overview Of The Project

* `/build/`: Contains all the build scripts and webpack related configs.
* `/config/`: Contains our applications config.
* `/node_modules/`: Vendored packages and tools that we depend on, this is
  populated when you run `npm install`.
* `/static/`: Any static files we want to serve.
* `/test/`: Unit and integrations tests.
* `/src/`: Our applications code.
* `/src/main.js`: The applications entry point.
* `/src/App.vue`: The main vue component that will be the root of all other
  components.
* `/src/components/`: Any other components we require.
* `/src/router/`: The application router, it decides which paths use which
  components.

## Extra Dependencies

The ui has already been configured with a hole bunch of core dependencies but
our application will require a few extra ones.

* [Concise CSS](http://concisecss.com/) is a simple css framework that give us a
  nice looking base theme to start from. It is similar to bootstrap but light
  weight, simpler and pure css.
* [Vue Resources](https://github.com/pagekit/vue-resource) is a http client for
  the vue framework. It is what we will use to make http calls to our backend
  and handle the responses.

These dependencies can be installed and added tour `ui/project.json` by
running.

```shell
npm install --save concise.css
npm install --save concise-ui
npm install --save vue-resource
```

`vue-cli` has configured webpack to automatically build (ie concatenate and
minify) css files for us so all we need to do to is to import them in
`ui/src/main.js` and webpack will find and process them along with the css from
the vue components into a single resource when we build the project.

We must also register `vue-resource` with vue and set some configuration
options.

```diff
 // The Vue build version to load with the `import` command
 // (runtime-only or standalone) has been set in webpack.base.conf with an alias.
 import Vue from 'vue';
+import VueResource from 'vue-resource';
 import App from './App';
 import router from './router';
 
+import '../node_modules/concise.css/dist/concise.css';
+import '../node_modules/concise-ui/dist/concise-ui.css';
+
 Vue.config.productionTip = false;
+Vue.use(VueResource);
+Vue.http.options.xhr = { withCredentials: true };
 
 /* eslint-disable no-new */
 new Vue({
```

## Configuration

The `config` directory allows us to set different variables for different
environments. We can make use of this to set different api urls for dev and
production. 

The development environment can be configured with `ui/config/dev.env.js`. We
add a variable for the base url of our api and default it to the hostname of the
rover. But if zero-conf/avahi is not available in your environment replace this
with the ip address of the rover.

```diff
 var merge = require('webpack-merge')
 var prodEnv = require('./prod.env')
 
 module.exports = merge(prodEnv, {
+  BASE_URL: '"http://rpizw-rover.local:3000"',
   NODE_ENV: '"development"'
 })
```

The production environment can be configured with `ui/config/prod.env.js`. Like
above we also add the base url of our api but set it to an empty string instead.
This will tell it to use a relative url so we do not need to know how the user
initially connected to our rover.

```diff
 module.exports = {
+  BASE_URL: '""',
   NODE_ENV: '"production"'
 }
```

Note the double quoted expressions, this is required by the `DefinePlugin` from
webpack that is setting up these values. You can look inside the build directory
for exactly how this is setup if you are interested.

## The Controls Component

This will be our only functional component for now. All of its logic and styles
will live in `ui/src/components/Controls.vue` and like all vue components it has
three main sections.

### Template

The template section contains the templated html code for our component.

If a call to the api, or anything else, goes wrong we want to alert the user to
this with a nice error message. We do this by creating an alert-box element
(part of concise-ui) that is visible only when the variable `errorMessage` is
not false which displays the contents of that variable. This allows us to show
the error simply by setting that variable when something goes wrong. We hide it
by adding a close button that simply sets the variable to `false`.

```html
<template>
  <main>
    <section class="alert-box -error" v-if="errorMessage !== false">
      <p>{{ errorMessage }}</p>
      <a class="close" @click="errorMessage = false" href="#">×</a>
    </section>
```

The rest of our template describes the actual controls we want. First, two range
sliders to show/set the current speed and direction of both the left and right
servos. These values are bound to the `left` and `right` variables on our
component which we will use to send the desired speeds to the rover later. We
have also adjusted the range from -10 (reverse) to 10 (forward) down from -100
to 100 as we do not need the full precision in the ui and it makes it slightly
nicer to set exact speeds, these will be multiplied by 10 before sending the
request to the rover.

```html
    <section>
      <div class="controls">
        <form>
          <div grid>
            <div column>
              <label>
                <span>Left Wheel Speed</span>
                <input type="range" id="left" v-model="left" max="10" min="-10" value="0">
              </label>
            </div>
            <div column>
              <label>
                <span>Right Wheel Speed</span>
                <input type="range" id="right" v-model="right" max="10" min="-10" value="0">
              </label>
            </div>
          </div>
```

For the rest of the controls we use three buttons, one to start/stop the rover,
one to enable/disable the rover and one to reset the rover. Both the toggleable
buttons have their text linked to either the `stopped` or `enabled` variable to
ensure they display the right text respectively. All the buttons are linked to a
function when they are clicked, which we will look at in the next section.

```html
          <div>
            <button @click="toggleStopped">
              <span v-if="stopped">Start</span>
              <span v-else>Stop</span>
            </button>
            <button @click="toggleEnabled">
              <span v-if="enabled">Disable</span>
              <span v-else>Enable</span>
            </button>
            <button @click="reset">Reset</button>
          </div>
        </form>
      </div>
    </section>
  </main>
</template>
```

### Script

The script section contains all of the javascript code related to our component.
Here we can place any custom logic our component requires. We start by creating
a constant variable for the base url to the backend api which we set in the
configs above.

```html
<script>
const API_URL = `${process.env.BASE_URL}/api`;
```

Vue requires some functions to be exported that it will use to create the
components, all of the rest of the functions here are inside this export block.
The name option gives our component an id vue can use to identify it.

```javascript
export default {
  name: 'controls',
```

The data function is called during the component creation to get the default
variable that the component will contain. We use it to initialize all of the
variable we require to some sane default.

```javascript
  data() {
    return {
      left: '0',
      right: '0',
      errorMessage: false,
      stopped: true,
      enabled: true,
      wPressed: false,
      aPressed: false,
      sPressed: false,
      dPressed: false,
      interval: null,
    };
  },
```

The created function is called once the component has been created. We use it to
register a couple of functions to the `keyup` and `keydown` events. These
functions will be used to capture the users key presses and trigger certain
actions once pressed which will will look at below.

```javascript
  created() {
    window.addEventListener('keyup', this.keyReleased);
    window.addEventListener('keydown', this.keyPressed);
  },
```

Similar to created `beforeDestroy` is run by vue as part of the components
lifetime but just before it is destroyed as the name implies. We use this to
ensure we send one final `stop` command to the rover.

```javascript
  beforeDestroy() {
    this.stop();
  },
```

The methods block describes all of the methods available to our component, they
can be bound to elements in our template or called from our components reference
(which will be exposed to us as `this` from other component methods). All of the
remaining functions are defined within the methods block.

```javascript
  methods: {
```

The `errorHandler` is a helper function that sets the `errorMessage` from the
given response. It will be passed into all of the http calls to handle any
returned errors.

```javascript
    errorHandler(response) {
      if (response.body && response.body.error) {
        this.errorMessage = response.body.error;
      } else {
        this.errorMessage = `Unable to connect to ${response.url}`;
      }
    },
```

`calculateSpeeds` is another helper function that sets the speeds of the left
and right servos based on which current buttons are pressed. It does this by
adding or subtracting 10 from the left and right speed based on which buttons
are pressed, then limiting each value to be between -10 and 10. This means that
if `w` is pressed the rover will move at full speed forwards. If `a` is pressed
then it will rotate on the spot to the left. But if both `w` and `a` are pressed
it will move in an arc to the left. It does not however call the api at all,
only sets up the speeds locally. Instead the api will be called periodically and
send the currently set values which we will see in the `start`/`stop` functions
below.

```javascript
    calculateSpeeds() {
      let left = 0;
      let right = 0;
      if (this.wPressed) {
        left += 10;
        right += 10;
      }
      if (this.sPressed) {
        left -= 10;
        right -= 10;
      }
      if (this.aPressed) {
        left -= 10;
        right += 10;
      }
      if (this.dPressed) {
        left += 10;
        right -= 10;
      }

      if (left > 10) {
        left = 10;
      }
      if (left < -10) {
        left = -10;
      }
      if (right > 10) {
        right = 10;
      }
      if (right < -10) {
        right = -10;
      }

      this.$set(this, 'left', left);
      this.$set(this, 'right', right);
    },
```

The rover will be controlled by periodically sending the currently set speeds to
it every 50 milliseconds while it is not in the stopped state. We do this
instead of simply sending one command every time something changes as it gives a
more predictable number of requests per second sent and then processed by the
rover, effectively stopping you from overwhelming the rover with a burst of
requests if you change the speed too quickly. It will also lets us build some
fail safes into the rovers rest api. For example we could get it to auto stop if
no request has been received in 200 milliseconds due to a connection loss or a
browser crash. We will however not be looking at that in this post.

There is a trade off with how often we send the commands, slower and it is less
responsive, faster and it requires more processing to handle all of the
requests. 50-100 milliseconds gave a good balance between these tradeoffs but it
was noted that the rest api is heaver on the pis cpu then I would have really
liked. We will looking at more efficient ways to do this in the future for now
it is good enough for the current task.

```javascript
    toggleStopped() {
      this.$set(this, 'stopped', !this.stopped);
      if (this.stopped) {
        this.stop();
      } else {
        this.start();
      }
    },
    stop() {
      clearInterval(this.interval);
      this.left = 0;
      this.right = 0;
      this.wPressed = false;
      this.aPressed = false;
      this.sPressed = false;
      this.dPressed = false;
      this.$http.put(`${API_URL}/stop`).then(null, this.errorHandler);
    },
    start() {
      this.setSpeed();
      this.interval = setInterval(this.setSpeed, 50);
    },
```

The `toggleEnabled` function acts like the `toggleStopped` method above, except
it simply disables/enables the servos while preserving their current speeds and
does not stop the speed commands from being sent.

```javascript
    toggleEnabled() {
      this.$set(this, 'enabled', !this.enabled);
      if (this.enabled) {
        this.$http.put(`${API_URL}/enable`).then(null, this.errorHandler);
      } else {
        this.$http.put(`${API_URL}/disable`).then(null, this.errorHandler);
      }
    },
```

The `setSpeed` function sends the currently set speeds to the rover, with a very
short timeout as we expect to call this function many times a second and we do
not want connections to build up if there are network issues. We multiply the
values by 10 as the rovers speed has a greater precision then we really care
about in this ui.

```javascript
    setSpeed() {
      this.$http.put(`${API_URL}/speed`, {
        left: this.left * 10,
        right: this.right * 10,
      }, { timeout: 200 }).then(null, this.errorHandler);
    },
```

The `reset` function resets the rovers state in both the ui and sends the
request to the rover. This will help to fix any potential problems that might
occur in the rover - at least from a software configuration point of view.

```javascript
    reset() {
      this.enabled = true;
      this.left = '0';
      this.right = '0';
      this.wPressed = false;
      this.aPressed = false;
      this.sPressed = false;
      this.dPressed = false;
      this.$http.put(`${API_URL}/reset`).then(null, this.errorHandler);
    },
```

Lastly we handle the key press and release events by setting the relevant
variables to true or false, or calling one off functions and then call the
`calculateSpeeds` function we described above to update the speed values.

```javascript
    keyPressed(event) {
      if (event.key === 'w') {
        this.wPressed = true;
      } else if (event.key === 'a') {
        this.aPressed = true;
      } else if (event.key === 's') {
        this.sPressed = true;
      } else if (event.key === 'd') {
        this.dPressed = true;
      } else if (event.key === 't') {
        this.toggleStopped();
        return;
      } else if (event.key === 'y') {
        this.toggleEnabled();
        return;
      } else {
        return;
      }
      this.calculateSpeeds();
    },
    keyReleased(event) {
      if (event.key === 'w') {
        this.wPressed = false;
      } else if (event.key === 'a') {
        this.aPressed = false;
      } else if (event.key === 's') {
        this.sPressed = false;
      } else if (event.key === 'd') {
        this.dPressed = false;
      } else {
        return;
      }
      this.calculateSpeeds();
    },
  },
};
</script>
```

### Style

The style sections allows us to define any extra css our component needs. We
define it as `scoped` to limit the css to only the elements we define above
rather then applying them globally. We only do some basic stuff to anchor the
controls to the bottom of the page and center them and generally make them look
nicer.

```html
<style scoped>
.controls {
  position: absolute;
  bottom: 0;
  width: 100%;
  margin-bottom: 50px;
}

.controls form {
  margin: auto;
  width: 50%;
}

main {
  height: 100vh;
  width: 100vw;
  background-color: #f5f5f5;
}
</style>
```

## The Root Component

The applications root component is located at `ui/src/App.vue` and will hold
anything we require on all pages of our application - which is currently
nothing. So lets remove the logo and custom styling of the default boiler plate
that was generated for us. We can expand on this later as we develop them
application in future posts.

```diff
 <template>
   <div id="app">
-    <img src="./assets/logo.png">
     <router-view></router-view>
   </div>
 </template>
 ...
 </script>
 
 <style>
-#app {
-  font-family: 'Avenir', Helvetica, Arial, sans-serif;
-  -webkit-font-smoothing: antialiased;
-  -moz-osx-font-smoothing: grayscale;
-  text-align: center;
-  color: #2c3e50;
-  margin-top: 60px;
-}
+
 </style>
```

Note that the `<router-view>` is where our component will be placed by the
router depending on which page we load. We don't strictly need the router and
could have just written our component as the root component but this will give
us more flexibility later should we decide to add more pages or other components
to the application.

## The Router

`vue-router` allows us to render different components based on the url of the
page. We are not strictly taking advantage of this feature yet but will keep it
in place to make expanding our site easier at a later date. All we require is to
update the router to tell it about our new component and to use it in place of
the `Hello` component for the root path `/`. Edit `ui/src/router/index.js` with
the following changes.

```diff
 import Vue from 'vue';
 import Router from 'vue-router';
-import Hello from '@/components/Hello';
+import Controls from '@/components/Controls';
 
 Vue.use(Router);
 
 ...
   routes: [
     {
       path: '/',
-      name: 'Hello',
-      component: Hello,
+      name: 'Controls',
+      component: Controls,
     },
   ],
 });
```

We can now delete `ui/src/components/Hello.vue` as we will no longer require it.

## Building The UI Into The Image

Finally we can build and package our ui into the image alongside the rest api.
If you recall from our last post static files will be served from
`/srv/rover/ui` so all we need to do is copy our built files to that location
inside the image. This can be done in the `create-image` script in the same spot
that we copy the binaries across.

```diff
 install -Dm755 "target/arm-unknown-linux-gnueabihf/release/rover-cli" "${mount}/usr/local/bin/rover-cli"
 install -Dm755 "target/arm-unknown-linux-gnueabihf/release/rover-server" "${mount}/usr/local/bin/rover-server"
 install -Dm755 "src/bin/rover-server.service" "${mount}/etc/systemd/system/rover-server.service"
+mkdir -p  "${mount}/srv/rover/ui"
+cp -r ui/dist/* "${mount}/srv/rover/ui/"
 
 # Prep the chroot
 mount -t proc none ${mount}/proc
```

The ui can be built for production by running `npm run build` inside the ui
directory. We do not include this command in the `create-image` script above for
the same reason we excluded the `cargo build` command - we do not want it to run
as root. Instead you must run these two commands before running the
`create-image` script. This is automated in travis by editing `.travis.yml` with
the following.

```diff
 - export PATH="$PATH:$HOME/.cargo/bin"
 - curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=stable
 - rustup target add arm-unknown-linux-gnueabihf
+- nvm install 7.7.3
+- nvm use 7.7.3
 script:
 - cargo build --release --target arm-unknown-linux-gnueabihf
+- ( cd ui && npm install && npm run build )
 - sudo ./create-image
 - xz -z rpizw-rover.img -c > rpizw-rover.img.xz
 - zip rpizw-rover.img.zip rpizw-rover.img
```

## Conclusion

We now have a solid foundation to start working from, a nice ui and api to
manually control our rover allowing us to position/reset it with ease. I would
like to expand upon this in the future to allow running and uploading of custom
scripts and programs to preform different dedicated tasks - to give similar
workflow, in a way, to uploading a sketches to an arduino as well as to
integrate the view from the pis camera. But in the next few posts I plan to
start hooking the rover up to some sensors to get it to interact with the world.

Considering this application was not very complex we could have this with a
simple html page and some simple javascript using jquery but I wanted to take
this chance to learn more about some of the emerging web technologies that are
popping up all over the place and to make it easier to grow the project in the
future.

Overall I found vue.js very easy to work with, its simple and has some excellent
documentation with plenty of examples about. Coupled with webpack makes the
process very fluid and more familiar to the compiled work flows I am used to.
Allowing you to have a highly organized and loosely coupled source that compiles
down to a small and clean set of deployable artifacts without needing to set up
a hugely complicated build pipeline by hand - something I have always missed
when working on browser side code in previous projects.

Again I have skipped over the unit testing side of the project, not doing so
would have distracted away from the core concepts of this post and made it far
to long. I hope to look back at this in a future post, possibly once the
application has evolved a bit more beyond the proof of concept stage.
