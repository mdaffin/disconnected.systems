+++
date = "2017-03-18T21:45:01Z"
title = "Small Refactor To Prepare For Writing The Rest API"
draft = false
description = "Small tweaks to the rovers code and overall project layout in preparation for adding a webserver that can control the rover."
slug = "rover-refactor"
tags = ["rust", "raspberry-pi"]
+++

Over the past few weeks I have been starting to build a raspberry pi zero w
based rover.

* [Pi Zero W Rover Setup]({{< relref "blog/2017-03-08-pi-zero-w-rover-setup.md" >}})
* [Customising Raspberry Pi Images with Github and Travis]({{< relref "blog/2017-03-10-custom-rpi-image-with-github-travis.md" >}})
* [Using Rust to Control a Raspberry Pi Zero W Rover]({{< relref "blog/2017-03-13-rust-powered-rover.md" >}})

In my last post I looked at writing a simple cli tool in rust for the rover.
Next I wanted to start building a rest api able to control the rover but before
I do there are a few things that need to be changed. I decided to separate it
out into another post to keep the next one more focused. You can grab the
changes made and skip this post by cloning the
[v0.3](https://github.com/mdaffin/rpizw-rover/tree/v0.3) branch or follow on
from the previous post which ended on the
[v0.2](https://github.com/mdaffin/rpizw-rover/tree/v0.2) branch.

We will look at separating out the rover code into a library and moving the
`main.rs` into a named binary. This will allow us to keep the rover cli tool
while developing the rest api along side it. We will also look at refactoring
the rover code to give us more control over the set up and tear down code.
Finally we fix an unrelated upstream bug with our image creation.

## Creating The Library

Turning our code into a library is trivial. All we need to do is create a file
called `src/lib.rs`, include any crates we use and export everything we want to
be available. In our case this is just the rover and error modules.

```rust
extern crate sysfs_pwm;
#[macro_use]
extern crate error_chain;

pub mod error;
pub mod rover;

pub use rover::Rover;
```

We also export the `rover::Rover` as a convenience allowing us to use `rpizw_rover::Rover` instead of `rpizw_rover::rover::Rover`.

## Moving The Binary

The default rust binary for a crate is located at `src/main.rs` just like ours
currently is. But you can have additional binaries, or include binaries along
side a library, like we want to do, by placing them in `src/bin/<bin_name>.rs`.
You can have as many binaries in there as you wish in a single crate by doing
this. Lets rename our binary to `rover-cli` by moving it to
`src/bin/rover-cli.rs`. We also need to move `cli.yml` to the same place.

```shell
mkdir src/bin
mv src/main.rs src/bin/rover-cli.rs
mv src/cli.yml src/bin/cli.yml
```

And change the `name: rpizw-rover` line in `src/bin/cli.yml` to `name:
rover-cli`.

Then add `extern crate rpizw_rover;` to the top of the `src/bin/rover-cli.rs` as
it is now effectively an external library as far as the binary is concerned.
This means we also need to remove the mod lines and prefix the uses of our
modules with `rpizw_rover::`. We can also remove the crates that we no longer
directly use. Below is the diff of these changes.

```diff
diff --git a/src/main.rs b/src/bin/rover-cli.rs
similarity index 93%
rename from src/main.rs
rename to src/bin/rover-cli.rs
index 596d005..f0cafaf 100644
--- a/src/main.rs
+++ b/src/bin/rover-cli.rs
@@ -1,13 +1,8 @@
-extern crate sysfs_pwm;
-#[macro_use]
-extern crate error_chain;
+extern crate rpizw_rover;
 #[macro_use]
 extern crate clap;
 
-mod error;
-mod rover;
-
-use error::*;
+use rpizw_rover::error::*;
 
 const PWM_CHIP: u32 = 0;
 const LEFT_PWM: u32 = 0;
@@ -15,7 +10,7 @@ const RIGHT_PWM: u32 = 1;
 
 fn run() -> Result<()> {
     use clap::App;
-    use rover::Rover;
+    use rpizw_rover::Rover;
 
     let yaml = load_yaml!("cli.yml");
     let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
```

To build all of the binaries in our project just run `cargo build` or with the
`--target ...` to cross compile them. To build a specific binary run `cargo
build --bin <bin_name>`. So to cross compile our rover-cli bin for the pi we can
run.

```shell
cargo build --bin rover-cli --target arm-unknown-linux-gnueabihf
```

## The Refactor

While building the webserver I realised I wanted more control over exporting and
unexporting the pwm modules. So I moved the export/set_period calls to their own
`export` function and removed the disable calls from the `unexport` function. I
also made the unexport function take a reference so it does not consume the rover
struct that called it.

```diff
diff --git a/src/rover.rs b/src/rover.rs
index 3396e2d..b3beffb 100644
--- a/src/rover.rs
+++ b/src/rover.rs
@@ -17,16 +17,20 @@ impl Rover {
     pub fn new(chip: u32, left_pin: u32, right_pin: u32) -> Result<Rover> {
         let left = Pwm::new(chip, left_pin).chain_err(|| "failed to create left motor")?;
         let right = Pwm::new(chip, right_pin).chain_err(|| "failed to create right motor")?;
-        left.export().chain_err(|| "failed to export the left motor pwm channel")?;
-        right.export().chain_err(|| "failed to export the right motor pwm channel")?;
-        left.set_period_ns(PERIOD).chain_err(|| "failed to set period on left motor")?;
-        right.set_period_ns(PERIOD).chain_err(|| "failed to set period on right motor")?;
         Ok(Rover {
             left: left,
             right: right,
         })
     }
 
+    /// Exports and setup the period for the servos.
+    pub fn export(&self) -> Result<()> {
+        self.left.export().chain_err(|| "failed to export the left motor pwm channel")?;
+        self.right.export().chain_err(|| "failed to export the right motor pwm channel")?;
+        self.left.set_period_ns(PERIOD).chain_err(|| "failed to set period on left motor")?;
+        self.right.set_period_ns(PERIOD).chain_err(|| "failed to set period on right motor")
+    }
+
     /// Enables/disables the motor. When disabled they keep their current
     /// speed and their speed can still be set but they will not move until
     /// enabled.
@@ -82,10 +86,8 @@ impl Rover {
     }
 
     /// Unexports the motors so they can no longer be used
-    pub fn unexport(self) -> Result<()> {
-        self.left.enable(false).chain_err(|| "failed to disable left motor")?;
-        self.right.enable(false).chain_err(|| "failed to disable right motor")?;
+    pub fn unexport(&self) -> Result<()> {
         self.left.unexport().chain_err(|| "failed to unexport left motor")?;
         self.right.unexport().chain_err(|| "failed to unexport right motor")
     }
```

Now we need to add the calls to export and disable in the relevant places in our
cli tool, basically after the creation of the rover and before it is unexported.

```diff
diff --git a/src/bin/rover-cli.rs b/src/bin/rover-cli.rs
index f0cafaf..bf6a5fa 100644
--- a/src/bin/rover-cli.rs
+++ b/src/bin/rover-cli.rs
@@ -16,6 +16,7 @@ fn run() -> Result<()> {
     let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
 
     let rover = Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM)?;
+    rover.export()?;
 
     if let Some(_) = matches.subcommand_matches("disable") {
         rover.enable(false)
@@ -36,6 +37,7 @@ fn run() -> Result<()> {
         Ok(())
 
     } else if let Some(_) = matches.subcommand_matches("unexport") {
+        rover.enable(false)?;
         rover.unexport()
     } else {
         println!("{}", matches.usage());
```

## Fixing The Build Tools

Lastly we have renamed or binary from `rpizw-rover` to `rover-cli` so we must
reflect this change in the `create-image` script.

```diff
diff --git a/create-image b/create-image
index 5d8540c..9c350e8 100755
--- a/create-image
+++ b/create-image
@@ -17,8 +17,8 @@ rpi_tar="ArchLinuxARM-rpi-latest.tar.gz"
 rpi_url="http://archlinuxarm.org/os/${rpi_tar}"
 
 # Check to see if the binary has been built, we check this first to we can bail early.
-if [ ! -f "target/arm-unknown-linux-gnueabihf/release/rpizw-rover" ]; then
-    echo "'target/arm-unknown-linux-gnueabihf/release/rpizw-rover' not found. Have you run 'cargo build --release --target=arm-unknown-linux-gnueabihf'?"
+if [ ! -f "target/arm-unknown-linux-gnueabihf/release/rover-cli" ]; then
+    echo "'target/arm-unknown-linux-gnueabihf/release/rover-cli' not found. Have you run 'cargo build --release --target=arm-unknown-linux-gnueabihf'?"
     exit 1
 fi
 
@@ -63,7 +63,7 @@ tar -xpf "${rpi_tar}" -C ${mount} 2> >(grep -v "Ignoring unknown extended header
 
 # Copy our installation script and other artifacts
 install -Dm755 "${script}" "${mount}/tmp/${script}"
-install -Dm755 "target/arm-unknown-linux-gnueabihf/release/rpizw-rover" "${mount}/usr/local/bin/rpizw-rover"
+install -Dm755 "target/arm-unknown-linux-gnueabihf/release/rover-cli" "${mount}/usr/local/bin/rover-cli"
 
 # Prep the chroot
 mount -t proc none ${mount}/proc
```

## Fixing The `ca-certificates-utils` Update

There has been a recent change to the upstream package `ca-certificates-utils`
which is detailed
[here](https://www.archlinux.org/news/ca-certificates-utils-20170307-1-upgrade-requires-manual-intervention/).
Until the upstream archlinuxarm update their rootfs we must make a small change
to our `setup` script to stop the initial update from breaking. Add the following
to the top of `setup` just before the first `pacman -Syu ...`.

```shell
# Fix for a recent change in ca-certificates-utils this can be removed once upstream rootfs has been update.
# https://www.archlinux.org/news/ca-certificates-utils-20170307-1-upgrade-requires-manual-intervention/
pacman -Syuw
rm /etc/ssl/certs/ca-certificates.crt
pacman -Su
```

Note that you may not require this fix for long, it is just needed until
archlinuxarm updates their rootfs images that we download in the `create-image`
script and this might cause issues after they do (at which point we can simply
remove the above lines).

## Conclusion

We only did some minor tweaks in preparation for building the rovers rest api
but will allow me to focus on that in the next post without also worrying about
these minor changes. In my next post we will take a look at using the iron web
framework to build a simple rest api around our rover module.