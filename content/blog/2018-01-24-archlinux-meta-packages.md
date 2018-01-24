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

What I wanted was a way to manage my whole system, to make it easy and quick to install a new system and to keep the system up to date with my latest configs and packages that I use. For a while, I tried using [SaltStack] to manage my systems and users and while effective for a while it was more maintenance than was worth it.

In my [last post], I talked about how you can create and maintain a custom Arch Linux repository for AUR packages. But there was something else I wanted to use this repo for - creating and maintaining [meta packages] to solve some of the issues above.  In this post, I will talk about these meta packages, what they are, how to create them and how they can solve the issues above.

[dotfile managers]: https://wiki.archlinux.org/index.php/Dotfiles 
[last post]: /blog/archlinux-repo-in-aws-bucket/
[meta packages]: https://wiki.archlinux.org/index.php/creating_packages#Meta_packages_and_groups

## How Pacman installs packages and basic package structure


## Meta-packages: why and how?

## Some examples problems and hacks

## Building a package and adding to a repo

## Conclusion
