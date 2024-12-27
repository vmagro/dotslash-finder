dotslash-finder
===============

[Dotslash](https://dotslash-cli.com/) is a useful tool to distribute
repo-deterministic binaries. One can check a simple Dotslash JSON file into a
source controlled repository and get a deterministic version of some binary.

However, invoking these binaries is not very ergonomic. This small tool fills
that gap for me. It behaves as a multi-call (aka busybox-like) binary that can
be linked somewhere on your `$PATH` for some well-known name (eg `buck2`). When
invoked, it will walk up the directory free from the current directory and
invoke a dotslash binary of the same name that it finds along the way.
