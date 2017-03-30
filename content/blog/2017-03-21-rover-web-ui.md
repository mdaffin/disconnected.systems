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

* [Concise CSS]() is a simple css framework that give us a nice
  looking base theme to start from. It is similar to bootstrap but light weight,
  simpler and pure css. 
* [Vue Resources]() is a http client for the vue framework. It is what we will
  use to make http calls to our backend and handle the responses.

These dependencies can be installed and added to our `ui/project.json` by
running.

```shell
npm install --save concise.css
npm install --save concise-ui
npm install --save vue-resources
```

Since we are using webpack and `vue-cli` has configured it to handle css files
we can simply import it, along with `vue-resource` in our `ui/src/main.js` file
to make use of them. For `vue-resource` we must also register it with vue and
configure it.

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

## The Controls Component

This will be our main and for now only functional component. All of its logic and styles will live in `ui/src/components/Controls.vue`. Vue components have tree main sections.

### Template

The template section contains the html templated code for our component.

The last error received from the api should be displayed to the user to show
them what went wrong. This will be done using an alert box from the concise-ui
framework along with some vue data bindings. The `errorMessage` variable will be
used to hold any error messages that we wish to display so we bind that inside a
`<p>` tag. Then we can use a `v-if` to set the element to not be rendered when
the `errorMessage` is set to `false`. Lastly we create a small close button to
allow the user to clear the error message when clicked by setting the `errorMessage` variable to `false`.

```html
<template>
  <main>
    <section class="alert-box -error" v-if="errorMessage !== false">
      <p>{{ errorMessage }}</p>
      <a class="close" @click="errorMessage = false" href="#">×</a>
    </section>
```

In the second section we will have all of the controls. Firstly two range
sliders to show/set the current speed of both the left and right servos. They
will have a minimum value of `-10` to indicate full reverse and a max of `10`
for full forwards where `0` is stopped - following our api values. These two
sliders are then bound to the `left` and `right` variables using `v-model`.

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

For the last controls we use three buttons, one to start/stop the rover, one to
enable/disable the rover and one to reset the rover. Both the toggleable buttons
have their text linked to either the `stopped` or `enabled` variable to ensure
they display the right text respectively. All the buttons are linked to a
function when they are clicked, which we will look at in a bit.

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
Here we can place any custom logic our component requires. We start by creating a constant variable for the base url to the backend api.

```html
<script>
const API_URL = 'http://rpizw-rover.local:3000';
```

Vue requires some functions to be exported that it will use to create the
components. TODO(describe name). 

```javascript
export default {
  name: 'controls',
```

The data function is called during the component creation to get the default
variable that the component will contain. We use it to initialize all of the
variable we require to some sane defaults.

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
are pressed, then limiting each value to be between -10 and 10. This was if `w`
is pressed the rover will move at full speed forwards. If `a` is pressed then it
will rotate on the spot to the left. But if both `w` and `a` are pressed it will
move in an arc to the left. It does not however call the api at all, only sets
up the speeds locally. Instead the api will be called periodically and send the
currently set values which we will see in the `start`/`stop` functions below.

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
it every 50 milliseconds. We do this instead of simply sending one command every
time something changes as there is potential for api calls to be lost over the
network. By constantly sending the commands it wont matter if a few get lost as
we only have to wait 50 milliseconds to correct it rather then sending another
command. This also lets us build some fail safes into the rovers rest api, for
example we could get it to auto stop if it does not receive a command within 200
milliseconds such as if we lose connection completely but we will not be looking
at that in this post.

There is a trade off with how often we send the commands, slower and it is less
responsive, faster and it requires more processing to handle all of the
requests. 50-100 milliseconds gave a good balance between these values but it
was noted that the rest api is heaver on the cpu then I would have really liked.
We will looking at more efficient ways to do this in the future for now it is
good enough for the current task.

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
      this.$http.put(`${API_URL}/api/stop`).then(null, this.errorHandler);
    },
    start() {
      this.setSpeed();
      this.interval = setInterval(this.setSpeed, 50);
    },
```

```javascript
    toggleEnabled() {
      this.$set(this, 'enabled', !this.enabled);
      if (this.enabled) {
        this.$http.put(`${API_URL}/api/enable`).then(null, this.errorHandler);
      } else {
        this.$http.put(`${API_URL}/api/disable`).then(null, this.errorHandler);
      }
    },
```

```javascript
    setSpeed() {
      this.$http.put(`${API_URL}/api/speed`, {
        left: this.left * 10,
        right: this.right * 10,
      }, { timeout: 200 }).then(null, this.errorHandler);
    },
```

```javascript
    reset() {
      this.enabled = true;
      this.left = '0';
      this.right = '0';
      this.wPressed = false;
      this.aPressed = false;
      this.sPressed = false;
      this.dPressed = false;
      this.$http.put(`${API_URL}/api/reset`).then(null, this.errorHandler);
    },
```

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

which contains all of the css our component requires.
