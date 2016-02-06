---
layout: post
title: Raspberry Powered Wallboard
description: A guide on how to setup a raspberry pi to use as a wallboard that once booted automatically loads a webpage.
tags: [raspberry-pi, archlinuxarm, archlinux, linux]
---

Having information displayed on a wall or monitor in an office or other public
space can be very useful to the people in that area. This post talks about how
you can use a raspberry pi and any old monitor or tv to display an arbitrary
webpage or application on it.

<!--more-->

## Setup

We are going to start with is a up-to-date ArchLinux image, read this if you
don't know how to install ArchLinux to an sd card. Boot the image from the
Raspberry Pi and login as root (the password is root as well). As with any
install the very first thing you should do is update the system:

    # pacman -Syu

## The User

In keeping with best security practices its best to run things that don't need
to run as root as a normal user. Create a new user called "wb" with:

    # useradd -m wb
    # passwd wb
    Enter new UNIX password: <type password here>
    Retype new UNIX password: <retype password here>
    passwd: password updated successfully

Now logout of root and login as the new user. When you need to run a command as
root now you can login as root from another tty or you can run a single command
as root with:

    $ su root command

Note that any command in this post that start with a # should be run as root,
and any command that start with a $ should be run as the normal user.

## XOrg

We are going to need xorg in order to run graphical programs so now is a good
time to install it along with a window manager of your choice:

    # pacman -S xorg-server xorg-xinit xorg-xset xterm ratpoison

I choose to use ratpoison as the window manager as it has no boarders and
displays all windows fully maximized by default. Its controls are a little weird
in that you interact with it similar to screen (press ctrl+t,? to display the
help page if you want to know the key bindings) but since we only want to launch
one program this should not matter.

To tell xinit what to do when a user tried to start a xorg session you need to
edit ~/.xinitrc of that user, so edit /home/wb/.xinitrc with the following
contents:

    setterm -blank 0 -powersave off -powerdown 0
    xset -dpms
    xset s off
    exec ratpoison &
    exec xterm

This will cause ratpoison to open in the background followed by xterm when the
user starts an xorg-session. Normally you would start the programs you want
first in the background then the window manager so that when you close the
window manager the xorg-session ends. But in this situation we don't care about
the window manager and would like to stop the session when our application stop
(or crashes) as this will allow us to restart it automatically.

It also turns off the automatic screen power saving and blanking features which
we don't want on a wallboard.

To test if this works run:

    $ startx

and you should get a xterm window appear and xorg should stop if you close it
(crtl+D or ctrl+T,K).

## The Web Browser

The common browsers, Firefox and chromium are sadly not available in the
ArchLinuxArm repositories so an alternative must be used. In the end I decided
to use the uzbl browser, a lightweight browser that sticks to the unix
philosophy of one application one task. Uzbl is mostly keyboard driven and
doesn't have lots of menus so makes it ideal for a wall board.

To install it run:

    # pacman -S uzbl-browser

There is also a uzbl-tabbled alternative if you require tab support, but this
shouldn't be needed for a wallboard.

Once installed you can replace the

    exec xterm

line in /home/wb/.xinitrc with

    exec uzbl-browser http://www.example.com

to open uzbl to the given site when x starts.

You should now see a web browser start pointing to www.example.com when the wb
user runs startx. Remember to change the url to where you want it to point.

If you are interested the key bindings for uzbl can be found
[here](http://uzbl.org/keybindings.php).

Note that you could replace the browser with any other browser you like, or even
with an entirely different application.

## Running It All At Startup

The final step is to get everything to startup when the computer boots.
ArchLinux now uses systemd by default so we need to create a new service file to
auto login to the wb user. Create a copy the getty@.service and place it in
/etc/systemd/system:

    cp /usr/lib/systemd/system/getty@.service /etc/systemd/system/autologin@.service

now edit the following parts to make it auto login to the wb user:

    [Service]
    Type=simple
    [...]
    ExecStart=-/sbin/agetty --noclear -a wb %I 38400
    [...]
    [Install]
    Alias=getty.target.wants/getty@_tty1_.service

Change tty1 if you want to login to another tty. See
[this](https://wiki.archlinux.org/index.php/Automatic_login_to_virtual_console#With_systemd)
for more info about editing the service file to your needs. Now set this service
to run at boot by running:

    # systemctl daemon-reload
    # systemctl disable getty@tty1.service
    # systemctl enable autologin@tty1.service

you can start it now to test it by running:

    # systemctl stop getty@tty1.service
    # systemctl start autologin@tty1.service

Note that you will be logged out of tty1 if you run this so best to do this from
another tty.

Lastly we need to start x when the wb user logs in. This can be done by editing
their .bash_profile file and add the following to the end:

    startx
    logout

The logout is there so that they automatically get logged out when the xorg
session stops and allows systemd to restart the getty, which will automatically
log them back in. This is very useful should the program crash at all as it will
be started again soon after but has the downside of making it harder to login as
the wb user manually.

## Conclusion

You can now reboot the raspberry pi

    # reboot

and it should come back displaying the webpage or application you picked. The
only downside to this is that it will not auto refresh the page, which is ok if
you have an ajax updated page, or one that auto refreshes its self, but not for
static pages that could update behind the scenes. If anyone is interested I
could look into how to make uzbl auto refresh, but for now this is good enough.
