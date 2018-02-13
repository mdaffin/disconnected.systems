+++
date = 2017-03-13T17:37:43Z
title = "Using Rust to Control a Raspberry Pi Zero W Rover"
draft = false
description = "Creates a simple program in rust to control the raspberry pi zero w rover we have been building"
slug = "rust-powered-rover"
tags = ["rust", "raspberry-pi"]
+++

Over the past few weeks I have been starting to build a raspberry pi zero w
controlled rover.

* [Pi Zero W Rover Setup](/blog/pi-zero-w-rover-setup)
* [Customising Raspberry Pi Images with Github and Travis](/blog/custom-rpi-image-with-github-travis)

In this post I am going build upon these posts to look at setting up a rust
project to control the rover which will be built on in later posts. Although you
can compile rust programs directly on the pi we are going to look at cross
compiling rust for the raspberry pi zero as it is much easer to develop and much
faster to compile on a more powerful system. The program we will create is a
simple command line tool to drive the rover. Finally we will look at modifying
the image building scripts we wrote in the last post to build and include our
binary in the image.

I am going to continue with the repo we created in the last blog post, if you
dont want to follow on from that, or if you want to start from the same base you
can clone it by running 

```sh
git clone https://github.com/mdaffin/rpizw-rover.git -b v0.1
cd ripzw-rover
```

## Setting Up Rust For Cross Compiling

To cross compile to arm on rust we require the arm linker which can by running

```sh
apt-get install -qq gcc-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross
```

on ubuntu or from the aur package
[`gcc-arm-linux-gnueabihf`](https://aur.archlinux.org/packages/arm-linux-gnueabihf-gcc/).

Once installed we also need to install rust and the arm target for rust.

```sh
curl https://sh.rustup.rs -sSf | sh # Or install from your package manager
rustup default stable
rustup target add arm-unknown-linux-gnueabihf
```

Note that this is for ARMv6 devices (aka, the raspberry pi 1 and zero). For
raspberry pi 2/3 use `armv7-unknown-linux-gnueabihf` in place of the above
target though out this post.

We need to tell rust about the linker we want to use for the
`arm-unknown-linux-gnueabihf` target. This can be done by creating
`.cargo/config` in the root of our project with the following contents.

```sh
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

## Cross Compile Hello World

Now lets us setup a simple hello world application to see if we have our
environment setup correctly. Inside the we created last time, or that you cloned
at the start of this post, create a `Cargo.toml` file with the following
contents.

```ini
[package]
name = "rpizw-rover"
version = "0.2.0"
```

Then create `src/main.rs` with the following contents.

```rust
fn main() {
    println!("Hello World!");
}
```

You can compile it natively and run it locally with to ensure rust is installed
correctly.

```sh
cargo run
#    Compiling rpizw-rover v0.1.0 (file:///home/mdaffin/projects/test)
#     Finished debug [unoptimized + debuginfo] target(s) in 0.38 secs
#      Running `target/debug/rpizw-rover`
# Hello World!
```

And cross compile and upload it to then run it on the pi with.

```sh
cargo build --target=arm-unknown-linux-gnueabihf
#    Compiling rpizw-rover v0.2.0 (file:///home/mdaffin/projects/test)
#     Finished debug [unoptimized + debuginfo] target(s) in 0.43 secs
scp target/arm-unknown-linux-gnueabihf/debug/rpizw-rover alarm@rpizw-rover.local: # or the ip address of your pi
ssh -t alarm@rpizw-rover.local ./rpizw-rover
# Hello World!
# Connection to rpizw-rover.local closed.
```
If you are having trouble with these steps have a look at this more
comprehensive guide on [cross compiling
rust](https://github.com/japaric/rust-cross). Alternatively you can install rust
on the pi and compile it natively, however this tends to be much slower for
larger projects.

Congratulations you can now cross compile for the raspberry pi in rust. 

## Dependencies

The three dependencies we are going to use are

* [sysfs-pwm](https://github.com/rust-embedded/rust-sysfs-pwm): for talking to the linux pwm sysfs
* [error-chain](https://github.com/brson/error-chain): to avoid some boiler plate on error handling
* [clap](https://clap.rs/): to parse the command line arguments

Append the following to the `Cargo.toml` we created above to add these
dependencies to our project.

```ini
[dependencies]
sysfs-pwm = "0.1.0"
error-chain = "0.10.0"
clap = {version = "2.20.5", features = ["yaml"]}
```

## Handling Errors

Dealing with errors is important in any program, so lets setup some of out error
handling code in preparation for later. Create the file `src/error.rs` with the
following contents.

```rust
use sysfs_pwm;

error_chain!{
    foreign_links {
        PWM(sysfs_pwm::Error);
    }
}
```

This macro will create a `Result`, `Error` types as well as some other useful
structs. It will also wrap the `sysfs_pwm::Error` in the `ErrorKind` enum
allowing us to seamlessly convert errors from it to our types and to match on it
later if required.

All errors are going to be propagated up to our main function, which will
handle them by printing out a nice error message to the user explaining what
failed and why. Replace `src/main.rs` with the following.

```rust
// The dependencies we are going to use.
extern crate sysfs_pwm;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;

// Include our error module so we can use it.
mod error;

// Import everything from our error module.
use error::*;

// A stub function we will implement our application logic in later.
fn run() -> Result<()> {
    bail!("Not yet implemented")
}

fn main() {
    // Run the run function and print any errors that it returns.
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        // Error message for when we cannot write to stderr
        let errmsg = "Error writing to stderr";

        // Print out the error that occurred.
        writeln!(stderr, "error: {}", e).expect(errmsg);

        // And what caused it.
        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // As well as any backtrace if they are enabled.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
```

You can learn more about error_chain at [this
post](http://brson.github.io/2016/11/30/starting-with-error-chain) or their on
[their repo](https://github.com/brson/error-chain).

## The Rover Module

The rovers code can be encapsulated into a module that we will be able to easily
reuse later and allow more complex applications to be built around it. Create
`src/rover.rs` with the following contents. Note that we make use of our error
module and that any function that can return an error does so with the `Result`
defined there. This gives us a consistent error type thought out our project.

```rust
use error::*;
use sysfs_pwm::Pwm;

const PERIOD: u32 = 20_000_000;
const MAX_DUTY_CYCLE: u32 = 2_000_000;
const MIN_DUTY_CYCLE: u32 = 1_000_000;

// Holds the left and right motor that the functions below will act upon.
pub struct Rover {
    left: Pwm,
    right: Pwm,
}

impl Rover {
    // Creates a new rovers with both motors ready to be enabled. The motors
    // will be disabled and the underlying pwm drivers unexported when the
    // rover is dropped. `enable(true)` must be called before the motor will
    // move.
    pub fn new(chip: u32, left_pin: u32, right_pin: u32) -> Result<Rover> {
        let left = Pwm::new(chip, left_pin).chain_err(|| "failed to create left motor")?;
        let right = Pwm::new(chip, right_pin).chain_err(|| "failed to create right motor")?;
        left.export().chain_err(|| "failed to export the left motor pwm channel")?;
        right.export().chain_err(|| "failed to export the right motor pwm channel")?;
        left.set_period_ns(PERIOD).chain_err(|| "failed to set period on left motor")?;
        right.set_period_ns(PERIOD).chain_err(|| "failed to set period on right motor")?;
        Ok(Rover {
            left: left,
            right: right,
        })
    }

    // Enables/disables the motor. When disabled they keep their current
    // speed and their speed can still be set but they will not move until
    // enabled.
    pub fn enable(&self, enabled: bool) -> Result<()> {
        self.left.enable(enabled).chain_err(|| "failed to enable left motor")?;
        self.right.enable(enabled).chain_err(|| "failed to enable right motor")
    }

    // Converts a speed between -100 (full reverse) and 100 (full forward)
    // to a duty cycle which we can pass to the Pwm struct from sysfs_pwm.
    // The idea is to map values from -100, 100 to 1_000_000, 2_000_000 where
    // 0 is 1500000 (the neutral point for servos). It also caps the return
    // value to be within this range.
    fn speed_to_duty_cycle(speed: i8) -> u32 {
        let duty_cycle = (((speed as i32 * 10000) + MIN_DUTY_CYCLE as i32) as u32 / 2) +
                         MIN_DUTY_CYCLE;
        if duty_cycle > MAX_DUTY_CYCLE {
            return MAX_DUTY_CYCLE;
        }
        if duty_cycle < MIN_DUTY_CYCLE {
            return MIN_DUTY_CYCLE;
        }
        duty_cycle
    }

    // Sets the speed of the left motor. Can be any value between -100 (full
    // reverse) and 100 (full forward), values above or below these limits will
    // be to to the limit.
    pub fn set_left_speed(&self, speed: i8) -> Result<()> {
        self.left
            .set_duty_cycle_ns(Rover::speed_to_duty_cycle(-speed))
            .chain_err(|| "failed to set duty on left motor")
    }

    // Sets the speed of the right motor. Can be any value between -100 (full
    // reverse) and 100 (full forward), values above or below these limits will
    // be to to the limit.
    pub fn set_right_speed(&self, speed: i8) -> Result<()> {
        self.right
            .set_duty_cycle_ns(Rover::speed_to_duty_cycle(speed))
            .chain_err(|| "failed to set duty on left motor")
    }

    // Stops both the motors, equlivent to setting their speeds to 0.
    pub fn stop(&self) -> Result<()> {
        self.set_left_speed(0)?;
        self.set_right_speed(0)
    }

    // Sets the speed of left and right motor. Can be any value between -100 (full
    // reverse) and 100 (full forward), values above or below these limits will
    // be to to the limit.
    pub fn set_speed(&self, left: i8, right: i8) -> Result<()> {
        self.set_left_speed(left)?;
        self.set_right_speed(right)
    }

    // Unexports the motors so they can no longer be used. Note that we use
    // `self` rather than `&self` as we want this function to consume the
    // rover stopping any future calls to it (which will cause a compile time
    // error)
    pub fn unexport(self) -> Result<()> {
        self.left.enable(false).chain_err(|| "failed to disable left motor")?;
        self.right.enable(false).chain_err(|| "failed to disable right motor")?;
        self.left.unexport().chain_err(|| "failed to unexport left motor")?;
        self.right.unexport().chain_err(|| "failed to unexport right motor")
    }
}
```

## Handling Cli Arguments

We are going to use [clap](https://clap.rs) to handle our command line
arguments. We are going to use its yaml feature to allow us to define the
arguments in a separate file keeping our `src/main.rs` cleaner. Create
`src/cli.yml` with the following contents.

```yaml
name: rpizw-rover
about: Controls a raspberry pi zero powered rover
subcommands:
    - stop:
        about: stops the rover
    - unexport:
        about: unexports the underlying pwm hardware interface so it can be used by other programs
    - enable:
        about: enables the motors
    - disable:
        about: disables the motors
    - speed:
        about: sets the speed of the rover
        args:
            - dont-enable:
                short: d
                long: dont-enable
                help: dont enable the motors after setting the speed
            - LEFT:
                required: true
                index: 1
                help: sets the left motor speed
            - RIGHT:
                required: false
                index: 2
                help: sets the right motor speed
```

As you can see from this we have five subcommands, `stop`, `unexport`, `enable`,
`disable`, and `speed` where `speed` takes one or two arguments and an optional
flag (if the right argument is not set then we will use the given argument for
both left and right speeds). These commands correspond to the functions on our
`rover::Rover` defined in the last section. We can encapsulate this behavior by
making the following changes to the run function.

```rust
mod rover;

const PWM_CHIP: u32 = 0;
const LEFT_PWM: u32 = 0;
const RIGHT_PWM: u32 = 1;

fn run() -> Result<()> {
    use clap::App;
    use rover::Rover;

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    let rover = Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM)?;

    // find out which subcommand was given, get_matches will print a help
    // message and exit if an unknown command is given.
    if let Some(_) = matches.subcommand_matches("disable") {
        rover.enable(false)
    } else if let Some(_) = matches.subcommand_matches("enable") {
        rover.enable(true)
    } else if let Some(_) = matches.subcommand_matches("stop") {
        rover.stop()
    } else if let Some(matches) = matches.subcommand_matches("speed") {
        // left is required so it will always be set here, otherwise
        // get_matches above will print a help message and exit.
        let left = matches.value_of("LEFT").unwrap();
        // if right is not set then use the left value
        let right = matches.value_of("RIGHT").unwrap_or(left);
        // parse the values into i8s and return an error if this fails.
        let left: i8 = left.parse::<i8>().chain_err(|| "failed to parse left speed")?;
        let right: i8 = right.parse::<i8>().chain_err(|| "failed to parse right speed")?;

        rover.set_speed(left, right)?;
        if !matches.is_present("dont-enable") {
            rover.enable(true)?;
        }
        Ok(())
    } else if let Some(_) = matches.subcommand_matches("unexport") {
        rover.unexport()
    } else {
        // If no command was specified print the help message
        println!("{}", matches.usage());
        Ok(())
    }
}
```

Now we can build and upload it to the pi by running:

```sh
cargo build --target=arm-unknown-linux-gnueabihf
scp target/arm-unknown-linux-gnueabihf/debug/rpizw-rover alarm@rpizw-rover.local:
```

Then ssh to the pi and try out the command. Note that the `--` is necessary to
stop clap from interpreting negative numbers as flags.

```sh
sudo ./rpizw-rover speed 100 100
sleep 1
sudo ./rpizw-rover speed -- -100 -100
sleep 1
sudo ./rpizw-rover stop
sleep 1
sudo ./rpizw-rover speed -- 100 -100
sleep 1
sudo ./rpizw-rover speed -- -100 100
sleep 1
sudo ./rpizw-rover speed -- -15 85
sleep 2
sudo ./rpizw-rover stop
sleep 1
sudo ./rpizw-rover unexport
```

## Updating `create-image` and the Travis config

Now we have a working program we should update the `create-image` script and
`.travis.yml` to include it inside our image. We are not going to build the
binary in the `create-image` script as it needs to run as root and we don't want
to build our program as root. So lets add a check to bail out early if it has
not already been build. In `create-image`, around line 18 add the following just
before the cleanup function.

```sh
...

rpi_tar="ArchLinuxARM-rpi-latest.tar.gz"
rpi_url="http://archlinuxarm.org/os/${rpi_tar}"

# Check to see if the binary has been built, we check this first to we can bail early.
if [ ! -f "target/arm-unknown-linux-gnueabihf/release/rpizw-rover" ]; then
    echo "'target/arm-unknown-linux-gnueabihf/release/rpizw-rover' not found. Have you run 'cargo build --release --target=arm-unknown-linux-gnueabihf'?"
    exit 1
fi

# Unmount drives and general cleanup on exit, the trap ensures this will always
# run except in the most extreme cases.
cleanup() {
    [[ -f "${mount}/tmp/${script}" ]] && rm "${mount}/tmp/${script}"

...
```

Then add the following install command just after we install the script

```sh
...

# Copy our installation script and other artifacts
install -Dm755 "${script}" "${mount}/tmp/${script}"

install -Dm755 "target/arm-unknown-linux-gnueabihf/release/rpizw-rover" "${mount}/usr/local/bin/rpizw-rover"

# Prep the chroot
mount -t proc none ${mount}/proc

...
```

While we are at it we can remove our old `rover-test.sh` from the image and
repo. Remove the following line from `create-image` then delete the
`rover-test.sh` script from the repo.

```sh
cp rover-test.sh ${mount}/home/alarm/rover-test.sh
```

In the `.travis.yml` we need to add some extra dependencies as well as setup rust before building our binary. Add the following to the different bits in `.travis.yml` (some lines left in for context).

```yaml
...
addons:
  apt:
    packages:
...
    - gcc-arm-linux-gnueabihf
    - libc6-armhf-cross
    - libc6-dev-armhf-cross
install:
- export PATH="$PATH:$HOME/.cargo/bin"
- curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=stable
- rustup target add arm-unknown-linux-gnueabihf
script:
- cargo build --release --target arm-unknown-linux-gnueabihf
- sudo ./create-image
...
```

Once you commit and push these changes travis should start building. We can then
tag our next version to cause travis to build and publish the ready to go images.

## Conclusion

We have introduced another building block to the rover platform, we can now
cross compile rust programs and embed them in our raspberry pi images and have a
basic module to control our rover. The next step is a to build basic web server
that uses what we have done here to allow us to finally remotely control the
rover through an web interface.
