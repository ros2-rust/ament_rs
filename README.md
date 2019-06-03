# ament_rs

This crate is a client for ament which is a system for cataloging and referencing resources distributed by software packages used by ROS2.

[![crate.io](https://img.shields.io/crates/v/ament_rs.svg)](https://crates.io/crates/ament_rs)
[![docs.rs](https://docs.rs/ament_rs/badge.svg)](https://docs.rs/ament_rs)

## Examples

```rust
use ament_rs::*;
println!("{:#?}", Ament::new()?.get_packages_prefixes());
```

This snippet will print a list of packages with the prefixes they were found in, depending of the value of the `AMENT_PREFIX_PATH` environment variable on your system.

```none
{
    "ros_core": [
        "/opt/ros/crystal",
        "/opt/ros/bouncy",
    ],
    "console_bridge_vendor": [
        "/opt/ros/dashing",
        "/opt/ros/crystal",
    ],
    "ament_cmake_export_interfaces": [
        "/opt/ros/dashing",
        "/opt/ros/crystal",
        "/opt/ros/bouncy",
    ],
}
```

