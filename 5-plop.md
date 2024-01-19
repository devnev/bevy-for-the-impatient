# Plop

> Spawn cubes on click

Add the picking package to your project:

```sh
cargo add bevy_mod_picking
```

Add picking to your application in `main`:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>(),
        )
    // remainder unchanged
}
```

Add a behaviour for spawning a pickable floor:

```rust
fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Plane::from_size(100.0))),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
        PickableBundle::default(),
        On::<Pointer<Click>>::run(
            move |event: Res<ListenerInput<Pointer<Click>>>,
                  mut commands: Commands,
                  box_assets: Res<BoxAssets>| {
                spawn_box_at(&mut commands, &box_assets, event.hit.position.unwrap())
            },
        ),
    ));
}
```

Add the behaviour to the application setup in `main`:

```rust
        .add_systems(Startup, spawn_floor)
```

## Navigation

[Code](./5-plop) / [Prev](4-multiply.md)

## Detailed Walk-Through

---

This work is licensed under [CC BY 4.0](http://creativecommons.org/licenses/by/4.0)
