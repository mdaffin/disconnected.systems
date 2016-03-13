---
layout: post
title: Create Custom Raspberry Pi ArchlinuxArm Images
description: Setup a raspberry pi archlinuxarm image or sd card without booting a pi.
tags: [raspberry pi, archlinuxarm, arm, automated]
---

<!--more-->

## Prerequisites

### Linux

The following packages need to be installed

* `qemu`
* `qemu-user-static`
* `binfmt-support`

Alternative you can make use of vagrant/virtualbox by following the windows/mac
instructions below to create a linux vm with everything preinstalled.

### Windows and Mac

This guide requires a linux install to be able to chroot into the raspberry pi
image. Windows and mac users can follow this guide by using linux in a vm. I
recommend using [virtualbox](https://www.virtualbox.org/wiki/Downloads) and
[vagrant](https://www.vagrantup.com/downloads.html) for this task with
the following vagrant file.

<figure>
  <figcaption>Vagrantfile</figcaption>
{% highlight ruby %}
# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/trusty64"
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update -y
    apt-get install -y qemu qemu-user-static binfmt-support
  SHELL
end
{% endhighlight %}
</figure>

Then open a shell/terminal/command prompt where you saved the `Vagrantfile` and
run the following to download/boot and ssh into a linux vm with everything you
need preinstalled. Note that this can take several minutes to complete.

{% highlight bash %}
vagrant up
vagrant ssh
{% endhighlight %}

When you are done remember to exit the ssh session and destroy the vm to free up
any resources it used with the following. `/vagrant` is a shared folder between
the vm and your host, any file inside it will appear in both which means it a
convenient way to copy files (such as the image file) between the linux guest
and your host.

{% highlight bash %}
vagrant destroy
{% endhighlight %}

I recommend reading the [getting started
guide](https://www.vagrantup.com/docs/getting-started/) for more information
about vagrant.

Also I have not yet found a nice way to pass the sd card through to the vm.
Instead you have to create the img inside the linux vm and copy to your host
when you are done. This is simple to do in vagrant by simply copying it to the
`/vagrant` folder: `cp custom-pi.img /vagrant/`. Then you can use what ever tool
you normally use to flash the img to your sd card (ie `dd`, `Win32DiskImager`, etc).

Finally there are some issues with the vagrant shares that can cause errors when
trying to manipulate the img inside `/vagrant` so it is best to copy/create it
in a non shared folder inside the vm: `cp /vagrant/custom-pi.img /home/vagrant`
and copy it back when you are done.

## Create an img file

We need to create a file that is large enough to store everything we want to
install. It must be smaller then the size of the sd card you want to install it
to, but the smaller it is the quicker you can write it to the sd card. I
recommend about 2G, larger if you plan to install allot. Note that you can
increase the size of the partitions after you burn the image to your sd card.

{% highlight bash %}
$ fallocate -l 2G "custom-pi.img"
{% endhighlight %}

Note that you can use `dd` to create the file if you prefer but it is allot
slower then `fallocate`.

Now setup the image as a loopback device so we can format and mount it as any
physical disk. Note the device that this command returns below its `/dev/loop0`,
we will need it later.

{% highlight bash %}
$ sudo losetup --find --show "custom-pi.img"
/dev/loop0
{% endhighlight %}

## Format and mount the device

Here we create the partitions the first one, 100M in size, for the boot files
and the second one for the rest of the system.

{% highlight bash %}
sudo parted --script /dev/loop0 mklabel msdos
sudo parted --script /dev/loop0 mkpart primary fat32 0% 100M
sudo parted --script /dev/loop0 mkpart primary ext4 100M 100%
{% endhighlight %}

This will create two new devices `/dev/loop0p1` and `/dev/loop0p2` which you can
see by running `ls /dev/loop0?*`. We can now format these partitions with vfat
(required for the boot partition) and ext4 respectively.

{% highlight bash %}
sudo mkfs.vfat -F32 /dev/loop0p1
sudo mkfs.ext4 -F /dev/loop0p2
{% endhighlight %}

Before we can install archlinuxarm we must mount the partition, first the root
then boot inside the root.

{% highlight bash %}
sudo mount /dev/loop0p2 /mnt
sudo mkdir /mnt/boot
sudo mount /dev/loop0p1 /mnt/boot
{% endhighlight %}

## Install the base system

Now download the archlinuxarm tar for your pi. For the raspberry pi 2

{% highlight bash %}
wget http://archlinuxarm.org/os/ArchLinuxARM-rpi-2-latest.tar.gz
{% endhighlight %}

Or for the raspberry pi 1.

{% highlight bash %}
wget http://archlinuxarm.org/os/ArchLinuxARM-rpi-latest.tar.gz
{% endhighlight %}

And extract it to the mounted img.

{% highlight bash %}
sudo tar -xpf "ArchLinuxARM-rpi-2-latest.tar.gz" -C /mnt
{% endhighlight %}

You may see a bunch of line like

{% highlight bash %}
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
{% endhighlight %}

These are safe to ignore.

You should now have a fully working raspberry pi img and can skip to the cleanup
step below if you do not want to make any customizations to the img.

## Chroot into the image

There are a few system directories that need mounting to
create a successful chroot environment.

{% highlight bash %}
sudo mount -t proc none /mnt/proc
sudo mount -t sysfs none /mnt/sys
sudo mount -o bind /dev /mnt/dev
{% endhighlight %}

Then to get a working network inside the chroot we need to fix the `resolv.conf`
file.

{% highlight bash %}
sudo mv /mnt/etc/resolv.conf /mnt/etc/resolv.conf.bak
sudo cp /etc/resolv.conf /mnt/etc/resolv.conf
{% endhighlight %}

And now for the bit that allows us to execute arm executables on a x86 or x86_64
system.

{% highlight bash %}
sudo cp /usr/bin/qemu-arm-static /mnt/usr/bin/
{% endhighlight %}

We should now be able to chroot into our raspberry pi image.

{% highlight bash %}
sudo chroot /mnt /usr/bin/bash
{% endhighlight %}

Any command you now run will affect the raspberry pi image rather then your
computer or vm.

## Install and configure your install

Here is where you have the most freedom to create a custom image. You can now
install what you want and configure it how you want. I will give you some
suggestions but feel free to skip or add extra steps to create the image the way
you want it.

We only have a chroot, not a fully running system so you are partially limited
in what you can do. For example you cannot enable a service with `systemctl
enable SOMESERVICE`, instead you have to create the symlinks your self `ln -sf
/usr/lib/systemd/system/SOMESERVICE.service
/etc/systemd/system/multi-user.target.wants/`

First thing we should do to our image is update it and install any extra
packages we want.

{% highlight bash %}
pacman -Syu vim bash-completion
{% endhighlight %}

You can change the hostname and rename the default user with the following.

{% highlight bash %}
echo custom-pi > /etc/hostname

sed -i "s/alarm/pi/g" /etc/passwd /etc/group /etc/shadow
mv /home/alarm "/home/pi"
echo -e "secret\nsecret" | passwd "pi"
{% endhighlight %}

Install and enable sudo for our new user with.

{% highlight bash %}
pacman -S --noconfirm sudo
echo '%wheel ALL=(ALL) ALL' >> /etc/sudoers.d/wheel
{% endhighlight %}

Enable the raspberry pi camera.

{% highlight bash %}
sed -i 's/gpu_mem=.*/gpu_mem=128/' /boot/config.txt
grep 'start_file=start_x.elf' /boot/config.txt >/dev/null || echo 'start_file=start_x.elf' >> /boot/config.txt
grep 'fixup_file=fixup_x.dat' /boot/config.txt >/dev/null || echo 'fixup_file=fixup_x.dat' >> /boot/config.txt
{% endhighlight %}

Setup wired and wireless networking to auto connect when the pi boots (or just
roams within range of the network). Remember to replace the `SSID` and
`PASSWORD` with your own wireless credentials. You can also add multiple
wireless networks and the pi will connect to any that it can see allowing you to
move your pi between locations.

{% highlight bash %}
pacman -S --noconfirm wpa_supplicant wpa_actiond ifplugd crda dialog
ln -sf /usr/lib/systemd/system/netctl-auto@.service /etc/systemd/system/multi-user.target.wants/netctl-auto@wlan0.service
ln -sf /usr/lib/systemd/system/netctl-ifplugd@.service /etc/systemd/system/multi-user.target.wants/netctl-ifplugd@eth0.service

cat <<EOF >"/etc/netctl/wlan0-SSID"
Description='Baked in profile'
Interface=wlan0
Connection=wireless
Security=wpa
ESSID="SSID"
IP=dhcp
Key="PASSWORD"
EOF
{% endhighlight %}

Enable zero-conf networking (aka avahi or Bonjour) to make discovering your pi
on the network easier if your system supports it.

{% highlight bash %}
pacman -S --noconfirm avahi nss-mdns
sed -i '/^hosts: /s/files dns/files mdns dns/' /etc/nsswitch.conf
ln -sf /usr/lib/systemd/system/avahi-daemon.service /etc/systemd/system/multi-user.target.wants/avahi-daemon.service
{% endhighlight %}

## Clean up

Once you have finished setting up your pi how you want it we need to clean up
some stuff. First exit the chroot by running `exit` or pressing `ctrl+d`. Then
we start to unwind some of the steps we ran before, starting with restoring the
resolve.conf, and unmounting the system folders needed by the chroot.

{% highlight bash %}
sudo rm /mnt/etc/resolv.conf
sudo mv /mnt/etc/resolv.conf.bak /mnt/etc/resolv.conf
sudo rm /mnt/usr/bin/qemu-arm-static

sudo umount ${root_dir}/dev
sudo umount ${root_dir}/proc
sudo umount ${root_dir}/sys
{% endhighlight %}

Now we can unmount the partitions and detach the loopback device.

{% highlight bash %}
sudo umount /mnt/boot
sudo umount /mnt
sudo losetup --detach "/dev/loop0"
{% endhighlight %}

Finally if you are working inside vagrant you should copy your img file back to
your host and exit vagrant.

{% highlight bash %}
cp custom-pi.img /vagrant/
exit # or ctrl+d
vagrant destroy # or vagrant halt if you plan to do this again soon
{% endhighlight %}

## Flash the image

We are now ready to flash the image. For mac and linux this is as simple as
running the following (remember to replace /dev/mmblk0 with the device for your
sd card).

{% highlight bash %}
dd if=custom-pi.img of=/dev/mmblk0 bs=1M
{% endhighlight %}

Or for windows users you can use what ever tool you normally used to flash the image to an sd card such as [Win32DiskImager](http://www.raspberry-projects.com/pi/pi-operating-systems/win32diskimager).
