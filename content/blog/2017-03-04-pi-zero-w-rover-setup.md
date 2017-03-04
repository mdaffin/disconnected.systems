+++
date = "2017-03-04T09:38:23Z"
title = "Pi Zero W Rover Setup"
draft = true
description = "Configure the pi zeros usb serial, networking and hardware pwm pins to control a servo wirelessly"
slug = "pi-zero-w-rover-setup"
tags = ["linux", "robot", "serial", "raspberry-pi", "pwm"]
aliases = [
    "/pi-zero-w-rover-setup/",
    "/posts/pi-zero-w-rover-setup/",
]
+++

The raspberry pi zero w has just been released and I managed to get my hands on
one before they sold out. This has sparked me to restart one of my old project
ideas of creating a wireless programmable rover platform. In this first post I
will talk about how to setup the pi and hook it up to the servos, in perticular
how to configure the usb serial and enable and control the hardware pwm pins.

This post will be based off [archlinux arm](https://archlinuxarm.org/) rather
than rasbian as my distro of choice but it should work equally well on both
distros but you will have to tweak some of the commands. For those that want to
follow along with arch you should first follow my [archlinuxarm setup guide]
({{< relref "blog/2016-03-21-raspberry-pi-archlinuxarm-setup.md" >}}) we will be
adding a few steps to the end of the [customisation stage]({{< relref
"blog/2016-03-21-raspberry-pi-archlinuxarm-setup.md#install-and-configure-your-install"
>}}), but they can also be run on a running pi.

```
# Enable the usb serial
grep 'dtoverlay=dwc2' /boot/config.txt >/dev/null || echo 'dtoverlay=dwc2' >> /boot/config.txt
grep 'modules-load=dwc2,g_serial' /boot/config.txt >/dev/null || sed 's/.*/& modules-load=dwc2,g_serial' >> /boot/config.txt
```

```
# Enable hardware pwm
grep 'dtoverlay=pwm-2chan,pin=12,func=4,pin2=13,func2=4' /boot/config.txt >/dev/null || echo 'dtoverlay=pwm-2chan,pin=12,func=4,pin2=13,func2=4' >> /boot/config.txt
```


```
wpa_passphrase "<SSID>" "<PASSPHRASE>" > /etc/wpa_supplicant/wpa_supplicant-wlan0.conf
```

[rpi pinout](https://pinout.xyz/)
[gpio18](https://pinout.xyz/pinout/pin12_gpio18)
[gpio13](https://pinout.xyz/pinout/pin12_gpio13)

[usb serial](https://learn.adafruit.com/turning-your-raspberry-pi-zero-into-a-usb-gadget/serial-gadget)
[hardware pwm](http://librpip.frasersdev.net/peripheral-config/pwm0and1/)

[cleaning up]({{< relref "blog/2016-03-21-raspberry-pi-archlinuxarm-setup.md#cleaning-up" >}})
