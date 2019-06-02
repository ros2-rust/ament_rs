# ament_rs

This crate is a client for ament which is a system for cataloging and referencing resources distributed by software packages used by ROS2.

## Naming conventions

Functions with a name ending with `_from` take an additionnal `prefixes` argument.
These functions also have a variant named without the `_from` ending which use the prefixes retrieved from the `AMENT_PREFIX_PATH` environment variable.

Functions starting with `list_` return an iterator instead of a collection.

## Examples

```rust
use ament_rs::*;
println!("{:#?}", get_packages_with_prefixes());
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