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

When you are done with the vm you should destroy it to free up all resources it used by running:

~~~
vagrant destroy
~~~

## 

First we need to create a file of a specific size, this can be done quickly by creating a sparse file with `fallocate`. A sparse file is one that contains block of empty data (all zeros), rather then storing the block on disk it is just marked as empty in the file metadata. This save space in large files that contain very little data (like img files). Unfortinuatly fallocate will not run in vagrants shared directories so we need to create it in a non shared directory, here we use home (aka `~`). But we want the image avaible to our host so we copy it to `/vagrant` after.

~~~
fallocate -l 1G ~/alarm-$(date +%F).img
cp --sparse=always ~/alarm-*.img .
~~~

Tip: you can see the actual and apparent size of the files with `ls -lsh`, for example.

~~~
$ ls -lsh sparse.img full.img
1.1G -rw-r--r-- 1 vagrant vagrant 1.0G Feb  4 12:52 full.img     <-- non sparse file
   0 -rw-r--r-- 1 vagrant vagrant 1.0G Feb  4 12:52 sparse.img   <-- sparse file
   ^ Actual space used on disk       ^ Apparent size of the file
~~~
