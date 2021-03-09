# psistats

Psistats is small and simple reporting tool. It can report on a variety of different statistics including CPU and memory usage. It is inspired by the venerable [collectd](https://github.com/collectd/collectd).

It is built upon a plugin architecture to make it easier to add more functionality.

Still in active development.

### Building

This project should build as is for 64bit linux and windows. To create 32bit and 64bit ARM builds on Linux, you must install the following packages:

* qemu-user
* gcc-aarch64-linux-gnu
* libc6-dev-arm64-cross
* gcc-arm-linux-gnueabihf
* libc6-dev-armhf-cross
