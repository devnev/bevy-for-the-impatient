# On-Screen

> Show a coloured and lit cube from a raised viewpoint

Add a behaviour for spawning a camera:

```rust
fn spawn_camera(mut commands: Commands) {
    let camera_pivot = commands
        .spawn((TransformBundle {
            local: Transform::default().with_rotation(
                Quat::from_axis_angle(Vec3::Y, (20 as f32).to_radians())
                    * Quat::from_axis_angle(Vec3::X, (30 as f32).to_radians()),
            ),
            ..default()
        },))
        .id();
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .set_parent(camera_pivot);
}
```

Add a behaviour for spawning a lit box:

```rust
fn spawn_lit_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cube::default())),
        material: materials.add(Color::BLUE.with_l(0.2).into()),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::default().looking_to(
            Vec3 {
                x: 3.0,
                y: -1.0,
                z: 1.0,
            },
            Vec3::Y,
        ),
        ..default()
    });
}
```

Change main to create a Bevy app that will call the above functions during startup:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_lit_box)
        .run();
}
```

And now run the app:

```sh
cargo run
```

## Navigation

[Code](./2-on-screen) / [Prev](1-install.md) / [Next](3-spin.md)

## Detailed Walk-Through

> Add a behaviour for spawning a camera

If you've read any amount of Bevy-related documentation, you'll almost certainly
have run across mention of "ECS", or "Entity Component Systems", not to be
confused with an "ECS", an "Entity Component System". To disambiguate and help
intuition, the "systems" in Bevys ECS are referred to "behaviours" in this book.

```rust
fn spawn_camera(mut commands: Commands)
```

Behaviours (or "systems" in Bevy/ECS parlance) can only specify certain types of
parameters, which Bevy will inject when the behaviour is invoked. `spawn_camera`
is a simple behaviour with only the command queue, specified with the type `mut
Commands`. Other valid types are resource and queries, which we will be using
later on.

```rust
    let camera_pivot = commands
```

To get things started we're going to have a slightly top-down camera pointing at
a target, and later we'll add input to revolve around the target. A pivot lets
us manage the target and rotation around it easily.

```rust
        .spawn((TransformBundle {
            local: Transform::default().with_rotation(
                Quat::from_axis_angle(Vec3::Y, (20 as f32).to_radians())
                    * Quat::from_axis_angle(Vec3::X, (30 as f32).to_radians()),
            ),
            ..default()
        },))
```

`spawn` creates a new entity with the provided components.  A "Bundle" is a
bundle of components commonly created together. In this case, we create the
pivot with a local transformation that points it slightly down (by rotating
around the X axis) and to the side (by rotating around the Y axis). Y is the up
axis because that is the up axis we select when we spawn the camera in a moment.

```rust
    let camera_pivot = commands
        .spawn(...)
        .id();
```

We track the pivot's entity ID so that we can make the camera a child of the
pivot in the next step.

```rust
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
```

We specify the camera position along the negative Z, but looking back towards
origin, with Y up. If this doesn't make sense, check out the [discussion on
coordinate systems](https://github.com/bevyengine/bevy/discussions/1979).

```rust
    commands
        .spawn(...)
        .set_parent(camera_pivot);
```

Here we make the camera a child of the pivot, so that it inherits the pivot's
transformation.

```rust
fn spawn_lit_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
```

Already we now see a behaviour receiving resources in addition to the command
queue. Resources are singletons living outside of the ECS system, and are best
used for having shared data between entities/components.

As you'll see on the next lines, we modify these resources `spawn_lit_box`, so
they're `mut` parameters of type `ResMut`, which also ensures there's no
unsafe concurrent access to the resources.

```rust
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cube::default())),
        material: materials.add(Color::BLUE.with_l(0.2).into()),
        ..default()
    });
```

Good old cube. This one is a dark blue.

Although the meshes here are created on the fly (for now), we put them into a
mesh assets manager resource, as PbrBundle references its mesh via a handle to a
mesh asset in the mesh assets manager. Same concept for the material.

```rust
    commands.spawn(DirectionalLightBundle {
        transform: Transform::default().looking_to(
            Vec3 {
                x: 3.0,
                y: -1.0,
                z: 1.0,
            },
            Vec3::Y,
        ),
        ..default()
    });
```

And to light up the scene, we spawn a directional light. The direction is chosen
arbitrarily to make the cube's shading look interesting.


```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_lit_box)
        .run();
}
```

The `Startup` parameter is the label for what Bevy calls a "schedule",
essentially a collection of behaviours. `add_system` adds our behaviours to the
`Startup` schedule, and the bevy `Main` system (registered by `DefaultPlugins`)
runs the `Startup` schedule's systems once, before it runs any main-loop
schedules.

```sh
cargo run
```

This should be quick, at most a few seconds, as all dependencies have already
been compiled and cached when we ran `cargo build` in the previous chapter. An
application window should open, showing a shaded dark-blue box from a slightly
raised viewpoint.

## Navigation

[Code](./2-on-screen) / [Prev](1-install.md) / [Next](3-spin.md)

---

This work is licensed under [CC BY 4.0](http://creativecommons.org/licenses/by/4.0)
