+++
date = "2017-03-09T13:00:55Z"
title = "Customising raspberry pi images with github and travis"
draft = true
description = "Make use of github and travis to automate the customisation of raspberry pi images"
slug = "custom-rpi-image-with-github-travis"
tags = ["linux", "automation", "raspberry-pi", "github", "travis"]
+++

Over the coming months I am goring to be building upon my raspberry pi rover.
Over this time I will be needing to change the image quite often. I have always
been in favour of making things builds reproducible rather then simply backing
up an image after setting it up so I am going to be creating a lot of images for
the pi. I figured I would take this time to automate this process so save on the
repetitive task later and will give me an distributable image others can use.

A large amount of this work has already been covered in my post on [setting up
archlinuxarm on the raspberry pi]({{< relref
"blog/2016-03-21-raspberry-pi-archlinuxarm-setup.md" >}}). This post will look
at taking the commands covered in that post and getting them to run in travis
via a github repo and publishing the image as to a release on github.

## Github Repo

Head over to [github.com](https://github.com), sign up or login and create a [new
public repo](https://github.com/new). The repo needs to be public to take
advantage of the free build on travis for open source projects, other wise you
need to pay for a private plan on both github and travis.

## create-image script

Create a new file called `create-image` (no extension) in the root of your git
repo (either clone it locally or add it through the github interface) with the
following contents.

```shell
#!/bin/bash
# Setup script error handling see {{< relref "blog/2016-04-14-another-bash-strict-mode.md" >}} for details
set -uo pipefail
trap 's=$?; echo "$0: Error on line "$LINENO": $BASH_COMMAND"; exit $s' ERR
IFS=$'\n\t'

# Ensure we are root
if [[ $EUID -ne 0 ]]; then
    echo "This script must be run as root" 1>&2
    exit 1
fi

# Some useful constants
mount="mnt" # Local directory to mount the image (automatically created/delete as needed)
script="setup" # Local setup script to run inside the image
rpi_tar="ArchLinuxARM-rpi-latest.tar.gz" # Raspberry pi 1 image others can be found on https://archlinuxarm.org/
rpi_url="http://archlinuxarm.org/os/${rpi_tar}"

# Grab the first two arguments, using the defaults if not set
# This allows running the command as any of
#   ./create-image                 # to use the defaults
#   ./create-image 4G              # create a 4 gig image instead of 2 gig
#   ./create-image 4G myname.img   # create a 4 gig image called myname.img
size="${1:-2G}"
image="${2:-rpizw-rover.img}"

# Tasks to run when the shell exits for any reason, unmount the image and 
# general cleanup
cleanup() {
    [[ -f "${mount}/tmp/${script}" ]] && rm "${mount}/tmp/${script}"
    if [[ -d "${mount}" ]]; then
        umount "${mount}/dev" || true
        umount "${mount}/proc" || true
        umount "${mount}/sys" || true
        umount "${mount}/boot" || true
        umount "${mount}" || true
        rmdir "${mount}" || true
    fi
    [ -n "${loopdev}" ] && losetup --detach "${loopdev}" || true
}
trap cleanup EXIT

# Download archlinux arm only if we have not already done so
[ ! -f "${rpi_tar}" ] && wget "${rpi_url}"

# Create, partition and format the image
fallocate -l "${size}" "${image}"
loopdev=$(losetup --find --show "${image}")
parted --script "${loopdev}" mklabel msdos
parted --script "${loopdev}" mkpart primary fat32 0% 100M
parted --script "${loopdev}" mkpart primary ext4 100M 100%
bootdev=$(ls "${loopdev}"*1)
rootdev=$(ls "${loopdev}"*2)
mkfs.vfat -F32 ${bootdev}
mkfs.ext4 -F ${rootdev}

# Mount the image
[ ! -d "${mount}" ] && mkdir "${mount}"
mount "${rootdev}" "${mount}"
[ ! -d "${mount}/boot" ] && mkdir "${mount}/boot"
mount "${bootdev}" "${mount}/boot"

# Install archlinuxarm to the image
tar -xpf "${rpi_tar}" -C ${mount} 2> >(grep -v "Ignoring unknown extended header keyword")

# Copy our installation script to the mount so it can be run in the chroot
cp "${script}" "${mount}/tmp/${script}"
chmod +x "${mount}/tmp/${script}"

# Prep the chroot
mount -t proc none ${mount}/proc
mount -t sysfs none ${mount}/sys
mount -o bind /dev ${mount}/dev
rm ${mount}/etc/resolv.conf
cp /etc/resolv.conf ${mount}/etc/resolv.conf
cp /usr/bin/qemu-arm-static ${mount}/usr/bin/

# Run the setup script inside the chroot
chroot ${mount}  "/tmp/${script}"
```

You can also copy any other resources (such as additional binaries or configs)
to the image in the same way we copied the script to the `chroot`.

## setup script

This script is what is run inside the `chroot` environment and is where we can
truly customise the image. Create another file in the root of your git repo
called `setup` (again, no extension) with the following contents.

```shell
#!/bin/bash
set -uo pipefail
trap 's=$?; echo "$0: Error on line "$LINENO": $BASH_COMMAND"; exit $s' ERR
IFS=$'\n\t'

# Install any packages we want
pacman -Syu --noconfirm vim bash-completion zsh grml-zsh-config sudo avahi nss-mdns wpa_supplicant termite-terminfo

# Update the hostname
echo rpizw-rover > /etc/hostname

# Configure sudo
echo '%wheel ALL=(ALL) ALL' >> /etc/sudoers.d/wheel

# Enable the camera
sed -i 's/gpu_mem=.*/gpu_mem=128/' /boot/config.txt
grep 'start_file=start_x.elf' /boot/config.txt >/dev/null || echo 'start_file=start_x.elf' >> /boot/config.txt
grep 'fixup_file=fixup_x.dat' /boot/config.txt >/dev/null || echo 'fixup_file=fixup_x.dat' >> /boot/config.txt

# Install zero-conf to make the pi easier to find on the network
sed -i '/^hosts: /s/files dns/files mdns dns/' /etc/nsswitch.conf
ln -sf /usr/lib/systemd/system/avahi-daemon.service /etc/systemd/system/multi-user.target.wants/avahi-daemon.service

# Enable wireless, actual connection details will be configured by the user, likely over usb-serial.
# No not put any secrets like wifi passphrases in here as they will be publicly exposed in the repo and image.
ln -sf /usr/lib/systemd/system/wpa_supplicant@.service /etc/systemd/system/multi-user.target.wants/wpa_supplicant@wlan0.service

cat >/etc/systemd/network/wlan0.network <<EOF
[Match]
Name=wlan0
[Network]
DHCP=yes
EOF

cat <<EOF > /etc/wpa_supplicant/wpa_supplicant-wlan0.conf
ctrl_interface=/var/run/wpa_supplicant
ctrl_interface_group=wheel
update_config=1
fast_reauth=1
ap_scan=1
EOF

# Enable the usb serial, this will make it easier to add any wifi credentials on the pi zero (will not work on other models).
grep 'dtoverlay=dwc2' /boot/config.txt >/dev/null || echo 'dtoverlay=dwc2' >> /boot/config.txt
grep 'modules-load=dwc2,g_serial' /boot/cmdline.txt >/dev/null || sed -i 's/.*rootwait.*/& modules-load=dwc2,g_serial/' /boot/cmdline.txt
ln -sf /usr/lib/systemd/system/getty@ttyGS0.service /etc/systemd/system/getty.target.wants/getty@ttyGS0.service

# Enable hardware pwm
grep 'dtoverlay=pwm-2chan,pin=12,func=4,pin2=13,func2=4' /boot/config.txt >/dev/null || echo 'dtoverlay=pwm-2chan,pin=12,func=4,pin2=13,func2=4' >> /boot/config.txt

# Set zsh as the default shell for root and alarm
chsh -s /usr/bin/zsh root
chsh -s /usr/bin/zsh alarm
touch /root/.zshrc
touch /home/alarm/.zshrc
chown alarm. /home/alarm/.zshrc
```

This is the script used to setup my [pi rover]({{< relref
"blog/2017-03-08-pi-zero-w-rover-setup.md" >}}) and you should customise it to suit 
your application.

You can test run these scripts locally if you are running linux, or by using
[Vagrant](https://www.vagrantup.com/) as mentioned in my previous post.

## Travis

Now its time to sign up to [travis](https://travis-ci.org) (you can use your
github account to do this). Ensure the account is linked and synced to your
github account then enable your repo in the account settings. You can read more
about getting started with travis
[here](https://docs.travis-ci.com/user/for-beginners).

Create a travis yaml file in the root of your repo called `.travis.yml` (the
leading dot `.` is important, do not miss it off).

```yaml
dist: trusty
sudo: required
addons:
  apt:
    packages:
    - qemu
    - qemu-user-static
    - binfmt-support
    - parted
    - wget
    - dosfstools
script:
- sudo ./create-image
- xz -z rpizw-rover.img -c > rpizw-rover.img.xz
```

Once you have saved, committed and push (if you have a local clone) travis will
automatically start building your image. You can follow the build on the travis
site including a full build log. If everything has gone write you should end up
with a green passing build. If not you can inspect the logs to find out why it
failed.

```shell
sudo docker run --privileged --rm -it -v $PWD:/code -w /code ubuntu bash -c "apt update -y && \
apt install -y qemu qemu-user-static binfmt-support parted wget dosfstools && \
./create-image" 
```

```shell
-rw-r--r-- 1 root    root    2.0G Mar  5 23:36 rpizw-rover.img
-rw-r--r-- 1 mdaffin mdaffin 303M Mar  5 23:38 rpizw-rover.img.bz2
-rw-r--r-- 1 mdaffin mdaffin 330M Mar  5 23:37 rpizw-rover.img.gz
-rw-r--r-- 1 mdaffin mdaffin  94M Mar  5 23:38 rpizw-rover.img.xz
```
