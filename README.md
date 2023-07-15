# `seldom_interop`

[![Crates.io](https://img.shields.io/crates/v/seldom_interop.svg)](https://crates.io/crates/seldom_interop)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_interop#license)
[![Crates.io](https://img.shields.io/crates/d/seldom_interop.svg)](https://crates.io/crates/seldom_interop)

`seldom_interop` is a crate featuring interoperability traits for Bevy components. This helps
apps and third-party plugins, that use alternative components to represent concepts
such as position instead of using the built-in components such as `Transform`, interoperate
with other third-party plugins that operate on such components. It currently only features
traits for positional components, and I will add more traits as needed. If you want
other interoperability traits for your plugin, please submit an issue!

This plugin is still being maintained, but it may stop being maintained if I decide to merge its
functionality into `seldom_map_nav`. The license is permissive, so feel free to fork or copy the
source code into your project!

## Features

* Position traits: `Position2` and `Position3`

## Usage

Add to your `Cargo.toml`

```toml
# Replace * with your desired version
[dependencies]
seldom_interop = "*"
```

For apps and plugins that add components for which you would like to add interoperability,
implement relevant traits for your components. For plugins that would like to use components
in an interoperable way, add a type parameter to your plugin and relevant systems. For example:

```Rust
use std::marker::PhantomData;

use bevy::prelude::*;
use seldom_interop::prelude::*;

struct MyPlugin<T: Position2<Position = Vec2> = Transform>(PhantomData<T>);

impl<T: Position2<Position = Vec2>> Plugin for MyPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(my_system::<T>);
    }
}

fn my_system<T: Position2<Position = Vec2>>(mut positions: Query<&mut T>, time: Res<Time>) {
    for mut position in &mut positions {
        let mut position_vec = position.get();
        position_vec.x += time.delta_seconds();
        position.set(position_vec);
    }
}
```

## Compatibility

| Bevy | `seldom_interop` |
| ---- | ---------------- |
| 0.10 | 0.3              |
| 0.9  | 0.2              |
| 0.8  | 0.1              |

## License

`seldom_interop` is dual-licensed under MIT and Apache 2.0 at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.