`ubpkg` is a package manager for **u**pstream **b**inaries.

# Installation

```
$ pipx install git+https://github.com/alexpdp7/ubpkg.git
```

# Usage

Right now, `ubpkg` can only install packages from GitHub.

```
$ ubpkg ubpkg/repo/talosctl.ubpkg.yaml
$ ubpkg talosctl
```

See the matching [manifest](ubpkg/repo/talosctl.ubpkg.yaml).
`ubpkg` fetches GitHub releases, looks for a specific binary, and installs the binary to `~/.local/bin`.

# Using without installing

`pipx` can run `ubpkg` without installing it.

```
$ pipx run --spec git+https://github.com/alexpdp7/ubpkg.git ubpkg ...
```

# TODO

* Investigate [mise/asdf plugins that use git ls-remote to fetch repo tags to avoid GitHub API rate limits](https://github.com/mise-plugins/mise-jless/blob/main/lib/utils.bash).
* Support manifests from URLs (run `ubpkg install https://example.com/package.ubpkg.yaml`).
* Support extra repositories (run `ubpkg add-repo https://example.com/repo.git`).
* Distribute as a single binary (which could be updated via itself) using [PyOxidizer](https://pyoxidizer.readthedocs.io/en/stable/pyoxidizer.html).
* Make `ubpkg` extensible, by detecting extra `ubpkg-*` commands.
  With `pipx inject` additional Python modules can be injected into a `pipx`-installed `ubpkg`.
* Support architectures other than x86 and operating systems other than Linux.
