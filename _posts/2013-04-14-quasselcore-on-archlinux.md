---
layout: post
title: QuasselCore on Archlinux
description: A guide to setting up a quassel server on Arch Linux server.
---

I just set up a quassel server on an Arch Linux box. There is a serious lack of documentation around
it, though it is simple enough to set up I stumbled around a bit wondering why I could not connect
and what I had done wrong so this post is about what I did to get it to work.

<!--more-->

My aim was to set up a quassel core running on a headless server, so no need for the client or any
of the GUI. The quassel package in Arch Linux contains both the client and the core and pulls in far
to many dependencies for a headless server. Lucky there is a
[quasselcore](https://aur.archlinux.org/packages.php?ID=42085) package in AUR that fits my needs
perfectly. Installation is simple enough, use packer, yaourt or simply download and build the
package manually:

    wget https://aur.archlinux.org/packages/qu/quasselcore/quasselcore.tar.gz
    tar -xf quasselcore.tar.gz
    cd quasselcore
    makepkg -si

Unfortunately by default quasselcore on Arch Linux only lessens to localhost by default. To change
this edit /etc/conf.d/quassel to the following

    QUASSEL_USER=quassel
    LISTEN=0.0.0.0

This allows quassel to listen to any ip address.

Now just start the service and set it to run on boot

    systemctl enable quassel
    systemctl start quassel

And that is it on the server! Initial configuration is all done on the first connection from the
client, no need to set up a username or password beforehand  And on this note you will want to do
this quickly after starting it and preferably before exposing it to the internet.

To start the first run wizard just launch quasselclient on you desktop and add a new host with the
following details

**Account name:** Any name you wish to give the core (for the client only)  
**Hostname:** The hostname or ip of the server  
**Port:** The default is 4242, use that unless you have changed it  
**User:** The name of the user you want to connect via (Might not matter on the first connection)  
**Password:** The password of that user (does not matter on the first connection)  

and click ok to connect. The first run wizard should now launch, just follow it through to set up
the admin user that you will use from now on to connect to the quassel server.

## Enable SSL

This is important if you wish the communication between the core and the client to be encrypted, but
is optional if you are just using it over a local network. The quasselcore package already has ssl
enabled in the build script, so to enable it all you need to do is generate the certificates:

    sudo -u quassel openssl req -x509 -nodes -days 365 -newkey rsa:1024 -keyout ~quassel/quasselCert.pem -out ~quassel/quasselCert.pem

and then restart quassel:

    systemctl restart quassel

## Add additional users

After set up you can add new users by logging into the server and running `sudo quasselcore
--configdir=/var/lib/quassel --add-user` and filling in the appropriate information when prompted.
Note that there is no easy way to delete a user once added.

You can change a forgotten password by running `sudo quasselcore --configdir=/var/lib/quassel
--change-userpass=USERNAME`
