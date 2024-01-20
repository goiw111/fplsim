use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod qstate;
use qstate::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup,(setup_camera,setup_physics))
        .add_systems(Update,(forces,control))
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(30.0, 0.0, 7.0).looking_at(13.0 * Vec3::Z, Vec3::Z),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands,mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vect::Z * -9.81;
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(50.0, 50.0, 1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(PlatformBundle::new(0.5,0.8,20.0));
}

fn forces(mut commands: Commands, query: Query<(Entity, &Qstate)>) {
    let (entity, state) = query.single();
    commands.entity(entity).insert(state.get_total_forces());
}

fn control(mut query: Query<&mut Qstate>, keys: Res<Input<KeyCode>>) {
    let mut state = query.single_mut();
    if keys.pressed(KeyCode::W) {
        state.set(BM::M1, 2.0);
        state.set(BM::M2, 2.001);
        state.set(BM::M3, 2.0);
        state.set(BM::M4, 2.0);
    } else {
        state.set(BM::M1, 0.0);
        state.set(BM::M2, 0.0);
        state.set(BM::M3, 0.0);
        state.set(BM::M4, 0.0);

    }
}
