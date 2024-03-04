use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#addendum-canvas".into()),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_systems(Startup, setup_graphics)
    .add_systems(Startup, setup_physics)
    .add_systems(Update, print_char_event_system);

    // ...
    app.run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(ExternalImpulse {
            impulse: Vec3::ZERO,
            torque_impulse: Vec3::ZERO,
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn print_char_event_system(
    mut char_input_events: EventReader<ReceivedCharacter>,
    mut ext_impulses: Query<&mut ExternalImpulse>,
) {
    for event in char_input_events.read() {
        info!("{:?}: '{}'", event, event.char);
        for mut ext_impulse in ext_impulses.iter_mut() {
            ext_impulse.impulse = Vec3::new(0.0, 1.0, 0.0);
        }
    }
}
