[![crates.io](https://img.shields.io/crates/v/drone-ti-sl-map.svg)](https://crates.io/crates/drone-ti-sl-map)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# drone-ti-sl-map

Texas Instruments SimpleLink peripheral mappings for Drone, an Embedded Operating
System.

This crate uses
[CMSIS-SVD](https://arm-software.github.io/CMSIS_5/SVD/html/index.html) files,
generated from TargetDB XML Specification files provided by 
[Texas Instruments](http://www.ti.com/) using the 
[tixml2svd](https://github.com/dhoove/tixml2svd) tool, to automatically
generate Drone register and interrupt bindings. However only
the corresponding Product Specification is the single source of truth. A
difference between this crate bindings and the Product Specification is
considered a bug. Fixing such a bug is *not a breaking change*.

This crate re-exports the contents of [`drone_cortex_m::map`] module and is
a drop-in replacement for it.

## Supported Devices

| `ti_sl_mcu`  | Core name              | Product specification                                                 |
|--------------|------------------------|-----------------------------------------------------------------------|
| `cc2538`     | ARM® Cortex®-M3 r1p0   | [SWRU319C](http://www.ti.com/lit/ug/swru319c/swru319c.pdf)            |

`ti_sl_mcu` config flag should be set at the application level according to
this table.

## Documentation

- [Drone Book](https://book.drone-os.com/)
- [API documentation](https://api.drone-os.com/drone-ti-sl-map/0.11/)

The API documentation intentionally skips auto-generated [`reg`] and [`thr`]
bindings. Otherwise it would use several gigabytes of space and would be
very slow to render in a browser. One should refer to the Product
Specification instead. And to get an idea of what the API looks like on the
Drone side, look at the [`drone_cortex_m::map`] module documentation.

## Usage

Place the following to the Cargo.toml:

```toml
[dependencies]
drone-ti-sl-map = { version = "0.11.0", features = [...] }
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
