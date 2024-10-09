use bevy::{prelude::*, window::WindowResolution};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x
        }
    }
}


fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

fn add_square(
    commands: &mut Commands, 
    materials: &mut ResMut<Assets<ColorMaterial>>, 
    size: f32, 
    color: Color, 
    position: Vec3
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(ColorMaterial::color(color)), // 设置正方形颜色
        sprite: Sprite::new(Vec2::new(size, size)), // 设置正方形大小
        transform: Transform::from_translation(position), // 设置正方形位置
        ..Default::default()
    });
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // 添加 2D 摄像机
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // 调用 add_square 方法绘制一个正方形
    add_square(&mut commands, &mut materials, 100.0, Color::rgb(0.2, 0.7, 0.9), Vec3::new(0.0, 0.0, 0.0));
    
    // 可以添加多个正方形
    add_square(&mut commands, &mut materials, 50.0, Color::rgb(0.9, 0.1, 0.1), Vec3::new(200.0, 100.0, 0.0));
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

pub struct DrawCubePlugin;

impl Plugin for DrawCubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn main() {
    App::new()
    // 设置窗口的大小   
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                resolution: WindowResolution::new(400.0, 700.0),
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .add_plugins(DrawCubePlugin)
        .run();
}