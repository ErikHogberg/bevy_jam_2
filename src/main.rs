use bevy::{prelude::*, render::mesh::MeshPlugin};
use heron::{
    // rapier_plugin::rapier3d::parry::query::{Ray, RayCast},
    AxisAngle,
    CollisionLayers,
    CollisionShape,
    PhysicsLayer,
    PhysicsPlugin,
    RigidBody,
    Velocity,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MeshPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_system(input_system.before(gravity_system))
        .add_system(spawn_system.after(spawn_timer_system))
        .add_startup_system(setup)
        .run();
}

#[derive(PhysicsLayer)]
enum GameLayer {
    Enemies,
    World1,
    World2,
    Player,
}

#[derive(Component)]
struct Wheel;
#[derive(Component)]
struct Debree;
#[derive(Component)]
struct Destructable;
#[derive(Component)]
struct Spawner {
    queue: i32,
}

#[derive(Component)]
struct Timer {
    time: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            // mesh: meshes.add(create_triangle()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: 2.5,
                y: 0.1,
                z: 2.5,
            },
            border_radius: Some(1.0),
        })
        .insert(
            CollisionLayers::none()
            .with_group(GameLayer::World1)
            .with_masks(&[GameLayer::Player, GameLayer::Enemies]),
        )
        .insert(RigidBody::Static)
        ;

    // plane
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
    //         // mesh: meshes.add(create_triangle()),
    //         material: materials.add(Color::rgb(0.6, 0.2, 0.3).into()),
    //         transform: Transform::from_xyz(0.0, 1.5, 0.0),
    //         ..default()
    //     })
    //     .insert(CollisionShape::Cuboid {
    //         half_extends: Vec3 {
    //             x: 2.5,
    //             y: 0.1,
    //             z: 2.5,
    //         },
    //         border_radius: Some(1.0),
    //     })
    //     .insert(RigidBody::Static)
    //     .insert(
    //         CollisionLayers::none()
    //             .with_group(GameLayer::World2)
    //     );
  

    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Wheel)
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 0.1 })
        .insert(
            CollisionLayers::none()
            .with_group(GameLayer::Player)
            .with_masks(&[GameLayer::World1, GameLayer::Enemies]),
        )
        .insert(Velocity::from_angular(AxisAngle::new(Vec3::X, 1.0)))
        ;

    // spawner
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 2.5, 0.0),
            ..default()
        })
        .insert(Spawner { queue: 0 })
        .insert(Timer { time: 1.0 });

    // wheel
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //         transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //         ..default()
    //     })
    //     .insert(Wheel);

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// fn create_triangle() -> Mesh {
//     let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
//     mesh.insert_attribute(
//         Mesh::ATTRIBUTE_POSITION,
//         vec![[0.0, 1.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
//     );
//     mesh.insert_attribute(
//         Mesh::ATTRIBUTE_NORMAL,
//         vec![[0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
//     );
//     mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
//     mesh
// }

fn input_axis_val(keyboard_input: &Res<Input<KeyCode>>, pos_key: KeyCode, neg_key: KeyCode) -> f32 {
    if keyboard_input.pressed(pos_key) {
        1.0
    } else if keyboard_input.pressed(neg_key) {
        -1.0
    } else {
        0.0
    }
}

fn gravity_system(mut query: Query<&mut Velocity>) {
    for mut tf in query.iter_mut() {
        println!("applied gravity");
        tf.linear = Vec3 {
            y: -9.82,
            ..tf.linear
        };
    }
}

fn input_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Wheel, &mut Velocity)>) {
    // let amount = input_axis_val(&keyboard_input, KeyCode::Q, KeyCode::Q);
    let dir = Vec3 {
        x: input_axis_val(&keyboard_input, KeyCode::A, KeyCode::D),
        y: input_axis_val(&keyboard_input, KeyCode::Space, KeyCode::LControl),
        z: input_axis_val(&keyboard_input, KeyCode::W, KeyCode::S),
    };

    for (_, mut tf) in query.iter_mut() {
        tf.linear = dir * 2.4;
    }
}

fn spawn_timer_system(time: Res<Time>, mut query: Query<(&mut Timer, &mut Spawner)>) {
    for (mut timer, mut spawner) in query.iter_mut() {
        timer.time -= time.delta_seconds();
        if timer.time < 0.0 {
            timer.time = 3.0;

            spawner.queue += 1;
        }
    }
}

fn spawn_mover_system(time: Res<Time>, mut query: Query<(&mut Transform, With<Spawner>)>) {
    for (mut tf, _) in query.iter_mut() {
        // TODO: move spawner in some pattern
    }
}

fn spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Transform, &mut Spawner)>,
) {
    for (tf, mut spawner) in query.iter_mut() {
        if spawner.queue > 0 {
            spawner.queue -= 1;

            println!("spawned new cube");

            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(fastrand::f32(), fastrand::f32(), fastrand::f32()).into()),
                    transform: tf.clone(),
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(CollisionShape::Cuboid { half_extends: Vec3::ONE*0.2, border_radius: Some(0.1) })
                // .insert(Velocity::from_angular(AxisAngle::new(Vec3::X, 1.0)))
                .insert(
                    CollisionLayers::none()
                        .with_group(GameLayer::Enemies)
                        .with_masks(&[GameLayer::World2, GameLayer::Enemies]),
                )
                .insert(Velocity::from_linear(Vec3::Y*-1.0))
                ;
        }
    }
}
