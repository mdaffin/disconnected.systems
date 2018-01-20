---
title: "Hosting an Archlinux Repo in an Amazon S3 Bucket"
date: 2018-01-19T20:18:17Z
draft: true
---

When you use Archlinux for any length of time you start collecting sets of
[AUR] packages that you frequently use. Now, Archlinux has loads of [AUR
helpers] that make managing packages in AUR painless. But when you start using
arch on multiple systems it becomes annoying to rebuild AUR packages on each
system. In this post, I will show you how to use an Amazon S3 bucket to create
a cheap, low maintenance Archlinux repository. As well as making use of the
aurutils package to make building and upgrading AUR packages a painless
exercise.

Although everything we are going to do in this post will fit inside the AWS
free tier, it only lasts for 12 months. Make sure to delete any resources you
create once you are done to avoid an unexpected charge from AWS way in the
future. Even without the free tier, it should only cost no more than a few
dollars a month to maintain the bucket. You can also use alternatives like
Digital Oceans Spaces, Google Cloud or a static file web server.

## Dependencies

We only require a few packages to get us going, only `aurutils` needs to be
installed from AUR and will be the only package we are required to
build/install manually.

* [aurutils]: a set of utilities that make it easy to manage/update a repo with
  AUR packages.
* [s3fs-fuse]: allows us to mount an s3 bucket locally so we can add/update the
  repo using local tools.
* [repose]: an alternative to add-repo, but makes deploying the repo easier
  inside a bucket. Aurutils automatically uses repose if it is installed, so we
  will not explicitly use. ^
* base-devel: needed to build aurutils and other packages.

To install all of these run the following.

```bash
sudo pacman -S --needed repose s3fs-fuse base-devel
wget https://aur.archlinux.org/cgit/aur.git/snapshot/aurutils.tar.gz
tar -xf aurutils.tar.gz
cd aurutils
makepkg -sci
```

If you get the following error while running `makepkg`.

```bash
==> Verifying source file signatures with gpg...
    aurutils-1.5.3.tar.gz ... FAILED (unknown public key 6BC26A17B9B7018A)
==> ERROR: One or more PGP signatures could not be verified!
```

Simply download the missing key with the following before running `makepkg`
above.

```bash
gpg --recv-key 6BC26A17B9B7018A
```

[aurutils]: https://github.com/AladW/aurutils
[s3fs-fuse]: https://github.com/s3fs-fuse/s3fs-fuse
[repose]: https://github.com/vodik/repose

^ The reason we use repose instead of repo-add is that it just works inside a
bucket, were repo-add creates symlinks which don't always work when served via
a bucket

## Creating the Amazon S3 Bucket

Sign in to [Amazon's console][Amazon S3] and head to the [Amazon S3] interface.
You will be required to enter your credit card details in order to create the
bucket, this should be free for the first year if you stay under 5GB of storage
and [fairly cheap][amazon pricing] after that.

Click on the create bucket button.

![Create Bucket](/images/amazon-s3/01-create-bucket.png)

Name your bucket and select the region you want to host it in.

![Name the Bucket](/images/amazon-s3/02-name-bucket.png)

Then click on Next twice to get to (3) Set permissions and make the bucket
public. This will allow anyone in the world to read the bucket and thus allows
Pacman to download the packages anonymously.

![Public Bucket](/images/amazon-s3/03-public-bucket.png)

After you should have one public bucket listed like so.

![Bucket List](/images/amazon-s3/04-bucket-list.png)


[Amazon S3]: https://s3.console.aws.amazon.com/s3/home?region=us-east-1
[amazon pricing]: https://aws.amazon.com/s3/pricing/

## Access credintials

Under your account select *My Security Credentials*.

![Account Settings](/images/amazon-s3/05-account-settings.png)

You might get a warning about creating account credentials.

![Account Warning](/images/amazon-s3/06-account-warning.png)

This is simply informing you that the credentials we are creating will give
whoever has them full access to your account. You can create more restricted
credentials if you want by creating a user and giving it access to the bucket
we created before. But for simplicity, I am going to stick with the root
credentials. **Do not upload these credentials anywhere public or insecure**

Expand the *Access keys* menu and click *Create New Access Key*.

![Create Access Key](/images/amazon-s3/07-create-access-key.png)

This will create an access key for you, make sure you copy it somewhere
locally, once you close this screen you will lose the secret part of the key
and will have to generate a new key.

![Access Key](/images/amazon-s3/08-access-key.png)

Save this key to ~/.passwd-s3fs in the form

```ini
bucket:access_key:secret_key
```

Like the following.

```ini
mdaffin-arch:AKIAID7W4RGIV46DPSEA:Uuf3GvIhkJodtSgRoxoXUfxgWSNYGA6ekZv/niZK
```

*Note that this is not a real key, replace it with your own*

And ensure it is only readable by your user

```bash
chmod 0600 ~/.passwd-s3fs
```

## Mounting the Bucket

Amazon S3 Buckets can be mounted locally with the [s3fs-fuse] utilities. This
makes the bucket act like any local filesystem and allows us to run commands to
manipulate files. This is very useful for running aurutils to build the
packages locally, upload them to the bucket and use repose to add them to the
repo.


```bash
mkdir -p bucket
s3fs "${BUCKET}" "bucket" -o "nosuid,nodev,default_acl=public-read"
```

Be aware that operations inside the `repo` can be slow as they require network
calls. Also, note that actions inside this directory will count towards your
usage limits so avoid doing crazy things like compiling packages inside it. 

We have mounted the directory with the `public-read`. This ensures that
anything we upload to the bucket is readable by everyone. This is so that
Pacman is able to read/download the packages without needing a
username/password. You should not store any secrets inside this bucket.

*Note: if you are using digital ocean spaces or another s3 compatible service
you can specify an alternative API URL to use with `url=<url>` in the `-o`
options. Such as `-o url=https://ams3.digitaloceanspaces.com,nosuid,...`.*

Now we can create a directory inside the bucket to contain the repo.


```bash
mkdir -p bucket/repo/x86_64
```

If you look inside the bucket on the Amazon web console you should see these
directories appear. You can use any path you like for this, but it is common to
end it in the architecture you are targeting (64bit in my case). Feel free to
create additional repositories if you want to target multiple architectures
though I will not cover cross-compiling for other architectures in this post.

## Aurutils - Building and Managing Packages

Aurutils contains a suite of utilities that can be used to manage a repo of AUR
packages. The two main utilities we will use are `aursearch`, which can search
AUR for packages that match a given pattern.

```bash
% aursearch aurutils                                                              :(
aur/aurutils 1.5.3-5 (55)
    helper tools for the arch user repository
aur/aurutils-git 1.5.3.r234.g15ef2ab-1 (5)
    helper tools for the arch user repository
```

And `aursync` which will download and build packages and ensure packages in the
repo are up to date.

For `aursync` to work, we need to add a repo to `/etc/pacman.conf`

```ini
[mdaffin]
SigLevel = Optional TrustAll
Server = https://s3.eu-west-2.amazonaws.com/mdaffin-arch/repo/x86_64/
```

Give your repo a unique name by replacing `[mdaffin]` with something else.
Change the URL to that of your bucket/repo path. You can get the exact URL by
creating a file inside the directory and getting a link to that file from the
[amazon web console].

Now we can create the repo and upload our first package to it. For this, we are
going to rebuild the aurutils package as it will be handy to have that stored
in our repo.


```bash
aursync --repo mdaffin --root bucket/repo/x86_64 aurutils
```

Replace mdaffin with the name of your repo, this must match the section in
`/etc/pacman.conf`. Since we have a remote repo we need to tell `aursync` were
to place the files using `--root <dir>` pointing it to the directory we created
inside our mounted bucket.

If all goes well you should end up with the package and repo database inside
the bucket.

```bash
% ls bucket/repo/x86_64
aurutils-1.5.3-5-any.pkg.tar.xz  mdaffin.db  mdaffin.files
```

They should also be visible on the Amazon Web Console and fetchable via Pacman.

```bash
% sudo pacman -Syy
% pacman -Ss aurutils                                                              :(
mdaffin/aurutils 1.5.3-5 [installed]
    helper tools for the arch user repository
```

And that's it, you have created a repo inside an amazon s3 bucket. You can add
more packages to this repo using the `aursync` command above.

To check for and update all the packages in the repo simply add `-u` to the
`aursync` command.

```bash
aursync --repo mdaffin --root bucket/repo/x86_64 -u
```

Finally, when you are done unmount the bucket with `fusermount`.

```bash
fusermount -u bucket
```

[amzon web console]: https://s3.console.aws.amazon.com/s3/home

## Wrapper Script

We can automate most of this with a simple wrapper script around `aursync`.
Simply save this script somewhere, replace the `BUCKET`, `REPO_PATH` and
`REPO_NAME` variables with your own and call it like you would `aursync`:
`./aursync_wrapper PACKAGE` or `./aursync_wrapper -u`.

```bash
#!/bin/bash
# Wraps aursync command to mount an amazon s3 bucket which contains a repository
set -uo pipefail
trap 's=$?; echo "$0: Error on line "$LINENO": $BASH_COMMAND"; exit $s' ERR

BUCKET=mdaffin-arch
REPO_PATH=repo/x86_64
REPO_NAME=mdaffin

exit_cmd=""
defer() { exit_cmd="$@; $exit_cmd"; }
trap 'bash -c "$exit_cmd"' EXIT

repo="$(mktemp -d)"
defer "rmdir '$repo'"

s3fs "${BUCKET}" "$repo" -o "nosuid,nodev,default_acl=public-read"
defer "fusermount -u '$repo'"
mkdir -p "$repo/${REPO_PATH}"

aursync --repo "$REPO_NAME" --root "$repo/$REPO_PATH" "$@"
```

## AWS S3 Alternitives

If you don't wish to use Amazon buckets there are some alternatives such as
[Digital Ocean Spaces] or [Google Cloud Buckets] that can be used inplace. Some
are compatible with the s3 API and thus can be used with the instructions above
while others require a different fuse wrapper. For example, if you had your own
static file web server you could use could use [sshfs] client instead.

[Digital Ocean Spaces]: https://m.do.co/c/8fba3fc95fef
[Google Cloud Buckets]: https://cloud.google.com/storage/
[sshfs]: https://github.com/libfuse/sshfs
