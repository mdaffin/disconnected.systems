---
date: '2018-11-27T00:00:00Z'
description: "Implementing a simple client side router with web components"
slug: deep-dive-into-webcomponents/router
tags:
- webdev
- web-components
- programming
---

# A Deep Dive into Web Components - The Router

Right, now we have a [simple shell] for our web compoent based application but
there is one problem - it loads the same blank page for both of our routes.
Before we can design each page we need a way to tell the shell which component
to import and load for each route - aka a router.

Now, there are a [bunch of existing routers] out there that you should consider
using for real world applications but for this project I want to see what it
actually takes to build your own router. Mostly as it is not something I have
looked into before and gives me something to talk about than for any other real
reason.

## Pages

Before we setup the routes lets create two stub pages that we can route to.
Each page is its own component with, for now, a simple title so we can tell
them apart.

src/home.js:

```javascript
import { LitElement, html } from "@polymer/lit-element";

class TLHome extends LitElement {
  render() {
    return html`
      <h1>Welcome</h1>
    `;
  }
}

customElements.define("tl-home", TLHome);
```

src/qr-code.js

```javascript
import { LitElement, html } from "@polymer/lit-element";

class TLQRCode extends LitElement {
  render() {
    return html`
      <h1>Qr Code</h1>
    `;
  }
}

customElements.define("tl-qr-code", TLQRCode);
```

## Router

`window.location.pathname` can be used to get the current path for the
currently loaded page, so lets start with a simple component that prints this
out to check everything is working as expected. Inside the empty
src/tl-router.js file (that we created in the last post) add:

```javascript
import { LitElement, html } from "@polymer/lit-element";

class TLRouter extends LitElement {
  render() {
    console.log(routes);
    return html`
      ${window.location.pathname}
    `;
  }
}

customElements.define("tl-router", TLRouter);
```

[images 01]
[impage 02]

## Specifing the routes

There are many different ways to do this but no matter the method we need three
key pieces of infomation, the component html so we know what to add to the dom,
the component javascript source code so we can load it on demand and the path
the compoent should be loaded on.

I quite liked the idea of using child elements to describe these and so went
with the following changes to `src/tl-app.js`:

```javascript
      <tl-router>
        <tl-qr-code tlr-path="^/qr-code/?$" tlr-src="./tl-qr-code.js"></tl-qr-code>
        <tl-home tlr-path="^/?$" tlr-src="./tl-home.js"></tl-home>
        <h1 tlr-path=".*">Page Not Found</h1>
      </tl-router>
```

Each component is a child element to the router compoent. With components child
elements will not be rendered directly, instead to use them you must include a
`<slot></slot>` element in the rendered template. Simply omit this and none of
the child elements will be rendered but we still have access to them
programatically. We could have also passed them into the `tl-router` as a array
attribute but I find the above more expressive.

For each child we add two attributes, `tlr-path`, the path the component will
be loaded on and optionally `tlr-src`, where to import the javascript source
from. Note that the paths are regular expressions and the order is important,
ones higher in the list will be chosen if the regex matches even if later ones
also match the same regex.

It is also worth noting that we will not be supporting dynamic or parametrised
paths (such as `/page/:id`) as they will not be required for this application.

## Parsing the Routes

The routes need to be loaded from the child nodes and stored as a reference in
the router.

```javascript
class TLRouter extends LitElement {
  static get properties() {
    return { currentComponent: Object };
  }

  constructor() {
    super();
    this._parseChildren();
  }

  _parseChildren() {
    const children = [...this.children];
    this.routes = children.map(child => {
      this.removeChild(child);
      return {
        component: child,
        path: child.getAttribute("tlr-path"),
        src: child.getAttribute("tlr-src")
      };
    });
  }
  ...
}
```

The child nodes are also removed from the routers dom as they are not required just yet.

## Loading a Route

The router then needs to decide on which compoent to load for the current route
which is done by looping over the routes and looking for the first one that
matches `window.location.pathname`.

```javascript
class TLRouter extends LitElement {
  constructor() {
    super();
    this._parseChildren();
    this._loadComponent();
  }

  ...

  _loadComponent() {
    const path = window.location.pathname;
    const route = this.routes.find(route => path.match(route.path));
```

If a matching route is found and it has a associated `src` we start to import
it and then set `currentCompoent` property to the routes component.

Note that `import()` is an async function so it will load in the background,
the component will populate once this has finished but are free to add the
element to the dom in the meantime. An error is logged to the console should
this fail however to aid debugging later though it would be nicer to present a
message to the user instead.

```javascript
    ...
    if (route) {
      if (route.src) {
        import(route.src).catch(err =>
          console.error(`error loading route source '${route.src}': ${err}`)
        );
      }
      this.currentComponent = route.component;
    } else {
      this.currentComponent = html`<h1>404 Not Found</h1>`;
    }
  }
  ...
```

The render function can then be updated to display the `currentComponent`.

```javascript
  ...
  render() {
    return html`
      ${this.currentComponent}
    `;
  }
}
```

[Images]

## Stopping the page reloading

While what we have works it has one major drawback - a full page reload on any
route change and the inability to handle the route dynamically updating. To fix
these a coupld of changes are required, first the links in the header must
update the location rather than preforming a full page reload.

### Link element

`a` tags are currently hardcoded in the header so the first step is to pull
them out and into their own link component in `src/tl-link.js`:

```javascript
import { LitElement, html } from "@polymer/lit-element";

class TLLink extends LitElement {
  static get properties() {
    return { href: String };
  }

  ...
}

customElements.define("tl-link", TLLink);
```

Essentially the same as our othercompoents, the main point to note it the href
property added so we can control which page we link to from the outside. It
also include two function, the render function which encapulates the `a` tag
passing the href property though to it, some basic styling for the a tag that
cannot be easily injected into the compoent (we will look at better ways of
themeing elements in a later post).

```javascript
  render() {
    return html`
      <style>
        a {
          display: inline-block;
          padding: 0 16px 0 16px;
          text-decoration: none;
          color: #fff;
        }
      </style>
      <a href="${this.href}" @click="${this._onclick}"><slot></slot></a>
    `;
  }
```

The `a` tag also has an onclick event attached which calls a `_onclick`
function.

```javascript
  _onclick(event) {
    window.history.pushState(null, "", this.href);
    event.preventDefault();
  }
```

Which uses the history api to navigate to a new page without a full reload and
then supressed the default event which would cause a full page reload.

This component is now ready to be included in the header of `src/tl-app.js` replacing the a tags and some of the styling:

```javascript
      <style>
        ...
        header > tl-link {
          height: 100%;
          text-align: center;
          line-height: 40px;
          font-weight: 600;
          border-bottom: 10px solid darken(#1e90ff, 150%);
          box-sizing: border-box;
        }
      </style>
      <header>
        <tl-link class="logo" href="/">Tools</tl-link>
        <tl-link href="/qr-code">Qr Code</tl-link>
      </header>
      ...
```

## Fixing the router

Great, no more full page reload - but now our component does not update as it
is not aware of the path change. Unfortinutly there is no global method to
listen for url changes, instead a global event must be triggered from the link
component:

```javascript
  _onclick(event) {
    const previous = window.location;
    window.history.pushState(null, "", this.href);
    event.preventDefault();
    window.dispatchEvent(
      new CustomEvent("route-changed", {
        detail: { previous: previous }
      })
    );
  }
```

And then attacher an event listener to the router:

```javascript
  connectedCallback() {
    this._windowEventListener = this._loadComponent.bind(this);
    window.addEventListener("route-changed", this._windowEventListener);
  }

  disconnectedCallback() {
    window.removeEventListener("route-changed", this._windowEventListener);
  }
```

Note that we save a copy of the callback function bound to the object so that
`removeEventListener` can find and remove it correctly.

## The back button

Updating the route works if we click on the links, but if the user hits the
back button the components do not update as our custom event never fires.
Luckly the history api has its own event we can listen to `popstate` to reload
the component.

```javascript
  connectedCallback() {
    this._windowEventListener = this._loadComponent.bind(this);
    window.addEventListener("route-changed", this._windowEventListener);
    window.addEventListener("popstate", this._windowEventListener);
  }

  disconnectedCallback() {
    window.removeEventListener("route-changed", this._windowEventListener);
    window.removeEventListener("popstate", this._windowEventListener);
  }
```

Note that we can reuse the bound event listener.

## Defining Fragments

TODO

```json
{
  ...
  "fragments": [
    "src/tl-home.js",
    "src/tl-qr-code.js"
  ],
  ...
}
```

## Dealing with the build warning

Unfortinutly when trying to build the project we end up with this nasty warning.

```bash
% yarn build
yarn run v1.12.3
$ polymer build
info:	Clearing build/ directory...
info:	(default) Building...


        import(route.src).catch(err =>
        ~~~~~~~~~~~~~~~~~

file:///home/mdaffin/projects/mdaffin/tools.daffin.io/src/tl-router.js(43,9) warning [non-literal-import] - Cannot analyze dynamic imports with non-literal arguments
info:	(default) Build complete!
Done in 2.37s.
```

As far as I can tell everything seems to work, even using the es6/es5-bundled
builds (more on these in a future post). So I am not fully sure what the issue
is with just ignoring it. But I hate warnings so lets fix it.

As stated the issue is caused by the dynamic import. In summary polymer hates
when it cannot tell what you are importing (due to it being a variable) and so
wants you to use simple strings, aka `import('./tl-home.js)` instead.

But this kinda breaks our nice API of having to only define the routes in a
single place while having a reusable router. To try and keep our current API
the dynamic importing must be moved out of the router and into some wrapper
components.

[styling]: https://www.smashingmagazine.com/2016/12/styling-web-components-using-a-shared-style-sheet/

[modern js router]: http://krasimirtsonev.com/blog/article/A-modern-JavaScript-router-in-100-lines-history-api-pushState-hash-url
[build your own router]: https://medium.com/@bryanmanuele/how-i-implemented-my-own-spa-routing-system-in-vanilla-js-49942e3c4573
[bunch of existing router]: https://www.webcomponents.org/search/router
[simple shell]: PREVIOUS_POST
