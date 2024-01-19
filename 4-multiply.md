# Multiply

> Create multiple cubes from the same assets

Create a resource for holding the asset handles shared between boxes:

```rust
#[derive(Resource, Clone, Default)]
struct BoxAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}
```

Add a behaviour that will initialise the box resources:

```rust
fn init_box_assets(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut box_assets: ResMut<BoxAssets>,
) {
    box_assets.mesh = meshes.add(Mesh::from(Cube::default()));
    box_assets.material = materials.add(Color::BLUE.with_l(0.2).into());
}
```

Add the resource and behaviour to the application startup:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<BoxAssets>()
        .add_systems(Startup, init_box_assets)
    // remainder unchanged
}
```

Define a utility for spawning boxes:

```rust
fn spawn_box_at(commands: &mut Commands, box_assets: &Res<BoxAssets>, position: Vec3) {
    commands.spawn(PbrBundle {
        mesh: box_assets.mesh.clone(),
        material: box_assets.material.clone(),
        transform: Transform::from_translation(position + Vec3::Y * 0.5),
        ..default()
    });
}
```

Update `spawn_lit_box` to use the utility:

```rust
fn spawn_lit_box(mut commands: Commands, box_resources: Res<BoxResources>) {
    for i in 0..5 {
        spawn_box_at(&mut commands, &box_assets, Vec3::X * (i as f32) * 1.5);
    }
    // remainder unchanged
}
```

And change application setup in `main` to ensure `init_box_assets` is called before `spawn_lit_box`:

```rust
    // replace
        .add_systems(Startup, spawn_lit_box)
    // with
        .add_systems(Startup, spawn_lit_box.after(init_box_assets))
```

## Navigation

[Code](./4-multiply) / [Prev](3-spin.md) / [Next](5-pop.md)

## Detailed Walk-Through


---

This work is licensed under [CC BY 4.0](http://creativecommons.org/licenses/by/4.0)
