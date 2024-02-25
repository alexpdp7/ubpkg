`ubpkg` is a package manager for **u**pstream **b**inaries.

Right now, `ubpkg` can only install packages from GitHub.

```
$ poetry run ubpkg ubpkg/repo/talosctl.ubpkg.yaml
```

See the matching [manifest](ubpkg/repo/talosctl.ubpkg.yaml).
`ubpkg` fetches GitHub releases, looks for a specific binary, and installs the binary to `~/.local/bin`.

# TODO

* Make a proper module installable with `pipx` and provide instructions.
* Support manifests from URLs (run `ubpkg install https://example.com/package.ubpkg.yaml`).
* Support extra repositories (run `ubpkg add-repo https://example.com/repo.git`).
* Distribute as a single binary (which could be updated via itself) using [PyOxidizer](https://pyoxidizer.readthedocs.io/en/stable/pyoxidizer.html).
* Make `ubpkg` extensible, by detecting extra `ubpkg-*` commands.
  With `pipx inject` additional Python modules can be injected into a `pipx`-installed `ubpkg`.
* Support architectures other than x86 and operating systems other than Linux.
