
use bevy::{
    prelude::*,
    render::{
        mesh::{
            // Indices, 
            MeshPlugin},
        // render_resource::PrimitiveTopology,
    },
};
use heron::{AxisAngle, CollisionShape, PhysicsPlugin, RigidBody, Velocity, rapier_plugin::rapier3d::parry::query::{Ray, RayCast}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(MeshPlugin)
        .add_system(input_offset_system)
        .add_system(gravity)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Wheel;

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
        .insert(RigidBody::Static);

    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 0.1 })
        .insert(Velocity::from_angular(AxisAngle::new(Vec3::X, 1.0)));

    // wheel
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Wheel)
        ;


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


fn gravity(
    mut query: Query<&mut Velocity>,
) {
    for mut tf in query.iter_mut() {
        tf.linear = Vec3 {y: -9.82, ..tf.linear};
    }
}

fn input_offset_system(
    // time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity>,
) {
    // let amount = input_axis_val(&keyboard_input, KeyCode::Q, KeyCode::Q);
    let dir = Vec3 {
        x: input_axis_val(&keyboard_input, KeyCode::A, KeyCode::D),
        y: input_axis_val(&keyboard_input, KeyCode::Space, KeyCode::LControl),
        z: input_axis_val(&keyboard_input, KeyCode::W, KeyCode::S),
    };

    for mut tf in query.iter_mut() {
        tf.linear = dir * 2.4;
    }
}

fn wheel_system(
    // time: Res<Time>,
    query: Query<(&Wheel, &Transform)>,
) {
    for (wheel, tf) in query.iter(){
        // let ray = Ray::new(, dir);
        // RayCast::cast_local_ray(&self, ray, max_toi, solid)
    }
}