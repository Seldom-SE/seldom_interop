// This is a runnable version of the example in README.md

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MyPlugin::<Transform>(default()))
        .add_startup_system(init)
        .add_system(log)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Transform::default());
}

fn log(transforms: Query<&Transform>) {
    for transform in &transforms {
        info!("{}", transform.translation);
    }
}
