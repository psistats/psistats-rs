[![Build Status](https://dev.psikon.org/jenkins/buildStatus/icon?job=psistats%2Fpsistats-rs%2Fdevelop)](https://dev.psikon.org/jenkins/job/psistats/job/psistats-rs/job/develop)

# psistats

Psistats is small and simple reporting tool. It can report on a variety of different statistics including CPU and memory usage. It is inspired by the venerable [collectd](https://github.com/collectd/collectd).

It is built upon a plugin architecture to make it easier to add more functionality.

## Installation

### Debian-based OSes

First install the necessary key:
```
$ curl -sS https://dev.psikon.org/pubkey.gpg | sudo apt-key add -
```

Then you can choose "testing" or "main" debian repos. "testing" will be synced with the "develop" branch on git. "main" will be synced with the "master" branch on git.

For stable releases:

```
$ echo "deb https://debrepo.psikon.org/ psikon-main main" | sudo tee /etc/apt/sources.list.d/yarn.list
```

For develop releases:

```
$ echo "deb https://debrepo.psikon.org/ psikon-testing testing" | sudo tee /etc/apt/sources.list.d/yarn.list
```

Then install:

```
$ apt-get update
$ apt-get install psistats
```

You can then configure psistats by editing `/etc/psistats.conf`.

### Windows

Currently, artifacts are only available from the build machine. The last successful master builds are available here:
https://dev.psikon.org/jenkins/job/psistats/job/psistats-rs/job/master/lastSuccessfulBuild/artifact/target/release/artifacts/

The sensors plugin is not available for windows.

### Plugin Authoring

Documentation is not yet published. You can run `cargo doc` however to generate your own documentation.

### Building

This project should build as is for 64bit linux and windows. To create 32bit and 64bit ARM builds on Linux, you must install the following packages:

* qemu-user
* gcc-aarch64-linux-gnu
* libc6-dev-arm64-cross
* gcc-arm-linux-gnueabihf
* libc6-dev-armhf-cross
