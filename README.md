# rules_rust clippy issue [#2988](https://github.com/bazelbuild/rules_rust/issues/2988)

This repo contains a go program that gets cross compiled and then a rust
program that provides access to that compiled go program via runfiles.
The surprising situation is that the repo builds just fine, but it fails
to run clippy checks.

See
```
bazel test //...
```
vs
```
bazel build //... --output_groups=clippy_checks --aspects=@rules_rust//rust:defs.bzl%rust_clippy_aspect
```
