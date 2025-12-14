use bevy::prelude::*;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: bevy::winit::WinitAndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );
    
    log::info!("Starting Gachaverse");
    
    bevy::winit::run_with_app(app, || {
        Box::new(|_| build_app())
    });
}

fn build_app() -> App {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            ..default()
        }),
        ..default()
    }));
    
    app.insert_resource(ClearColor(Color::rgb(0.2, 0.5, 0.8)));
    app.add_systems(Startup, setup);
    
    app
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    log::info!("Setup started");
    
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("backgrounds/000.jpg"),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
    });
    
    log::info!("Setup done");
}
