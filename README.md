[![Build Status](https://dev.psikon.org/jenkins/buildStatus/icon?job=psistats%2Fpsistats-rs%2Fmaster)](https://dev.psikon.org/jenkins/job/psistats/job/psistats-rs/job/master)

# psistats

Psistats is small and simple reporting tool. It can report on a variety of different statistics including CPU and memory usage. It is inspired by the venerable [collectd](https://github.com/collectd/collectd).

It is built upon a plugin architecture to make it easier to add more functionality.

Still in active development.

## Installation

### Debian-based OSes

```
$ curl -sS https://dev.psikon.org/pubkey.gpg | sudo apt-key add -
$ echo "deb https://debrepo.psikon.org/ psikon-testing testing" | sudo tee /etc/apt/sources.list.d/yarn.list
```

You can then configure psistats by editing ```/etc/psistats.conf```.

### Windows

Currently, artifacts are only available from the build machine. The last successful master builds are available here:
https://dev.psikon.org/jenkins/job/psistats/job/psistats-rs/job/master/lastSuccessfulBuild/artifact/target/release/artifacts/

### Plugin Authoring


### Building

This project should build as is for 64bit linux and windows. To create 32bit and 64bit ARM builds on Linux, you must install the following packages:

* qemu-user
* gcc-aarch64-linux-gnu
* libc6-dev-arm64-cross
* gcc-arm-linux-gnueabihf
* libc6-dev-armhf-cross
