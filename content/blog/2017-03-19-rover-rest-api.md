+++
date = "2017-03-19T17:20:50Z"
title = "Writing A Rest API For The Pi Rover"
draft = false
description = "Using the iron web framework to build a simple rest api for the raspberry pi zero w rover"
slug = "rover-rest-api"
tags = ["rust", "raspberry-pi", "iron", "rest"]
+++

Over the past few weeks I have been building a raspberry pi zero w based rover.
This post follows on from the previous posts which you can checkout below.

* [Pi Zero W Rover Setup]({{< relref "blog/2017-03-08-pi-zero-w-rover-setup.md" >}})
* [Customising Raspberry Pi Images with Github and Travis]({{< relref "blog/2017-03-10-custom-rpi-image-with-github-travis.md" >}})
* [Using Rust to Control a Raspberry Pi Zero W Rover]({{< relref "blog/2017-03-13-rust-powered-rover.md" >}})
* [Small Refactor To Prepare For Writing The Rest API]({{< relref "blog/2017-03-18-rover-refactor.md" >}})

In this post we are going to look at wrapping our rover api into a rest api that
we will be able to build a web interface on top of.

## New Dependencies

Add the following to the `[dependencies]` section in `Cargo.toml`.

```ini
iron = "0.5.0"
router = "0.5.1"
logger = "0.3.0"
log = "0.3.7"
env_logger = "0.4.2"
chan-signal = "0.2.0"
chan = "0.1.19"
serde = "0.9.11"
serde_json = "0.9.9"
serde_derive = "0.9.11"
```

For this we require a fair few dependencies, lets take a brief moment to talk
about what each one brings us below.

* [Iron](http://ironframework.io/) is the web framework that we are going to use,
it is currently the most popular web framework for rust but unfortunately still
lacks in overall documentation. This, however, also holds true for allot of the
alternative frameworks. [Rocket](https://rocket.rs/) was a tempting alternative,
its documentation seems more complete but still requires rust nightly which I
want to avoid at the moment.

* [Router](https://github.com/iron/router) is simply the router middleware for
the iron web framework, it lets us handle multiple paths and bind them to
different functions.

* [Logger](https://github.com/iron/logger) is the logging middleware for iron. It
lets us log all of the requests that we receive with some useful information
like the time it took to process.

* [Log](https://github.com/rust-lang-nursery/log) gives us some handy macros like
`info!` `warning!` and `error!` that act like the `println!` macro allowing us
to print scoped messages to the logs.

* [Env_logger](https://github.com/rust-lang-nursery/log) is the implementation of
logging, they two libraries above basically wrap this library.

* [Chan](https://github.com/BurntSushi/chan) gives us access to channels which
we use with chan-signal.

* [chan-signal](https://github.com/BurntSushi/chan-signal) library allow us to
capture and gracefully handle signals that might be sent to our program. In
particular we want to be able to tear down our rover (aka stop it) when our
webserver exits for any reason. `SIGTERM` and `SIGINT` are two signals that are
commonly used to tell applications to stop. `SIGTERM` is sent by default when
you run `kill` and `SIGINT` is sent when you press `ctrl+c`.

* [Serde](https://github.com/serde-rs/serde) is a serialisation library, it allows
us to convert different encoded string into structs and vice versa. We will make
use of it to convert message we send back to the client and messages we receive
from the client to a form rust understands.

* [Serde_json](https://github.com/serde-rs/json) is the json implementation of
serde, we are only going to be converting to and from json.

* [Serde_derive](https://github.com/serde-rs/serde) allows us to use
`#[derive(Serialize, Deserialize)]` save us from writing a bunch of boiler plate
code to serialize and deserialize our types and overall makes the serde library
very simple to use.

## The Rover Server

In this section we will look at writing `src/bin/rover-server.rs` which will
become our server binary, this is the only rust file we will need to edit. I am
going to split this file up to talk about each bit separately, each of the
sections below should be appended to `src/bin/rover-server.rs` as it is
mentioned.

### Includes

This first bit is simple, we just declare all the external libraries that we
will be using and any use statements. Also we define a few constants that we
used in the `rover-cli` tool.

```rust
extern crate rpizw_rover;
extern crate iron;
extern crate router;
extern crate logger;
#[macro_use]
extern crate chan;
extern crate chan_signal;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use logger::Logger;
use router::Router;
use rpizw_rover::Rover;
use chan_signal::Signal;
use std::io::Read;

const PWM_CHIP: u32 = 0;
const LEFT_PWM: u32 = 0;
const RIGHT_PWM: u32 = 1;
```

### Response Structures

All of the possible responses from our api will be constructed from the
`ResponsePayload` enum. This allows us to strictly define all possible responses
in one place and handle them together. As you can see from the `#` annotation
around the enum, it can be serialized and deserialized by serde. We also use the
`untagged` flag as we don't want any extra tags surrounding or within our
response. You can read more about the container attributes for serde
[here](https://serde.rs/attributes.html#container-attributes).

The only possible responses we require at the moment is the error response which
will look like `{"success":false,"error":"some error message"}` to the client.
As well as the success message, `{"success":true}`. We can expand on these in
the future if we require more response types (for example returning the current
speed, status or sensor data).

```rust
/// The payload that is json encoded and send back for every request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ResponsePayload {
    Error { success: bool, error: String },
    Simple { success: bool },
}
```

We also create some convince functions to make this structure easier to work
with.

```rust
impl ResponsePayload {
    /// The response that is sent when an error in encountered.
    pub fn error(error: String) -> ResponsePayload {
        ResponsePayload::Error {
            success: false,
            error: error,
        }
    }
    /// The response that is sent when a request is carried out without error
    /// and there is no data to return to the client.
    pub fn success() -> ResponsePayload {
        ResponsePayload::Simple { success: true }
    }

    /// Converts the payload to a iron response with the ok status.
    pub fn to_response(self) -> Response {
        let mut resp = Response::with((status::Ok, serde_json::to_string(&self).unwrap()));
        resp.headers.set(ContentType(Mime(TopLevel::Application,
                                          SubLevel::Json,
                                          vec![(Attr::Charset, Value::Utf8)])));
        resp
    }
}
```

### Error Macro

Iron has a very useful macro `itry!` that wraps rusts `try!` macro making it
easier to return `IronErrors`. We have reimplemented this macro so that we can
json encode the error messages produced and place it in the body of the
response. It has three possible ways it can be called, 

* `rtry!(rover.stop())` - that produce the default message and an internal server error.
* `rtry!(rover.stop(), "could not stop: {}")` - that produces a custom message
round the error message as an internal server error.
* `rtry!(rover.stop(), "could not stop: {}", status.BadRequest)` - the produces
a custom message with the specified status.

```rust
/// Reimplementation of irons itry! macro that sets the body to a json message on error.
macro_rules! rtry {
    ($result:expr) => (rtry!($result, "{}"));
    ($result:expr, $message:expr) => (rtry!($result, $message, iron::status::InternalServerError));
    ($result:expr, $message:expr, $status:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            let message = serde_json::to_string(&ResponsePayload::error(format!($message,
                                                err))).unwrap();
            return ::std::result::Result::Err(iron::IronError::new(err, ($status, message)))
        }
    });
}
```

### Resetting The Rover

Here is a simple helper function to reset the rover to a consistent state. Its
job is to ensure the rover has been properly initialised from any state it may
have ended up in. It does this by exporting, disabling then unexporting the pwm
modules. This was required as there was some weird behaviour something did not
disable the pwm modules before unexporting them previously. We then reexport,
stop and enable them.

```rust
/// Helper function to ensure the rover is stopped, enabled and ready to start.
fn reset_rover() -> rpizw_rover::error::Result<()> {
    let rover = Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM)?;
    rover.export()?;
    rover.enable(false)?;
    rover.unexport()?;
    rover.export()?;
    rover.stop()?;
    rover.enable(true)
}
```

### The Main Function

```rust
fn main() {
```

First we setup the env_logger so it is ready for whenever we require it and 
reset the rover as defined in the last section.
```rust
    env_logger::init().unwrap();
    reset_rover().unwrap();
```

Next we setup the routes our application will handle. The functions used here
will be defined in the next section. Note that all of the endpoints are put
calls as they all set things on the rover.

```rust
    let mut router = Router::new();
    router.put("/api/reset", reset, "reset");
    router.put("/api/stop", stop, "stop");
    router.put("/api/enable", enable, "enable");
    router.put("/api/disable", disable, "disable");
    router.put("/api/speed", set_speed, "set_speed");
```

Iron has a very flexible middleware system, which is used by chaining middleware
together with the `Chain` type. We add the `logger_before` to execute before our
router, which sets up some timing variables that are used by `logger_after`.
`logger_after` is setup to run after our router and outputs the request to the
logs detailing what was called and how long it took, as well as any errors that
were encountered.

```rust
    let mut chain = Chain::new(router);
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
``

Now we are ready to get things running, we just need to start `chan_signal` to
listen for `SIGTERM` and `SIGINT` then start they iron web server.
```rust
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let mut serv = Iron::new(chain).http("0.0.0.0:3000").unwrap();
    info!("listening on 0.0.0.0:3000");
```

We capture the `serv` here so that we don't block on the server but instead
block on the `chan_singal` we setup above in the select defined below. This
allows us to wait for a signal and close the server once we receive it followed
by any tear down code we require.

```rust
    // Block until SIGINT or SIGTERM is sent.
    chan_select! {
        signal.recv() -> _ => {
            info!("received signal shutting down");
            // Shutdown the server. Note that there is currently a bug in hyper
            // that means the server does not actually stop listening at this
            // point.
            serv.close().ok();
        }
    }
```

Lastly we ensure we stop the rover so it does not go running off uncontrollably
and end the main function.

```rust
    // Ensure we stop the rover and cleanup.
    let rover = Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM).unwrap();
    rover.unexport().unwrap();
}
```

### Route Functions

The routes we used above can now be defined, they are all very similar. They
simply call the appropriate rover function and return an `Ok` response if no
error was encountered. The reset function just calls the reset helper function
we defined and used in previous sections.

```rust
/// Resets the rover to its default settings.
fn reset(_: &mut Request) -> IronResult<Response> {
    rtry!(reset_rover());
    Ok(ResponsePayload::success().to_response())
}

/// Stops the rover from moving. Equivalent to settings its speed to 0.
fn stop(_: &mut Request) -> IronResult<Response> {
    let rover = rtry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    rtry!(rover.stop());
    Ok(ResponsePayload::success().to_response())
}

/// Enables the rover, allowing it to move. The rover will start moving at what
/// ever its speed was last set to (this includes stop). It is recommended to
/// call `speed` or `stop` before enabling movement if you are unsure about its
/// previous speed.
fn enable(_: &mut Request) -> IronResult<Response> {
    let rover = rtry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    rtry!(rover.enable(true));
    Ok(ResponsePayload::success().to_response())
}

/// Disables the rover, stopping it from moving and reacting to future calls to
/// speed/stop. Note that this is a soft stop, it does not cause the rover to
/// `break` like calling `stop` does. As a result the rover will coast for a
/// short period of time. If this is not desired then call `stop` followed by a
/// short delay before disabling the rover.
fn disable(_: &mut Request) -> IronResult<Response> {
    let rover = rtry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    rtry!(rover.enable(false));
    Ok(ResponsePayload::success().to_response())
}
```

The `set_speed` endpoint is slightly more complex as it needs to accept and
parse some user input. This is done by creating a struct and json decoding the
body of the request. Then passes these values to the `set_speed` function on the
rover. It also returns a `BadRequest` if it could not parse the json.

```rust
/// Sets the speed of the rover. The speed can be any value from 100 to -100. 0
/// causes the rover to break and negative numbers cause it to go in reverse.
fn set_speed(req: &mut Request) -> IronResult<Response> {
    #[derive(Serialize, Deserialize, Debug)]
    struct SpeedRequest {
        left: i8,
        right: i8,
    }
    let mut body = String::new();
    rtry!(req.body.read_to_string(&mut body));
    let SpeedRequest { left, right } = rtry!(serde_json::from_str(&body),
                                             "invalid json: {}",
                                             status::BadRequest);

    let rover = rtry!(Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM));
    rtry!(rover.set_speed(left, right));
    Ok(ResponsePayload::success().to_response())
}
```

## Compiling And Running

Compile the code like we did for the `rover-cli` tool

```shell
cargo build --bin rover-server --target=arm-unknown-linux-gnueabihf
```

Then upload and run the server

```shell
scp target/arm-unknown-linux-gnueabihf/debug/rover-server alarm@rpizw-rover.local:
ssh -t alarm@rpizw-rover.local sudo RUST_LOG=info ./rover-server
```

Note that with env_logger we can set the log level we want to use by setting the
`RUST_LOG` environment variable. Currently most of the messages are in the `info`
level so we use that to see requests being made.

We can now call the endpoints using curl or rest api explorer like
[postman](https://chrome.google.com/webstore/detail/postman/fhbjgbiflinjbdggehcddcbncdddomop?hl=en).

Try out some of the following.

```shell
curl -XPUT http://rpizw-rover.local:3000/api/speed -d '{"left":100,"right":100}'
#{"success":true}
curl -XPUT http://rpizw-rover.local:3000/api/speed -d '{"left":-100,"right":-100}'
#{"success":true}
curl -XPUT http://rpizw-rover.local:3000/api/stop
#{"success":true}
curl -XPUT http://rpizw-rover.local:3000/api/reset
#{"success":true}
curl -XPUT http://rpizw-rover.local:3000/api/disable
#{"success":true}
curl -XPUT http://rpizw-rover.local:3000/api/speed -d '{"left":100,"right":-100}'
#{"success":true}
curl -XPUT http://rpizw-rover.local:3000/api/enable
#{"success":true}
curl -XPUT http://rpizw-rover.local:3000/api/stop
#{"success":true}
curl -iXPUT http://rpizw-rover.local:3000/api/speed -d '{}'
#HTTP/1.1 400 Bad Request
#Content-Length: 81
#Content-Type: text/plain
#Date: Sun, 19 Mar 2017 12:40:56 GMT
#
#{"success":false,"error":"invalid json: missing field `left` at line 1 column 2"}
```

## The Service File

You can start `rover-server` in the background and detach it from your terminal
by running it with `sudo ./rover-server & disown`. But it is far better to let
our init system (systemd for archlinux) handle this for us by creating a service
file that defines how to start our service. This also allows us to set it to
start on boot so we don't need to log into our rover at all (well, except to
setup the wireless for the moment).

Lets move the server to `/usr/local/bin`

```shell
sudo mv ./rover-server /usr/local/bin/rover-server
```

Then create a service at `src/bin/rover-server.service` with the following
contents (do this locally rather then on the pi so we can include it in the repo
and add it to the image in the next section).

```shell
[Unit]
Description=Rest API for a Raspberry Pi Zero W Rover

[Service]
Environment=RUST_LOG=info
ExecStart=/usr/local/bin/rover-server

[Install]
WantedBy=multi-user.target
```

And copy it to our rover, ssh into the rover and install/start the service.

```shell
scp src/bin/rover-server.service alarm@rpizw-rover.local:
ssh -t alarm@rpizw-rover.local
> sudo cp rover-server.service /etc/systemd/system/rover-server.service
> sudo systemctl daemon-reload
> sudo systemctl start rover-server
> sudo systemctl enable rover-server
```

## Adding The Binary And Service File To Our Image

Once again we must make a small tweak to the `create-image` script to include
our new binary and service file in the images we build. This way they is no
extra setup and will just start running once the pi is booted for the first
time. This is the same process to how we added the `rover-cli` binary, by
including a check for the binary and copying it and the service file to the
mounted image. Below is the diff for these changes.

```diff
diff --git a/create-image b/create-image
index a026d6c..76fe54a 100755
--- a/create-image
+++ b/create-image
@@ -22,6 +22,11 @@ if [ ! -f "target/arm-unknown-linux-gnueabihf/release/rover-cli" ]; then
     exit 1
 fi
 
+if [ ! -f "target/arm-unknown-linux-gnueabihf/release/rover-server" ]; then
+    echo "'target/arm-unknown-linux-gnueabihf/release/rover-server' not found. Have you run 'cargo build --release --target=arm-unknown-linux-gnueabihf'?"
+    exit 1
+fi
+
 # Unmount drives and general cleanup on exit, the trap ensures this will always
 # run execpt in the most extreme cases.
 cleanup() {
@@ -64,6 +69,7 @@ tar -xpf "${rpi_tar}" -C ${mount} 2> >(grep -v "Ignoring unknown extended header
 # Copy our installation script and other artifacts
 install -Dm755 "${script}" "${mount}/tmp/${script}"
 install -Dm755 "target/arm-unknown-linux-gnueabihf/release/rover-cli" "${mount}/usr/local/bin/rover-cli"
+install -Dm755 "target/arm-unknown-linux-gnueabihf/release/rover-server" "${mount}/usr/local/bin/rover-server"
+install -Dm755 "src/bin/rover-server.service" "${mount}/etc/systemd/system/rover-server.service"
 
 # Prep the chroot
 mount -t proc none ${mount}/proc
```

One extra change is needed in the `setup` script to enable our service on boot.

```diff
diff --git a/setup b/setup
index 0c7f4be..5ceb245 100755
--- a/setup
+++ b/setup
@@ -51,6 +51,9 @@ ln -sf /usr/lib/systemd/system/getty@ttyGS0.service /etc/systemd/system/getty.ta
 # Enable hardware pwm
 grep 'dtoverlay=pwm-2chan,pin=12,func=4,pin2=13,func2=4' /boot/config.txt >/dev/null || echo 'dtoverlay=pwm-2chan,pin=12,func=4,pin2=13,func2=4' >> /boot/config.txt
 
+# Enable the rover-server to start on boot
+ln -sf /etc/systemd/system/rover-server.service /etc/systemd/system/multi-user.target.wants/rover-server.service
+
 # Set zsh as the default shell for root and alram
 chsh -s /usr/bin/zsh root
 chsh -s /usr/bin/zsh alarm
```

Now you can build the binary and image by running.

```shell
cargo build --release --target=arm-unknown-linux-gnueabihf
sudo ./create-image
```

Finally you can burn the image to an sd card, boot the pi, setup the wireless,
just like we have previously done. Once booted and connected to the network we
can control the rover through the rest api.

## Conclusion

Although very basic this give a good starting point that we can build upon
later. There are a number of things missing from what we have done so far, most
notability the total lack of authentication and authorisation allowing anyone to
control the rover simply by knowing its ip or hostname. In addition to the basic
access controls the whole application must currently run as root making it
easier for potential attackers to gain root access if there are any bugs in our
program. Not to mention the use of a the default user and password accessible
over ssh (which you really should change on your first login).

Considering this is a simple demo and is still a work in progress these are not
a major concern at the moment but I will be looking to address some of the
security issues in a future post. For now I would just re-image the rover if it
is ever connected to an untrusted network. Thankfully our automated image
creation makes this simple and repeatable as long as we don't live customise the
image too much.

The iron web framework was a major pain point in writing this, mostly due to its
lack of overall documentation and outdated/misleading examples that can be found
online. They have fairly detailed documentation about every bit of the api, but
lack any good examples of how it should all tie in together or was designed to
be used. The other frameworks did not off much of a better alternative in this
regard except perhaps the [rocket framework](https://rocket.rs/), which still
requires rust nightly to compile. Although there is nothing fundamentally wrong
with the rust web ecosystem it is still quite immature and these issues will
hopefully be solved over time.

You can view the final source code on the
[v0.4](https://github.com/mdaffin/rpizw-rover/tree/v0.4) branch or download the
image created from this process
[here](https://github.com/mdaffin/rpizw-rover/releases/tag/v0.4.0).

Now we only have one core component left to write: the front end code that will run
in the browser and communicate with the rest api we developed in this post.