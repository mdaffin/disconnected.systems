+++
title = "Create Custom ArchlinuxArm Images for the Raspberry Pi"
description = "Setup a raspberry pi archlinuxarm image or sd-card before needing to boot the pi."
slug = "raspberry-pi-archlinuxarm-setup"
date = 2016-03-21
tags = ["raspberry pi", "archlinuxarm", "arm", "chroot"]
aliases = [
    "/raspberry-pi-archlinuxarm-setup/",
    "/posts/raspberry-pi-archlinuxarm-setup/",
]
+++

I generally work with headless Raspberry Pis either by running them as
lightweight servers or embedding them into projects. It is very anoying having
to plug them into a monitor keyboard and mouse to set them up and quite often
impossible if not at home.

I use to get around this by using a serial cable to configure them after
flashing the image to the sd-card but that has its own disadvantages. Recently
I discovered [how to chroot into a Raspberry
Pi](http://raspberrypi.stackexchange.com/questions/855/is-it-possible-to-update-upgrade-and-install-software-before-flashing-an-image)
image using `qemu` and `binfmt-support` making it possible to setup a pi before
you boot it. In this post I will take this one step further and look at what it
takes to setup a raspberry pi image/sd-card and customise it before you even
have to plug it into a pi.

## Prerequisites

You will require a Linux box with the following packages need to be installed

* `qemu`
* `qemu-user-static`
* `binfmt-support`

See [this](https://wiki.archlinux.org/index.php/Raspberry_Pi#QEMU_chroot) for
setting them up on Arch Linux.

You can alternatively use [Vagrant](https://www.vagrantup.com/) with the
following `Vagrantfile` to give you a Linux virtual machine with all the
prerequisites preinstalled. There are some caveats with using Vagrant for this
process which are out of scope of this post. If there is enough interest I may
cover Vagrant in a future post.

```ruby
# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/trusty64"
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update -y
    apt-get install -y qemu qemu-user-static binfmt-support
  SHELL
end
```

## Create an image file

If you want to write directly to an sd-card skip this step and head to [Format
and Mount the Device](/raspberry-pi-archlinuxarm-setup/#format-and-mount-the-device). Remember to substitute
`/dev/loop0` with the device you want to write to, typically `/dev/mmblkX` or
`/dev/sdX` for sd-cards.

Creating an image file is simple, just create an empty file large enough to
store everything we want to install. A simple way to do this is with `fallocate`
to create a 2 gigibyte file run the following.

```sh
fallocate -l 2G "custom-pi.img"
```

Feel free to play with the image size but 2G is a good starting point. Just
remember that it needs to be smaller then the size of the sd-card you want to
use but large enough to store the operating system and any applications you want
to install. The smaller the better as it takes less time to write the image to
your sd-card later.

Now setup the image as a loopback device so we can format and mount it as any
physical disk. Note the device that this command returns, we will need it later -
in the example below its `/dev/loop0`.

```sh
sudo losetup --find --show "custom-pi.img"
> /dev/loop0
```

## Format and mount the device

Here we create the partitions the first one, 100M in size, for the boot files
and the second one for the rest of the system.

```sh
sudo parted --script /dev/loop0 mklabel msdos
sudo parted --script /dev/loop0 mkpart primary fat32 0% 100M
sudo parted --script /dev/loop0 mkpart primary ext4 100M 100%
```

This will create two new devices `/dev/loop0p1` and `/dev/loop0p2` which you can
see by running `ls /dev/loop0?*`. We can now format these partitions with vfat
(required for the boot partition) and ext4 respectively.

```sh
sudo mkfs.vfat -F32 /dev/loop0p1
sudo mkfs.ext4 -F /dev/loop0p2
```

Before we can install archlinuxarm we must mount the partition.

```sh
sudo mount /dev/loop0p2 /mnt
sudo mkdir /mnt/boot
sudo mount /dev/loop0p1 /mnt/boot
```

## Install the base system

Now download the archlinuxarm tar for your pi. For the raspberry pi 2/3

```sh
wget http://archlinuxarm.org/os/ArchLinuxARM-rpi-2-latest.tar.gz
```

Or for the raspberry pi 1.

```sh
wget http://archlinuxarm.org/os/ArchLinuxARM-rpi-latest.tar.gz
```

And extract it to the mounted image.

```sh
sudo tar -xpf "ArchLinuxARM-rpi-2-latest.tar.gz" -C /mnt
```

You may see a bunch of line like

```bash
tar: Ignoring unknown extended header keyword 'SCHILY.fflags'
```

These are safe to ignore.

You should now have a fully working raspberry pi image and can skip to the
[cleanup step](raspberry-pi-archlinuxarm-setup/#cleaning-up) below if you do not want to make any customizations
to the image.

## Chroot into the image

There are a few system directories that need mounting to
create a successful chroot environment.

```sh
sudo mount -t proc none /mnt/proc
sudo mount -t sysfs none /mnt/sys
sudo mount -o bind /dev /mnt/dev
```

Then to get a working network inside the chroot we need to fix the `resolv.conf`
file.

```sh
sudo mv /mnt/etc/resolv.conf /mnt/etc/resolv.conf.bak
sudo cp /etc/resolv.conf /mnt/etc/resolv.conf
```

And now for the bit that allows us to execute arm executables on a x86 or x86_64
system.

```sh
sudo cp /usr/bin/qemu-arm-static /mnt/usr/bin/
```

We should now be able to chroot into our raspberry pi image.

```sh
sudo chroot /mnt /usr/bin/bash
```

Any command you now run will affect the raspberry pi image rather then your
computer.

## Install and configure your install

Here is where you have the most freedom to create a custom image. You can now
install what you want and configure it how you want. I will give you some
suggestions but feel free to skip or add extra steps to create the image the way
you want it.

Keep in mind we only have a chroot, not a fully running system so you are
partially limited in what you can do. For example you *cannot* enable a service
with `systemctl enable SOMESERVICE`, instead you have to create the symlinks
your self `ln -sf /usr/lib/systemd/system/SOMESERVICE.service
/etc/systemd/system/multi-user.target.wants/`.

First thing we should do to our image is update it and install any extra
packages we want.

```sh
pacman -Syu vim bash-completion
```

You can change the hostname and rename the default user with the following.

```sh
echo custom-pi > /etc/hostname

sed -i "s/alarm/pi/g" /etc/passwd /etc/group /etc/shadow
mv /home/alarm "/home/pi"
echo -e "secret\nsecret" | passwd "pi"
```

Install and enable sudo for our new user with.

```sh
pacman -S --noconfirm sudo
echo '%wheel ALL=(ALL) ALL' >> /etc/sudoers.d/wheel
```

To enable the raspberry pi camera do.

```sh
sed -i 's/gpu_mem=.*/gpu_mem=128/' /boot/config.txt
grep 'start_file=start_x.elf' /boot/config.txt >/dev/null || echo 'start_file=start_x.elf' >> /boot/config.txt
grep 'fixup_file=fixup_x.dat' /boot/config.txt >/dev/null || echo 'fixup_file=fixup_x.dat' >> /boot/config.txt
```

Setup wired and wireless networking to auto connect when the pi boots (or just
roams within range of the network). Remember to replace the `SSID` and
`PASSWORD` with your own wireless credentials. You can also add multiple
wireless networks and the pi will connect to any that it can see allowing you to
move your pi between locations.

```sh
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
```

Enable zero-conf networking (aka avahi or Bonjour) to make discovering your pi
on the network easier, if your system supports it.

```sh
pacman -S --noconfirm avahi nss-mdns
sed -i '/^hosts: /s/files dns/files mdns dns/' /etc/nsswitch.conf
ln -sf /usr/lib/systemd/system/avahi-daemon.service /etc/systemd/system/multi-user.target.wants/avahi-daemon.service
```

## Cleaning Up

Once you have finished setting up your pi how you want it we need to clean up
some stuff. First exit the chroot by running `exit` or pressing `ctrl+d`. Then
we start to unwind some of the setup steps, starting with restoring the
resolve.conf, and unmounting the system folders needed by the chroot.

```sh
sudo rm /mnt/etc/resolv.conf
sudo mv /mnt/etc/resolv.conf.bak /mnt/etc/resolv.conf
sudo rm /mnt/usr/bin/qemu-arm-static

sudo umount /mnt/dev
sudo umount /mnt/proc
sudo umount /mnt/sys
```

Now we can unmount the partitions and detach the loopback device.

```sh
sudo umount /mnt/boot
sudo umount /mnt
sudo losetup --detach "/dev/loop0"
```

## Flash the Image to an SD-Card

We are now ready to flash the image, which can be done with `dd`. Remember to
replace `/dev/mmblk0` with the device for your sd-card.

```sh
sudo dd if=custom-pi.img of=/dev/mmblk0 bs=1M
```

## Conclusion

Thats it, you are now ready to boot your pi. If you followed the above exactly
your pi should boot and connect to your chosen network and be ready to ssh into.
However if you encounter problems with connecting you may need to dig out your
serial cable or attach your pi to a keyboard and monitor after all to debug it.

You may have noticed that the procedure lends itself well to being scripted
making the creating an custom image repeatable. That way you can commit the
scripts to your projects repository and regenerate an image when you want,
rather then having to keep around a bunch of 2G+ backup images that you have
manually setup. I hope to look at scripting this process in a future post to
make creating repeatable images even easier.
