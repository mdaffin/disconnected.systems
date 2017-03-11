+++
date = "2017-03-13T23:21:07Z"
title = "Using Rust to Control a Raspberry Pi Zero Rover"
draft = true
description = ""
slug = "custom-rpi-image-with-github-travis"
tags = ["linux", "raspberry-pi", "rover", "rust"]
+++


```
[package]
name = "rpizw-rover"
version = "0.1.0"

[dependencies]
sysfs-pwm = "0.1.0"
```


```
extern crate sysfs_pwm;
use sysfs_pwm::{Pwm, Result};
use std::{thread, time};

const PWM_CHIP: u32 = 0;
const LEFT_PWM: u32 = 0;
const RIGHT_PWM: u32 = 1;

fn main() {
    let left = Pwm::new(PWM_CHIP, LEFT_PWM).unwrap(); // number depends on chip, etc.
    let right = Pwm::new(PWM_CHIP, RIGHT_PWM).unwrap(); // number depends on chip, etc.
    left.export().unwrap();
    right.export().unwrap();
    left.set_period_ns(20_000_000).unwrap();
    right.set_period_ns(20_000_000).unwrap();
    left.set_duty_cycle_ns(1_500_000).unwrap();
    right.set_duty_cycle_ns(1_500_000).unwrap();
    left.enable(true).unwrap();
    right.enable(true).unwrap();
    left.set_duty_cycle_ns(1_000_000).unwrap();
    right.set_duty_cycle_ns(2_000_000).unwrap();
    thread::sleep(time::Duration::from_secs(5));
    left.enable(false).unwrap();
    right.enable(false).unwrap();
    left.unexport().unwrap();
    right.unexport().unwrap();
}
```