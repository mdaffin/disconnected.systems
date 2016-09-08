---
layout: post
title: Introduction to git2go
description: A quick look at how to use git2go
tags: [go, golang, git]
---

## Opening the Repository

Everything in git2go revolves around the
[Repository](https://godoc.org/github.com/libgit2/git2go#Repository) struct. It
is the accessor to the on disk git repo and there are three ways to create it. 

### Opening a local repo

If you have already initlized of cloned a repo locally you can simply open it with.

```go
repo, err := git.OpenRepository("web")
```

### Creating a New Repository

If you do not have a repo to start with you can create a new empty one; this is
equlivent to the `git init` command.

```go
repo, err := git.InitRepository("my-new-repo", false)
```

You can create a bear repository be setting the second parameter to true; like passing `--bare` to `git init`.

### Cloning a repo

Cloning a public repo is stright forward, simply call `git.Clone()` with the
url, local destantion and any options.

```go
repo, err := git.Clone("git://github.com/gopheracademy/gopheracademy-web.git", "web", &git.CloneOptions{})
```

For private repos then gets a little more complicated. You require two
callbacks, one to handle certificate validation and the other to handle the
users credentials.

To simply ignore the host certificate (insecure, but easy to get started) you
can use the following callback function.

```
func IgnoreCertificateCallback(cert *git.Certificate, valid bool, hostname string) git.ErrorCode {
	return 0
}
```

Next we need a callback to load the users credentials, here is a simply method
that pulls out the default ssh keypair for the user.

```
func DefaultSSHKeyCallback(url string, username string, allowedTypes git.CredType) (git.ErrorCode, *git.Cred) {
	ret, cred := git.NewCredSshKey(username, path.Join(os.Getenv("HOME"), ".ssh/id_rsa.pub"), path.Join(os.Getenv("HOME"), ".ssh/id_rsa"), "")
	return git.ErrorCode(ret), &cred
}
```

To use other credential (such as username/password) you can use one of the
following instead.

```
NewCredDefault()
NewCredSshKey(username string, publickey string, privatekey string, passphrase string)
NewCredSshKeyFromAgent(username string)
NewCredUserpassPlaintext(username string, password string)
```

We can then pass these callbacks to the `git.Clone()` methods via the
`git.CloneOptions` struct.

```
co := &git.CloneOptions{
	FetchOptions: &git.FetchOptions{
		RemoteCallbacks: git.RemoteCallbacks{
			CredentialsCallback:      DefaultSSHKeyCallbacklk,
			CertificateCheckCallback: IgnoreCertificateCallback,
		},
	},
}

repo, err := git.Clone(url, dest, co)
```
	
## Loop over branches
```go
branches, err := repo.NewBranchIterator(git.BranchLocal)
if err != nil {
	panic(err)
}
err = branches.ForEach(func(branch *git.Branch, branchType git.BranchType) error {
	fmt.Printf("%#v  %#v\n", branch, branchType)
	name, err := branch.Name()
	if err != nil {
		panic(err)
	}
	fmt.Println(name)
	return nil
})

if err != nil && !git.IsErrorCode(err, git.ErrIterOver) {
	panic(err)
}
```

## Commiting a change

## Pushing commits

## Pulling changes

## Creating a branch

## Merging branches

## Getting the current ref
```go
ref, err := repo.Head()
if err != nil {
	panic(err)
}

if ref.IsBranch() {
	name, err := ref.Branch().Name()
	if err != nil {
		panic(err)
	}
	fmt.Printf("Current Head: %s\n", name)
} else if ref.IsTag() {
	obj, err := ref.Peel(git.ObjectTag)
	if err != nil {
		panic(err)
	}
	tag, err := obj.AsTag()
	if err != nil {
		panic(err)
	}
	fmt.Printf("Current Head: %s\n", tag.Name())
} else {
	fmt.Printf("Current Head: %s\n", ref.Name())
}
```
