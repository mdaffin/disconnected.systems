+++
date = "2018-02-01T09:00:00Z"
title = "Automating Arch Linux Part 2: Managing Arch Linux with Meta Packages"
draft = false
description = "Show how you can use meta package to manage multiple arch linux systems"
slug = "archlinux-meta-packages"
tags = ["linux", "automation", "archlinux"]
+++

In this three-part series, I will show you one way to simplify and manage multiple Arch Linux systems using a custom repo, a set of meta-packages and a scripted installer. Each part is standalone and can be used by its self, but they are designed to build upon and complement each other each focusing on a different part of the problem.

- **Part 1:** [Hosting an Arch Linux Repo in an Amazon S3 Bucket]
- **Part 2:** *Managing Arch Linux with Meta Packages*
- **Part 3:** [Creating a Custom Arch Linux Installer]

[Hosting an Arch Linux Repo in an Amazon S3 Bucket]: /blog/archlinux-repo-in-aws-bucket
[Creating a Custom Arch Linux Installer]: /blog/archlinux-installer

I really enjoy Arch Linux for its customisation - you can truly make it *your* own. But while crafting your system exactly how you want it once can be a fun and rewarding experience, repeating this process on multiple computers becomes tedious. Worst, it very easily leads to inconsistencies between each system as you forget things that you configured or as you slowly evolve your configs on each system you set up. The more systems you manage the worst this problem becomes.

There are a fair few applications that aim to make this easier, most notably the plethora of *[dotfile managers]* out there. I have used a few of these over the years and they all suffer from the same problem; they are only designed to manage user files. I want to manage my whole system. I used [SaltStack] for [a while][salt-arch] but started to dislike it when it became hard to debug problems (salts error handling is quite bad) and I kept forgetting to run `salt-call state.apply` to resync my systems.

My ultimate goal is to find a way to automatically install Arch Linux on a system with minimal input (only things like hostname, username, password etc). As well as for a way to automatically update all of my systems whenever I change a config or add/remove a package. Ideally just running `pacman -Syu` on a system to sync all the configs from a master set. In this post, I will show you how I solved this second goal using the repo I set up in my [last post] with a set of custom [meta-packages].

[dotfile managers]: https://wiki.archlinux.org/index.php/Dotfiles 
[SaltStack]: https://saltstack.com/
[salt-arch]: https://github.com/mdaffin/salt-arch
[last post]: /blog/archlinux-repo-in-aws-bucket/
[meta-packages]: https://wiki.archlinux.org/index.php/creating_packages

## What are Meta-Packages

Simply put, meta-packages are packages that do nothing but depend on other packages. The result of this is that you can install a group of packages at once by installing a single meta-packages. Now, Arch Linux has a similar concept; package groups, and while they serve a similar purpose they are suitably different. This difference is when you add or remove dependencies - groups will not install/remove additional dependencies whereas meta packages will. This is what we want, when a dependency is added all our systems to automatically install it when they update.

Now, we are going to abuse this concept slightly and not only use them to install groups of packages via dependencies but also to install related system configuration. This is not something `pacman` is really designed to do and results in some hacky workaround which we will discuss below. The end result, however, works very well in practice.

## Organising Our Meta-Packages

You can create as many or as few meta-packages as you require. The more you have, the more there is to maintain. The less you have the less flexible they become. How many you require depends on your needs but I recommend starting with few large packages that do the bulk of the work that is common to all or most of your systems. Then create smaller more specific ones, that depends on these more general ones.

For example, I have [`mdaffin-base`] that contains everything that I require on all of my systems. Then [`mdaffin-desktop`] for any system that needs a desktop interface, such as my desktops and laptops. This package depends on `mdaffin-base` so I still only have one package to install. But then I have a [`mdaffin-dell-xps-13`] which contains very specific settings only useful on my laptop. This, in turn, depends on [`mdaffin-desktop`].

Separate to this I have [`mdaffin-devel`] package that contains a whole bunch of development tools and languages that I commonly use and want on any system that I do development on. Changing a system to a development system becomes as simple as `pacman -S mdaffin-devel`.

In the future, I might have a `mdaffin-laptop` or `mdaffin-server` as or when I decide I will require them but for now, these systems will be based off one of the existing packages.

One major advantage of using meta-packages like this is it is very simple to see how your system is configured with simple `pacman` commands, for example, to see packages that were explicitly installed run the following.

```bash
% pacman -Qe
hugo 0.34-1
krita 3.3.3-1
mdaffin-dell-xps-13 0.0.2-1
mdaffin-devel 0.0.3-1
powertop 2.9-1
s3fs-fuse 1.80-2
```

Or to see all of our meta-packages are install run the following.

```bash
% pacman -Qqs mdaffin-
mdaffin-base
mdaffin-dell-xps-13
mdaffin-desktop
mdaffin-devel
```

This is a hidden and quite powerful feature of this method. Whereas on most Arch Linux systems you might see hundreds of packages explicitly installed with no way to track why you installed them, we only have a handful. This is actually very useful as it lets you track bloat on your systems over time where you can either promote them to one of your meta-packages if you use them a lot or remove them from your systems if you don't. You also have the option of commenting the dependencies in your meta-packages to remind you why you added them.

[`mdaffin-base`]: https://github.com/mdaffin/arch-repo/tree/master/pkg/base
[`mdaffin-desktop`]: https://github.com/mdaffin/arch-repo/tree/master/pkg/desktop
[`mdaffin-dell-xps-13`]: https://github.com/mdaffin/arch-repo/tree/master/pkg/dell-xps-13
[`mdaffin-devel`]: https://github.com/mdaffin/arch-repo/tree/master/pkg/devel

## Creating a Meta-Package

All you require to build a meta-package is a `PKGBUILD` file along with any configs you want. You can read more about the finer details about [creating a package] and the [PKGBUILD] file structure, both of which are worth a read or at least to look up as a reference.

So, the first part of creating meta-packages is specifying dependencies. For this we need a minimal `PKGBUILD` file, all you need to do is fill in the details with your own and flesh out the dependency list.

```bash
# Maintainer: Michael Daffin <michael@daffin.io>
pkgname=mdaffin-base
pkgver=0.0.1
pkgrel=1
pkgdesc="Base system configuration for mdaffin systems"
arch=('any')
url="https://github.com/mdaffin/arch-repo"
license=('MIT')
depends=(
    # package list
)
```

[creating a package]: https://wiki.archlinux.org/index.php/creating_packages#Meta_packages_and_groups
[PKGBUILD]: https://wiki.archlinux.org/index.php/PKGBUILD

### Adding Dependencies

List the packages you want to be installed with this package in the `depends` block, as quoted strings separated by white space (such as a newline). A good set of packages to start with is the `base` group. But there is a problem - you cannot specify groups of packages as dependencies. Instead, we must first expand the group to get a list of packages in that group.

```bash
 % pacman -Sqg base
bash
bzip2
coreutils
...
which
xfsprogs
```

Then surround all of these with quotes and include them in the depends list like so.

```bash
depends=(
    'bash'
    'bzip2'
    'coreutils'
    ...
    'which'
    'xfsprogs'
)
```

And there, that's our first meta package. Although, it's currently not much more helpful than the base group so go ahead and add any additional packages you want. For example, I have added these (as well as many others) to the list as I use them on all of my systems.

```bash
    'sudo'
    'neovim'
    'avahi'
    'nss-mdns'
```

### Adding Config Files

Adding a config file to a package is done just like any other package except you also add it to the `backup` array in the `PKGBUILD`. This instructs pacman to not replace the file during an upgrade if the user has modified it at all. But we don't want this for our package - rather, we want to treat config files as any other package file so that it is replaced on an upgrade. We do this to stop the configs from drifting apart at the risk of losing some changes to one system if you don't add those changes back to the package.

Some packages don't drop a default config, while others will read and merge all files inside a certain directory. These types of packages are easy to add configs for, we just add the file to our package and let packman place it in the correct place. To do this place the configs you want next to the `PKGBUILD` and add it to the `sources` array with a corresponding entry in `md5sum` or `sha256sum` (or equivalent) arrays. Then add an `install` line in the `package` function to copy the file to the correct location inside `$pkgdir`, like so:

```bash
source=('vconsole.conf'
        'sudoers.wheel'
        'mdaffin-base.sh')
md5sums=('12733d28ff3b5352ea1c3d84b27cd6bd'
         '52719e50fbbea8255275964ba70aa0a7'
         '9463e8e19ee914684f7bd5190243aa3f')

package() {
    install -Dm 0644 vconsole.conf "$pkgdir/etc/vconsole.conf"
    install -Dm 0640 sudoers.wheel "$pkgdir/etc/sudoers.d/wheel"
    install -Dm 0755 mdaffin-base.sh "$pkgdir/etc/profile.d/mdaffin-base.sh"
}
```

With this, pacman will drop our configs in the given locations when the package is installed. It will replace them if we upgrade the package in the future and will prevent any other package from directly modifying them.

### Overwriting Existing Configs

However, most packages drop a default config file to give the users a base to start editing and do not support config directories. These are problematic as pacman will not allow us to simply install our config over them. Unfotinuatly, pacman has no nice way around this limitation but there is a slightly hacky way to work around the problem using pacman's install hooks.

Instead of dropping the config file directly into place, we can drop it alongside the existing file, that way we get no conflict with other packages. Then we can use the `post_install` and `post_upgrade` hooks to copy our config into place.

The downside of this approach is that pacman does not own or manage the lifecycle of this file, at least not with our package. It will still treat it as a config file for the original package, to be backed up or removed upon the removal of the original package. It will also not be reverted when we remove our package, although, this can be worked around by using the `post_install` hook to backup the original config and the `pre_remove` hook to restore it if you desire. If anyone knows of a better solution to this problem I would love to hear from you.

Let's take a look at how I did this with my i3 config file in my `mdaffin-desktop` package. First, we install the config file like we did in the previous section, but this time next to the original packages config.

```bash
source=('i3-config')
md5sums=('d9dcd133475af688ed86a879821c9384')
package() {
    install -Dm 0644 i3-config "$pkgdir/etc/i3/mdaffin-desktop-config"
}

```

Then we define the hooks in a separate install file, which is a bash script with at least one of the following functions defined.

* `pre_install()` - runs before the package contents are installed the first time the package is installed
* `post_install()` - after the package contents are installed the first time the package is installed
* `pre_upgrade()` - before the package contents are installed when a package is being upgraded
* `post_upgrade()` - after the package contents are installed when a package is being upgraded
* `pre_remove()` - before the package contents are removed when a package is being removed
* `post_remove()` - after the package contents are removed when a package is being removed

Let's call this file `mdaffin-desktop.install`, once again place it next to the `PKGBUILD` file. It can be called whatever you want but `<package-name>.install` is how it is conventionally named.

```bash
post_install() {
    post_upgrade
}

post_upgrade() {
    cp /etc/i3/mdaffin-desktop-config /etc/i3/config 
}
```

This copies the file from the file that was dropped by the package, to its live location during an upgrade or install, overriting the original packages config. Next, we need to tell makepkg about this install file by adding the following to `PKGBUILD`.

```bash
...
install=mdaffin-desktop.install
...
```

Which tells makepkg to include this install script in the package and pacman will run the relevant functions during an install or upgrade of the package.

### Starting Services

One more common thing we want to do is enable/start services. Typically, Arch Linux does not auto enable or start services on a package install, instead, it leaves this up to the user. Normally, this is a better approach as it lets you configure them before they are started for the first time. But we are configuring the applications with our meta-packages so why not enable and start the services as well.

The `post_install` hook from the previous section can also be used to get `systemctl` to start and enable the service on the first install, where the user is then free to disable/stop it thereafter. We could also add it to the `post_upgrade` hook to ensure it gets reenabled after an upgrade, or if you add an extra service to a package later on. For example, my desktop package has these in the install script (in addition to the things in the previous section).

```bash
post_install() {
    systemctl enable --now sddm
    systemctl enable --now avahi-daemon
}
```

Note that `--now` on a `systemctl enable` causes it to also start the service in addition to enabling it, basically equivalent to `systemctl start` and `systemctl enable` in one command. `systemctl` will also behave correctly inside a chroot environment (such as when pacstraping a system).

## Building the Package

Once you have crafted a package to your liking it is time to build it. This can be done with `makepkg` as anyone who has built a package from AUR should be aware. But instead, I will make use of its lesser-known wrapper `makechrootpkg` (part of the same package as `makepkg`). While a little bit involved it does provide a clean build for packages by building them in a fresh chroot environment rather than on your host system. The downside is that it takes some prep work and is a little slower. Feel free to continue to use `makepkg` if you want.

To save time on installing a base arch system into each chroot, `makechrootpkg` relies on a preprepared root which it copies for each package it builds. We can create this root fs by running `mkarchroot`.

```bash
mkdir -p ./chroots
mkarchroot -C /etc/pacman.conf ./chroots/root base-devel
```

`chroot` is where all of the chroots for each of our build environments will live and `root` is the base root fs `makechrootpkg` will use as a base for each environment by default.

While inside our packages directory run `makechrootpkg` and tell it what directory to use for the chroot environments (`-r`).

```bash
makechrootpkg -cur ./chroots
```

Once done you will end up with a `<package>-<version>.pkg.tar.xz` package in the current directory just like with `makepkg`.

For the last step we will install this package into a repo, such as the one I showed you how to create in my [last post]. This is done by mounting the repo, copying the package into it and running `repose` to update the package database.

```bash
mkdir -p repo
s3fs mdaffin-arch:/repo "repo" -o "nosuid,nodev,default_acl=public-read"
cp *.pkg.tar.xz "repo/x86_64/"
repose --verbose --xz --root="repo/x86_64/" mdaffin
```

You can now install this package like you do any other package with `pacman`, as long as you have your repo added to `/etc/pacman.conf`.

[last post]: /blog/archlinux-repo-in-aws-bucket/

## Git Repo and Scripting the Build

Now that we can create meta-package and publish them for use let's place these in a git repo (or another version control system if you prefer) and write a wrapper script to make building/uploading the packages even easier. You can find [my repo] on github, feel free to use it as a reference or clone it to create your own but the packages in there are tuned to my liking and so I encourage you to create your own with how you like your systems setup.

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
    new file:   pkg/mdaffin-base/mdaffin-base.install
    new file:   pkg/mdaffin-base/mdaffin-base.sh
    new file:   pkg/mdaffin-base/sudoers.wheel
    new file:   pkg/mdaffin-base/vconsole.conf
```

If anything is listed that you don't want, just add it to the `.gitignore` and run `git reset <file>`. Repeat until you are happy then commit.

```bash
git commit -m "My first package"
```

[my repo]: https://github.com/mdaffin/arch-repo

### The Build Script

The commands above can be wrapped into a helper script to make building and uploading the packages very simple. Here is the script in its entirety.

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

Some useful variables are then defined, `${@:-pkg/*}` means take all arguments, but if there are none it defaults to `pkg/*`. This allows us to build a single package, any number of packages or by default all packages. `BUCKET`, `REPO_PATH` and `REPO_NAME` should be changed to match your repo.

We create the chroot directory and init the main root fs if it does not already exist. Then loop over all the packages to build them one at a time with `makechrootpkg`, during which we delete all old package files left over from previous builds. This keeps the list of built packages down and ensures we only upload the latest build version.

Lastly, we mount the remote repo to a tempory directory and copy all packages we have built into the repo. We ignore any that already exist in the repo with `--ignore-existing` on the `rsync` command. Packages should be immutable once uploaded to the repo if you want to change a package you should increment its version number which will create a newer non-conflicting package. Ideally we should refuse to build any package that already exists in the repo with the same version and a future version of this script may do that but for now, this is good enough. The final command updates the repo database with any packages that were added.

Some of this should look familiar from the script we created in the last post. It would be handy to store both this script and the `aursync` script from my previous post inside our repo under the `./bin/` directory. I have also created a shell script that mounts the repo and drops you in a shell, auto cleaning up after you exit the shell. I found this useful for manually fixing things in the repo as I was developing everything. I will not cover it in this post as the bulk of it has been described above. You can view/download it [here][shell-wrapper].

[bash-strict-mode]: /blog/another-bash-strict-mode/
[shell-wrapper]: https://github.com/mdaffin/arch-repo/blob/71c6e07afc0a349b518444f5f383bd9dc44f05e0/bin/shell

## Summary

Now, this is quite a lot of work to set up initially and might not be worth it if you only manage one or two Arch Linux systems that you rarely change. But if you manage multiple systems and want to keep them in sync it can be worth the effort. Once you have set everything up for the first time tweaks to the packages are much simpler to make and ongoing maintenance less time consuming than manually ensuring all of your systems have your latest settings.

It is also worth noting that this does not solve the issue of keeping user files in sync. But most of the user files I want to keep in sync also have system level defaults that I can keep in sync instead lowering the number of files I need to manage in my home directory. This works best when you only have one user, or all your users are fine with the same default settings, but they can always override them within their own home directory like you normally would.
