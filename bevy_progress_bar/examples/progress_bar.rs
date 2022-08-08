use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchPlugin};
use bevy_progress_bar::{ProgressBarBundle, ProgressBarData, ProgressBarPlugin};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(NinePatchPlugin::<()>::default())
    .add_plugin(ProgressBarPlugin)
    .add_startup_system(setup_loading.system())
    .add_system(update_loading.system())
    .run();
}

fn setup_loading(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
  // We need a UI camera so the ninepatches show up
  commands.spawn_bundle(UiCameraBundle::default());

  let foreground_nine_patch = nine_patches.add(NinePatchBuilder::by_margins(2, 2, 2, 2));
  let background_nine_patch = nine_patches.add(NinePatchBuilder::by_margins(2, 2, 2, 2));
  let foreground_texture = asset_server.load("loader_fg.png");
  let background_texture = asset_server.load("loader_bg.png");

  // Spawn the progress bar entity
  commands.spawn_bundle(ProgressBarBundle {
    progress_bar_data: ProgressBarData {
      foreground_nine_patch,
      background_nine_patch,
      foreground_texture,
      background_texture,
      percent: 0.,
      ..Default::default()
    },
    style: Style {
      margin: Rect::all(Val::Auto),
      align_self: AlignSelf::Center,
      size: Size::new(Val::Percent(80.), Val::Percent(8.)),
      ..Default::default()
    },
    ..Default::default()
  });
}

fn update_loading(time: Res<Time>, mut query: Query<&mut ProgressBarData>) {
  // For this example we'll just have this progress bar
  // fill over a span of 10 seconds again and again.
  let progress = (time.seconds_since_startup() as f32 * 10.) % 100.;

  for mut progress_bar in query.iter_mut() {
    // Just update the percent field and the plugin will
    // do all the propagating and resizing.
    progress_bar.percent = progress;
  }
}
