---
date: '2018-11-27T00:00:00Z'
description: "A look into writing a website with web components"
slug: deep-dive-into-webcomponents/shell
tags:
- webdev
- web-components
- programming
---

# A Deep Dive into Web Components - The Shell

[Web components][web-components] are a very interesting web technology and
standard that has made its way [into all major browsers][caniuse-webcomponents]
and has the potential to change the shape of web development over the comming
years.

I doupt it is going to kill off react or vue or angular just like how react
never managed to kill of Angular but it does have the potential to compete with
the big boy framworks and even coperate and intergrate with them.

Every new tech must be trying to solve some problem or it is not worth using.
So what problem is it trying to solve? Well, primartly it is brining component
based architecture nativly to the browsers - so without a framework you can use
web components with nothing more than vanilla javascript. This helps to solve
one of the biggest problem with current framworks - none reusable components
across frameworks. Have a Vue component that does exactly what you wantto use
in your react project? Now you need to pull in a whole nother framework just
for the one component leading to a masivly increased download size and
hampering the preformance of your application.

But the web component standard is understood by the browser so each component
can be truly self contained and incouprate into any other framework with
minimal overhead or extra libraries - at least that is the promise.

In this series I will explore web components to build an application from
scratch to see if they can hold up to this promise and to see how viable they
currently are for actual projects. I will also dive a bit into some other web
technologies such as web assembly as I have some ideas for projects that I want
to test out.

## The Application

I am going to use this as a chance to bootstrap a project I have been wanting
to start for a while - to create a collection of useful web based tools that I
often use but have issues with the existing ones for whatever reason.  For this
project I will start with a simple skeleton application then build a QR Code
generator.

For this project I am going to use as minimal set of tooling and libraries as
required to get a real understanding of what they brin to the table. I am going
to start by using the [polymer libraries] as they do make working with
components nicer while being fairly lightweight and able to intergrate with
other projects. It is also designed as separate parts that you can pull into
your project as required.

## Project Setup

Now, if you want to get off the ground quickly there is the [PWA Starter Kit]
which includes everything you might want for creating your own web component
based PWA application. But I wanted to learn the core concepts and I felt that
there was a bit too much magic in the started kit that I didn't yet understand.
So I am going to start from an empty project instead.

```bash
mkdir web-tools && cd $_
cat >package.json <<EOF
{
  "name": "tools.daffin.io",
  "version": "0.0.1",
  "license": "MIT"
}
EOF
```

The polymer-cli tool is a very useful build tool for webcomponents and makes
working with them very easy. You could replace this with webpack or parcel but
in some quick tests I found polymer-cli both easier to setup and more reliable
- though it misses some nice features of webpack such as live reloading your
page.

```bash
yarn add --dev polymer-cli
```

Then add a couple of scripts to `package.json` to make serving and building the
project a bit easier.

```json
{
  ...
  "scripts": {
    "start": "polymer serve",
    "build": "polymer build"
  }
}
```

The entry point that polymer uses is `index.html` by default so lets go a head and create a minimal boiler plate:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Tools</title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type="text/css">
      body {
        margin: 0;
        padding: 0;
        color: #333;
        font-family: sans-serif;
      }
    </style>
  </head>
  <body>
    <h1>Hello, World!</h1>
  </body>
</html>
```

We are inlining the CSS here - it will not grow very much as each component
will style itself so there is little need for SCSS or Less or generally a
separate style sheet. For now some minimal style to reset the body to a more
minimal state.

To run the polymer dev serve run `yarn start` and head over to http://127.0.0.1:8081 when it finishes:

![Hello World Image]

Depending on which browsers you want to support some polyfils might be required. These are avaiable in the [@webcomponents/webcomponentsjs] package which can be installed with the following.

```bash
yarn add @webcomponents/webcomponentsjs
```

The polyfills must be loaded before any web components are initlised so are
best to add to our index file as the first included script. There are two main
scripts you can pick from `webcomponents-loader.js` which will load any
required polyfills on demand based on runtime feature tests and
`webcomponents-bundle.js` which contains them all bundled togeather. The former
is better for newer browsers so that is what we will use but if you are
targeting older browsers the bundle might be a better choice. Also, if you need
to compile to es5 also include the `custom-elements-es5-adapter.js` script, but
for this project I am only going to target es6 browsers to give them the best
preformance possible.

```html
...
  <head>
    ...
    <script src="node_modules/@webcomponents/webcomponentsjs/webcomponents-loader.js"></script>
  </head>
...
```

## The Shell

The shell is the main component of your application and is where all other
components are rooted and loaded. If you have a multipage application than this
is what contains the router in addition to all common UI elements such as
headers, footers and possibly side bars. There is not much special about the
shell, it is just another web component but polymer can treat it specially
during builds (I am still not sure exactly what this means though).

All components will live in the src directory, they can go anywhere but this
gives a convenitplace (and polymer-cli expects this by default, though it can
be changed if desired). The shell component will live at `src/tl-app.js`, the
`tl-` prefix will be added to all of our components as one requirement of
custom web components is they have to include a `-`. This is to let the browser
tell them appart from native components. So to keep things simple and avoid
having to pick names to force them to include a `-` as well as to namespace our
components it is convinet to pick a common prefix for them all.

Lets load it in the `index.html` document, after the webcomponentjs script and
include it in the body as the only element.

```html
  <head>
    ...
    <script src="/node_modules/@webcomponents/webcomponentsjs/webcomponents-loader.js"></script>
    <script type="module" src="/src/tl-app.js"></script>
    ...
  </head>
  <body>
    <tl-app></tl-app>
  </body>
```

Note that custom elements can be used like any other element and can contain
attribues, be styled, manipulated by javascript or contain sub elements just
like you would expect. But they are only shown if the element has been
registered in the componet store, which will be done in `src/tl-app.js`.

Also, the links should be absolute paths as this index page will be used for
all routes and relative paths won't work for any sub paths.

### Lit-Element

Although web components are nativly supported by the browser polymer have
created a library to make working with them a quite a bit nicer and give a more
familar experence if you are comming from react and jsx. Lit-element will act
as the base class all components derive from and contains some core useful
behaviour. In addition lit-html can be used to give an jsx type syntax to
creating web components. Unlikey jsx the browser has native support for
lit-html syntax and so does not need to be compiled to pure javascript.

First, it must be added as a dependency and download.

```bash
yarn add @polymer/lit-element
```

Then our first minimal element can be created in `src/tl-app.js` by first
importing lit-element:

```javascript
import { LitElement, html } from "@polymer/lit-element";
```

Note that while modern web browsers understand the import statement, they do
not understand node_modules or that the path above should be search for inside
node_modules. But polymer-cli will translate this for us into a path the
browser understands.

Components are typically written as es6 classes, in perticular LitElement based
components must extend the ListElement base class.

```javascript
class TLApp extends LitElement {
  render() {
    return html`
      <h1>Hello, LitElement!</h1>
    `;
  }
}
```

This only has one required method, `render()`. Here we make use of the html
[tagged template literal] which allows lit-html to do some processing on the
template before returning it and it is responsible for rendering the element to
the dom.

Finally we must register the element.

```javascript
customElements.define("tl-app", TLApp);
```

This is where the element name is created and it does not matter what the file
or class is called but is convention to have the same name (but snakecased for
the class name).

And there, that is our first component. Reloading the page and it will update
to the new heading:

![Image 2]

### Polymer config

Polymer works without a config - as all good tools should and as we have seen.
But their documentations states that you should give it some hints to help with
preformance. At this stage telling it where our application shell is located
and enabling npm mode. In `polymer.json`:

```json
{
  "shell": "src/tl-app.js",
  "npm": true
}
```

I am not fully sure what specifing the shell does, but I assume it helps
optimise the build in some way. And the npm flag tells polymer to look in
node_modules for components - though I syspect this might be a default setting
in later versions of the cli tool (polymer use to be bower based but has been
moving towards npm).

We will add to this config as required.

### Building the Shell

Now it is time to actually build the shell. There are many ways to do this but
a seemingly common approach is to use one of the existing layout components
provided by various component libraries such as [@polymer/app-layout] or
[vaadin-app-layout]. These will make it easy to create some very nicly styled
application shells with very little code. You can find other alterntives as
well as other components at [www.webcomponents.org].

One very nice thing about web components is that you are not stuck with only
those of the framework/libraries you picked and are free to mix and match
components from different collections. Though this may look strange if their
style differes quite considrably it is very handy when your favoured one does
not have the exact compoent you requre.

But, this is a project to learn and experiment on and I want to see what it is
like to do things manually - largley so I can better understand what other
components are doing and judge for myself if they are actually adding value or
simply bloating a project.

The header was simple enough that I decided to just inline it in the app
component, after all it is one simple element with a couple of children and a
bit of styling.

The `tl-router` component is also imported and added as the body of the page.

```javascript
import { LitElement, html } from "@polymer/lit-element";
import "./tl-router.js";

class TLApp extends LitElement {
  render() {
    return html`
      <style>
        header {
          height: 40px;
          display: flex;
          align-items: center;
          background-color: #1e90ff;
        }

        .logo {
          margin-right: auto;
        }

        header > a {
          padding: 0 16px 0 16px;
          height: 100%;
          text-align: center;
          line-height: 40px;
          font-weight: 600;
          text-decoration: none;
          border-bottom: 10px solid darken(#1e90ff, 150%);
          box-sizing: border-box;
          color: #fff;
        }
      </style>
      <header>
        <a class="logo" href="/">Tools</a> <a href="/qr-code">Qr Code</a>
      </header>
      <tl-router></tl-router>
    `;
  }
}

customElements.define("tl-app", TLApp);
```

To get this to run we just need to create a stub `tl-router` which can just be
an empty file.

```bash
touch src/tl-router.js
```



[https://www.webcomponents.org]: https://www.webcomponents.org
[@polymer/app-layout]: https://www.webcomponents.org/element/@polymer/app-layout
[vaadin-app-layout]: https://www.webcomponents.org/element/vaadin/vaadin-app-layout
[tagged template literal]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#Tagged_templates
[@webcomponents/webcomponentsjs]: https://www.npmjs.com/package/@webcomponents/webcomponentsjs
[PWA Starter Kit]: https://pwa-starter-kit.polymer-project.org/
[polymer libraries]: https://www.polymer-project.org/
[web-components]: TODO
[caniuse-webcomponents]: TODO
