---
published: false
---

The archlinuxarm install instructions are only designed for linux, they likley also work on mac but windows users are out of luck. However it is quite simple to use a linux vm to create an `.img` from the root filesystem tar archlinuxarm supplies. This guide is primarly aimed at windows users, but will work just as well in linux or mac. 

## Install Vagrant and VirtualBox

We are going to use VirtualBox and Vagrant to run and manage a linux vm which we will use to create an .img file. You will need to download and install these two programs.

* [virtualbox](https://www.virtualbox.org/wiki/Downloads)
* [vagrant](https://www.vagrantup.com/downloads.html)

Unfortintly I have not found an easy way to pass a usb drive or sdcard into the vm. Instead we are going to create a .img file that we can then use `windd`, `Win32DiskImager` or what ever tool you are use to for writing the image to the sdcard in windows.

~~~
vagrant up
vagrant ssh
cd /vagrant
~~~

When you are done with the vm you can destroy it to free up all resources it used by running:

~~~
vagrant destroy
~~~

