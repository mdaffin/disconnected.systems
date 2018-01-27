+++
date = "2018-01-24T18:12:17Z"
title = "Managing Arch Linux with Meta Packages"
draft = true
description = "Show how you can use meta package to manage multiple arch linux systems"
slug = "archlinux-meta-packages"
tags = ["linux", "automation", "archlinux"]
+++

I really enjoy Arch Linux for its customisation, you can truly make it your own. But while crafting your system exactly how you want it once can be a fun and rewarding experience, repeating this process on multiple computers becomes tedious. Worst, it very easily leads to inconsistencies between each system as you forget things that you configured or as you slowly evolve your configs on each system you set up. The more systems you manage the worst this problem becomes.

There are a fair few solutions out there that aim to make this easier, most notably the plethora of *[dotfile managers]* out there. I have used a few of these over the years and while they have made the above problems easier to deal with they do not solve all the problems I have. The biggest of which is that none of them handles system configs at all only your configs in your home directory, basically only solving half the problem.

What I wanted was a way to manage my whole system, to make it easy and quick to install a new system and to keep the system up to date with my latest configs and packages that I use. For a while, [I tried][salt-arch] using [SaltStack] to manage my systems and users and while effective for a while it was more maintenance than was worth it.

In my [last post], I talked about how you can create and maintain a custom Arch Linux repository for AUR packages. But there was something else I wanted to use this repo for - creating and maintaining [meta packages] to solve some of the issues above.  In this post, I will talk about these meta packages, what they are, how to create them and how they can solve the issues above.

[dotfile managers]: https://wiki.archlinux.org/index.php/Dotfiles 
[SaltStack]: https://saltstack.com/
[salt-arch]: https://github.com/mdaffin/salt-arch
[last post]: /blog/archlinux-repo-in-aws-bucket/
[meta packages]: https://wiki.archlinux.org/index.php/creating_packages#Meta_packages_and_groups

## What are Meta-Packages

This is quite simple, meta-packages a simply packages that install nothing, but depend on other packages. The result of this is that you can install a group of packages at once by installing a single meta-packages. Now Arch Linux has a similar method; package groups, while they serve a similar purpose they are suitably different. This difference is when you add or remove dependencies - groups will not install/remove additional dependencies whereas meta packages will. This is what we want; when we add a dependency we want all our systems to automatically install it when they update.

Now, we are going to abuse this concept slightly and not only use them to install groups of packages via dependencies but also to install related system configuration. This is not something `pacman` is really designed to do and results in some hacky workaround which we will discuss below. The end result, however, works very well in practice.

## Organising Our Meta-Packages

We can create as many or as few meta-packages as we require. If you want to configure all of your systems identically you can create a single mega-meta-package that defines all of the dependencies and packages that you want. Or you can bundle up each application with related packages and configs into separate packages. You can even create a hierarchy of packages by depending on other meta packages you have created. This can be useful for creating more complex systems of overrides.

For example, I have a set of packages that I install on all of my systems - so I created a meta package `mdaffin-base` to contain these. Note that I will start all my packages with `mdaffin-` to make searching/filtering easier. I can then create a `mdaffin-server` that depends on this base package as well as addition packages that I use on servers while also creating a `mdaffin-desktop` that contains all the packages I use on my desktop systems. This has the advantage that I can update the base package to install or change something on all the system I manage, or add/change something inside my desktop package to only affect my desktop systems.

In spite of this, on each system, I only need to install one package, either `mdaffin-server` or `mdaffin-desktop` and they will both pull in the base configs.  But you can also separate packages orthogonally, for example in addition to the `mdaffin-desktop` I have a `mdaffin-devel` that contains development utilities and tools. This package can be installed alongside, or separately from the main desktop package. This allows me to configure any system for development simply by running `pacman -S mdaffin-develop` but most of my systems do not have to include these. You can also have packages for specific systems, such as my `mdaffin-dell-xps-13` which requires additional packages and configs that are required for my Dell XPS 13 which are not needed on my desktops or other systems.

One major advantage of using meta-packages like this is it is very simple to see how your system is configured with simple `pacman` commands, for example, to see packages that were explicitly installed run the following

```bash
% pacman -Qe
hugo 0.34-1
krita 3.3.3-1
mdaffin-dell-xps-13 0.0.2-1
mdaffin-desktop 0.0.9-1
mdaffin-devel 0.0.3-1
powertop 2.9-1
s3fs-fuse 1.80-2
```

Here you can see my system has a desktop interface, the Dell XPS additions and is configured for development. Note that there are additional packages as well - but not too many. These are packages which I wanted on this system only, but not on my systems in general. The use of meta-packages helps to keep this list clear and makes it easier to find/remove unused packages. From here I can either chose to add them to a meta-package, remove them or just leave them on this one system.

## Creating a Meta-Package

This is actually really simple, all you require is a `PKGBUILD` file along with any configs you want. You can read more about the finer details about [createing a package] and [PKGBUILD] file structure, both of which are worth a read or at least to look up as a reference. I will give a quick example and then talk about somehow to deal with some problematic situations.

So, the first part of creating meta-packages is specifying dependencies, here is a minimal PKGBUILD config 

```bash
# Maintainer: Michael Daffin <michael@daffin.io>
pkgname=mdaffin-base
pkgver=0.0.5
pkgrel=1
pkgdesc="Base system configuration for mdaffin systems"
arch=('any')
url="https://github.com/mdaffin/arch-repo"
license=('MIT')
groups=('mdaffin')
depends=(
    # package list
)
```

[creating a package]: https://wiki.archlinux.org/index.php/creating_packages#Meta_packages_and_groups
[PKGBUILD]: https://wiki.archlinux.org/index.php/PKGBUILD

### Adding Dependencies

Simply list the packages you want to install with this package in the `depends` block. A good set of packages to start with is the `base` group. But there is a problem - you cannot specify groups of packages as dependencies so we must first expand the group to get a list of packages in that group.

```bash
 % pacman -Sqg base                                                                                                                       :(
bash
bzip2
coreutils
cryptsetup
device-mapper
dhcpcd
diffutils
e2fsprogs
file
filesystem
findutils
gawk
gcc-libs
gettext
glibc
grep
gzip
inetutils
iproute2
iputils
jfsutils
less
licenses
linux
logrotate
lvm2
man-db
man-pages
mdadm
nano
netctl
pacman
pciutils
pcmciautils
perl
procps-ng
psmisc
reiserfsprogs
s-nail
sed
shadow
sysfsutils
systemd-sysvcompat
tar
texinfo
usbutils
util-linux
vi
which
xfsprogs
```

Then simple surround all of these with quotes `'`  and include them in the depends list like so;

```bash
depends=(
    'bash'
    'bzip2'
    'coreutils'
    'cryptsetup'
    'device-mapper'
    ...
    'which'
    'xfsprogs'
)
```

And there, that's our first meta package. Though it's currently not much more helpful than the base group so go ahead and add any additional packages you want. 

For example, I have added these (as well as many others) to the list as I use them on all of my systems.

```bash
    'sudo'
    'neovim'
    'avahi'
    'nss-mdns'
```

### Adding Config Files

Now we have our base packages installed it is time to configure some of them. Some config is very simple, simply add the file as you want it alongside the PKGBUILD then add `sources`, `md5sum` or `sha256sum` and a `package()` sections to  the PKGBUILD config like the following;

```bash
source=('locale.conf'
        'vconsole.conf'
        'sudoers.wheel'
        'mdaffin-base.sh')
md5sums=('f6ade2d2d1e9b9313f6a49a7ea7b81ea'
         '12733d28ff3b5352ea1c3d84b27cd6bd'
         '52719e50fbbea8255275964ba70aa0a7'
         '9463e8e19ee914684f7bd5190243aa3f')

package() {
    install -Dm 0644 locale.conf "$pkgdir/etc/locale.conf"
    install -Dm 0644 vconsole.conf "$pkgdir/etc/vconsole.conf"
    install -Dm 0640 sudoers.wheel "$pkgdir/etc/sudoers.d/wheel"
    install -Dm 0755 mdaffin-base.sh "$pkgdir/etc/profile.d/mdaffin-base.sh"
}
```

Here I have four files, `locale.conf`, `vconsole.conf` which contain the basic locale and console settings as described on the ArchWiki. `sudoers.wheel` contains the `sudo` config needed to allow anyone in the `wheel` group access to run any command with `sudo` and `mdaffin-base.sh` which contains extra environment variables and shell configuration which I like to use. 

`locale.conf` and `vconsole.conf` are nice and easy to install as these files do not exist by default and no package in the base group owns or creates them. `sudoers.wheel` and `mdaffin-base.sh` are also trivial to install as both `sudo` and shells are set up to allow packages to extend their config by using config directories, which include any file, or files with particular extensions when they start up.

Note that we do not mark any configs as `configs` in PKGBUILD. This is intentional. Files marked as configs in PKGBUILD are treated specially by pacman and are designed for files which the user might want to edit. As such if they differ from the version that was originally installed pacman will not update/replace them but instead, leave them in place and allow the user to manually update them. For most packages, this is what you want, but for our meta-packages, we don't want the user to customise them - that's the job of the meta-package and this is what allows us to keep all our systems in sync. This is a mild break from how you are meant to design well-rounded packages but we don't really care as these are specific to us and not intended for general use and should not be uploaded to AUR.

### Overwriting Existing Configs

There is one last bit to managing configs in our meta-packages, unfortunately, this part is a bit of a hack so it is worth talking about how packages work. Anything inside `$pkgdir`, after all of the sections have run, is included in the package and will be installed on the target system as they appear inside `$pkgdir`. All of these files are removed when the package is removed, including during an update, before being replaced by newer versions of the files. Now, you mark certain files as config files, ones that should be backed up and not replaced during an upgrade, by placing them in the `backup` array in `PKGBUILD`.

Since most packages do not support config directories (where they load any config file inside a directory, such as how shells read all `*.sh` files from `/etc/profile.d`) the only way to configure some packages is to edit the config files owned by a package. Now, `pacman` directly forbids this and for good reason, but we only want to modify config files, the same way a user would modify these files but in an automated way. Given that our meta-packages are designed by you for you and no one else and that these packages are not expected to be uploaded to the AUR or generally used by anyone else it is less of a problem to make them a little hacky. If anyone knows of a better way to achieve this please let me know.

The general idea is to place the config we want alongside the config we want to replace. I am going to take the i3 config as an example. First, we place the config in our package like any other config - next to PKGBUILD, in the `sources` and `md5sum` or `sha256sum` arrays.  Now, i3 places its system config in `/etc/i3/config`, but we cannot overwrite this file directly in `$pkgdir` so let's install it alongside this file in `$pkgdir/etc/i3/mdaffin-desktop-config`. The relevant bits of the `PKGBUILD` becomes:

```bash
source=('i3-config')
md5sums=('d9dcd133475af688ed86a879821c9384')
package() {
    install -Dm 0644 i3-config "$pkgdir/etc/i3/mdaffin-desktop-config"
}

```

So far, no hacks, but also this file does not do anything. We could simply instruct the user to copy this file over the actual config - probably something you would/should do in a more official package - but in our case, we want to minimise anything the user has to do to their system. Luckily `pacman` offers a way to run commands during various stages of the install/upgrade process, these are called hooks. And we can use them to automate this copying step. Now since we are only modifying config (aka files in another packages `backup` array) `pacman` will not overwrite them during an upgrade of the official package, but treat it like the user has changed the config themselves.

These hooks are defined in a file, call it whatever you like but they are typically called `package-name.install` and referenced in `PKGBUILD`'s `install` key. So inside your `PKGBUILD` add something like:

```bash
...
install=mdaffin-desktop.install
...
```

]Then inside `mdaffin-desktop.install` (which should be placed next to the `PKGBUILD` file) add the following hooks.

```bash
post_install() {
    post_upgrade
}

post_upgrade() {
    cp /etc/i3/mdaffin-desktop-config /etc/i3/config 
    cp /etc/xdg/termite/mdaffin-desktop.config /etc/xdg/termite/config
}
```

### Starting Services

One last common thing we want to do is enable/start services. Typically arch does not auto enable or start services but leaves this up to the user. This is normally nice as it lets you configure them before they start. But we are dropping the config files into the package and don't really want to have to remember all of the services to start/enable.

Now there are a couple of ways to do this, we could create symlinks in the correct places to enable services to start on boot, which can be very handy when you are installing our packages in a chroot on the live cd. But this will not start the services immediately on already running systems, such as if you install an extra package or upgrade a package with a new service you want to enable. It would start on next boot - but that requires a reboot, something I like to avoid where possible.

We can also use the `post_install` hook from the previous section to tell `systemctl` to start and enable the service on the first install, where the user is then free to disable/stop it thereafter. We could also add it to the `post_upgrade` hook to ensure it gets reenabled after an upgrade, or if you add an extra service to a package later on. For example, my desktop package has these in the install script (in addition to the things in the previous section).

```bash
post_install() {
    systemctl enable --now sddm
    systemctl enable --now connman
    systemctl enable --now avahi-daemon
}
```

Note that `--now` on a `systemctl enable` causes it to also start the service in addition to enabling it, basically equivalent to `systemctl start` and `systemctl enable` in one command.

## Building the Package

Once you have crafted a package to your liking it is time to build it. This can be done with `makepkg` as anyone who has built a package from AUR should be aware. But instead, I will make use of its lesser-known wrapper `makechrootpkg`. While a little bit involved it does provide a clean build for packages by building them in a fresh chroot environment rather than your host system. The downside is that it takes some prep work and is a little slower. Feel free to continue to use `makepkg` if you want.

To save time on installing a base arch system into each chroot, `makechrootpkg` relies on a prepped root which it copies for each package it builds. We can create this root fs by running `mkarchroot`.

```bash
mkdir -p ./chroots
mkarchroot -C /etc/pacman.conf ./chroots/root base-devel
```

`chroot` is where all of the chroots will live and `root` is the base root fs `makechrootpkg` will use by default. Now while inside our packages directory run `makechrootpkg` and tell it what directory to use for the chroots scratch area.

```bash
makechrootpkg -cur ./chroots
```

Once done you will end up with a `*.pkg.tar.xz` package in the current directory just like with `makepkg`.

Lastly, we can install this package into a repo, such as the one I showed you how to create in my [last post] by mounting the repo, copying the package into it and running `repose` to update the package database.

```bash
mkdir -p repo
s3fs mdaffin-arch:/repo "repo" -o "nosuid,nodev,default_acl=public-read"
cp *.pkg.tar.xz "repo/x86_64/"
repose --verbose --xz --root="repo/x86_64/" mdaffin
```

Now you can install the package as any other package with `pacman` as long as you have your repo added to `/etc/pacman.conf`.

[last post]: /blog/archlinux-repo-in-aws-bucket/

## Git Repo and Scripting the Build

Now we can create meta package and publish them for use lets place these in a git repo (or another version control system if you prefer) and write a wrapper script to make building/uploading the packages even easier. You can find [my repo] on github, feel free to use it as a reference or clone it to create your own but the packages in there are tuned to my liking and so I encourage you to create your own with how you like your systems setup.

Let us start with a new repo.

```bash
mkdir arch-repo
cd arch-repo
git init
```

I like to put all of my packages inside `pkg/<package name>` to keep them in one place. So let's copy the package we created above to that location

```bash
mkdir pkg
cp -r ~/mdaffin-base pkg/mdaffin-base
```

But there a whole bunch of temporary/generated files we don't want to commit to let's add a gitignore for these.

```bash
cat <<'EOF' >.gitignore
*.pkg.tar.xz
*.tar.gz
/pkg/**/pkg/
/pkg/**/src/
repo/
*.log
/root/
EOF
```

Now stage these files and check we are not including anything we don't want.

```bash
% git add .
% git status
On branch master

No commits yet

Changes to be committed:
  (use "git rm --cached <file>..." to unstage)

    new file:   .gitignore
    new file:   pkg/mdaffin-base/PKGBUILD
    new file:   pkg/mdaffin-base/locale.conf
    new file:   pkg/mdaffin-base/mdaffin-base.install
    new file:   pkg/mdaffin-base/mdaffin-base.sh
    new file:   pkg/mdaffin-base/sudoers.wheel
    new file:   pkg/mdaffin-base/vconsole.conf
```

If anything is listed that you don't want, just add it to the .gitignore and run `git reset <file>`. Repeat until you are happy then commit.

```bash
git commit -m "My first package"
```

[my repo]: https://github.com/mdaffin/arch-repo

### The Build Script

The commands above can be wrapped into a helper script to make building and uploading the packages very simple. Here is the script in its entirety;

```bash
#!/bin/bash
set -uo pipefail
trap 's=$?; echo "$0: Error on line "$LINENO": $BASH_COMMAND"; exit $s' ERR

exit_cmd=""
defer() { exit_cmd="$@; $exit_cmd"; }
trap 'bash -c "$exit_cmd"' EXIT

PACKAGES=${@:-pkg/*}
CHROOT="$PWD/root"

BUCKET=mdaffin-arch
REPO_PATH=repo/x86_64
REPO_NAME=mdaffin

mkdir -p "$CHROOT"
[[ -d "$CHROOT/root" ]] || mkarchroot -C /etc/pacman.conf $CHROOT/root base base-devel

for package in $PACKAGES; do
    cd "$package"
    rm -f *.pkg.tar.xz
    makechrootpkg -cur $CHROOT
    cd -
done

repo="$(mktemp -d)"
defer "rmdir '$repo'"

s3fs "$BUCKET" "$repo" -o "nosuid,nodev,default_acl=public-read"
defer "fusermount -u '$repo'"

rsync --ignore-existing -v pkg/*/*.pkg.tar.xz "$repo/$REPO_PATH"
repose --verbose --xz --root="$repo/$REPO_PATH" "$REPO_NAME"
```

It starts with some boilerplate code which I will skip over, read [this post][bash-strict-mode] for more details about it.

Next is a cleanup helper, again I am not going to talk about it here (but might in a future post if there is interest). All you need to know is that any command added by the `defer` function will be run in reverse order when the program exits for any reason. This ensures we clean up no matter how the program exits.

Some useful variables are then defined, `${@:-pkg/*}` means take all arguments, but if there are none default to `pkg/*`. This allows us to only build a single package, any number of packages but default to all packages if none are supplied. `BUCKET`, `REPO_PATH` and `REPO_NAME` should be changed to match your repo.

We create the chroot directory and init the main root fs if it does not already exist. And then loop over all the packages to build them one at a time, during which we delete all old package files left over from previous builds. This keeps the list of built packages down and ensures we only upload the latest build version.

Lastly, we mount the remote repo to a tempory directory and copy all packages we have built to the repo. We ignore any that already exist in the repo with `--ignore-existing` on the `rsync` command. Packages should be immutable once uploaded to the repo if you want to change a package you should increment its version number which will create a newer non-conflicting package. Ideally we should refuse to build any package that already exists in the repo with the same version and a future version of this may do that but for now, this is good enough. The final command updates the repo database with any packages that were added.

Some of this should look familiar from the script we created in the last post. It would be handy to store both of these scripts inside our repo under the ./bin/ directory. I have also created a shell script that mounts the repo and drops you in a shell, auto cleaning up after you exit the shell. I found this useful for manually fixing things in the repo as I was developing everything. I will not cover it in this post as the bulk of it has been described above. You can view/download it from [here][shell-wrapper].

[bash-strict-mode]:
[shell-wrapper]:

## Summary


Now, this is quite a lot of work to set up initially and might not be worth it if you only manage one or two Arch Linux systems that you rarely change. But if you manage multiple systems and want to keep them in sync it can be worth it. Once you have set everything up for the first time tweaks to the packages is much simpler making ongoing maintenance less time consuming than manually ensuring all of your systems have your latest settings.

It is also worth noting that this does not solve the issue of keeping user files in sync. But most of the user files I want to keep in sync also have system level defaults that I can keep in sync instead lowering the number of files I need to manage in my home directory. This works best when you only have one user, or all your users are fine with the same default settings, but they can always override them within their own home directory like you normally would.

There is one last step to make managing multiple Arch Linux systems almost fully automated, the installation process. I will cover that in my next post as this one has become long enough.

Any thoughts, suggestions, problems you have faced, discuss them on this [reddit thread]

[reddit thread]: 