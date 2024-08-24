# Upstream binaries package manager

`ubpkg` is a command-line tool to install software.
`ubpkg` can install upstream binaries, such as those attached to GitHub releases.

## Usage

### Installation

```
$ wget https://github.com/alexpdp7/ubpkg/releases/latest/download/ubpkg-linux-x86_64 -O ~/.local/bin/ubpkg  # or some other directory in your $PATH you can write to
$ chmod +x ~/.local/bin/ubpkg
```

### Usage

[Available packages](repo/).

```
$ ubpkg PACKAGE...
```

Downloads the packages and installs binaries to `~/.local/bin`.

## About

I find myself using more and more binaries built by the developers of some software (upstream).
Linux distributions cannot possibly include updated packages for everything, and nowadays much software is packaged as self-contained, relocatable packages, that can just be downloaded and executed.

`ubpkg` uses recipes to install software.

Recipes use [the Starlark language](https://github.com/bazelbuild/starlark/blob/master/spec.md).
Starlark is very similar to Python, but Starlark code cannot have side-effects by default.
`ubpkg` introduces some functions to Starlark, so that recipes can download files, unpack archives, and more.
But `ubpkg` recipes can execute only those controlled functions.

### What is your development/contributions policy?

I am developing `ubpkg` as a personal helper.
I intend to develop and maintain only what I need.
However, I will gladly look at PRs, and might merge anything as long as we agree on a maintenance policy.
(Before making a big effort, I would recommend you open an issue or discussion first, though.)

I think new recipes could be added, but it is likely new recipes will need extra Starlark functions.

Also, I'm still experimenting with the API, so Starlark functions are bound to have breaking changes.
The more packages in the repository, the more likely the API is close to being complete and stable.

Also, I'm a Rust novice: any contribution to improve the quality of the code will be very welcome!

### Are `ubpkg` recipes safe?

Although `ubpkg` recipes cannot execute arbitrary code, they can still download and put in your path malicious binaries.

### Does this support Windows or macOS?

Not yet.

`ubpkg` is designed to be cross-platform (it does not even depend on `bash`), and some code even supports Windows.
However, I do not think it is usable right now outside Linux.
Right now I do not work with macOS or Windows frequently enough to support any of them, but I welcome maintainers and contributions.

### How does this compare to...

Compared to everything on this list, `ubpkg`:

* Has very few packages.
  It is unlikely that it has the packages you need.
  (But adding packages can be simple.)
* It is very new and unstable, and lacks basic features.
  (For example, it does not keep a list of installed packages, so it cannot update all installed packages.)
* There's likely software that it cannot install.
  (Right now, anything that does not have self-contained binaries available.)
* Does not have any versioning support.
  Right now, it always finds the latest binary.
  (On the other hand, this means that recipes will not need updates to pick the latest version available.)

Likely `ubpkg` is not worth using, and you will be better served by the tools listed below, and others.
Feel free to create a PRs to add other tools, or to correct incorrect facts (I am not superfamiliar with all of those).

#### Homebrew

* Homebrew does not have an inclusive design for Windows.
* Homebrew modifies `/usr/local`, `ubpkg` stays strictly in your home directory.

#### https://github.com/Rishang/install-release

* `install-release` seems to target only GitHub releases.
  Right now `ubpkg` has only packages from GitHub releases, although it has functions that could install software from other sources.
* I'm not sure `install-release` has a good strategy to handle the different ways binaries are packaged in GitHub releases (e.g. different naming conventions, archive formats, etc.)

#### https://github.com/alexellis/arkade/

* Arkade seems to have a specific focus on Kubernetes tools, although [it packages a wide variety of tools](https://github.com/alexellis/arkade/tree/master?tab=readme-ov-file#catalog-of-clis).

#### asdf/mise/...

* These tools are more designed to install tools inside a project environment, although they support global installation.
* They require to alter your path with shims (I think).
* Plugins seem to require bash (I think).

#### WinGet / Homebrew Cask

Good eye, `ubpkg` is likely closer to WinGet and Homebrew Cask.
However, both only support Windows and macOS, respectively.

#### https://github.com/xplshn/dbin

dbin uses [Toolpacks](https://github.com/Azathothas/Toolpacks), a project that builds statically linked binaries.
Therefore, it does not use upstream binaries.
dbin supports only Linux, although Toolpacks provides Windows binaries (but does not provide macOS binaries).

### Is this a good idea?

I do not know yet.

* Rust is a complex language, and embedding Starlark is not easy.
  The [starlark-rust crate](https://github.com/facebook/starlark-rust) looks like something that is just meant for internal use in Facebook.
  [The only significant user I have found is considering replacing Starlark](https://github.com/indygreg/PyOxidizer/issues/444).
  Probably this would work better using some other embeddable language (perhaps Lua, Javascript, Typescript...?) and/or another base language.
