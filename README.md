# TUXEDO InfinityBook Gen10 plugin for CoolerControl

This is a device plugin for [CoolerControl](https://docs.coolercontrol.org) that implements support for controlling system fans on [TUXEDO](https://www.tuxedocomputers.com) InfinityBook Gen10 laptops. After installing this plugin, CoolerControl will be able to monitor the laptop fan speeds as well as apply custom fan curves.

This plugin expects you to have the official TUXEDO kernel modules installed and loaded to work, as we use the same driver APIs that the official [TUXEDO Control Center (TCC)](https://github.com/tuxedocomputers/tuxedo-control-center) application uses. Our own daemon is provided written in Rust as an alternative to TCC's `tccd`, which is written in TypeScript.

To get the best support you are probably better off just using TCC. But for myself, I don't need TCC to manage CPU performance (I use tlp for that), so the only thing I use it for is fan control. And CoolerControl is a bit nicer and more customizable than TCC for this. Not to mention, CoolerControl uses less system resources in the background than TCC does.

## Hardware support

I've only tested this on a TUXEDO InfinityBook Max 15 Gen10 AMD, because that is the device that I have. But I suspect it will work for other Gen10 InfinityBooks as well. The plugin could probably be extended to support more TUXEDO laptops, but I don't have a need for that.

## Usage

Installing the plugin can be done by cloning this repository and running

```sh
make install
```

You will need working Rust protobuf compilers installed for building the plugin.
