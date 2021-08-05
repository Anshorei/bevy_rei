use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData};

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system(create_progress_bars.system())
      .add_system(update_progress_bars.system());
  }
}

#[derive(Debug, Clone)]
pub struct ProgressBarData {
  pub nine_patch:         Handle<NinePatchBuilder<()>>,
  pub foreground_texture: Handle<Texture>,
  pub background_texture: Handle<Texture>,
  pub percent:            f32,
  // No need to touch
  pub percent_mutex:      Arc<Mutex<f32>>,
}

impl Default for ProgressBarData {
  fn default() -> Self {
    Self {
      nine_patch:         Default::default(),
      foreground_texture: Default::default(),
      background_texture: Default::default(),
      percent:            0.,
      percent_mutex:      Arc::new(Mutex::new(0.)),
    }
  }
}

struct ProgressBarForeground {
  pub percent_mutex: Arc<Mutex<f32>>,
}

#[derive(Bundle, Default)]
pub struct ProgressBarBundle {
  pub progress_bar_data: ProgressBarData,
  pub style:             Style,
  pub node:              Node,
  pub transform:         Transform,
  pub global_transform:  GlobalTransform,
}

fn create_ninepatch_bundle(
  nine_patch_handle: Handle<NinePatchBuilder<()>>,
  texture_handle: Handle<Texture>,
  percent: Option<f32>,
) -> NinePatchBundle<()> {
  NinePatchBundle {
    style: Style {
      margin: Rect::all(Val::Px(0.)),
      position_type: PositionType::Absolute,
      size: Size::new(Val::Percent(percent.unwrap_or(100.)), Val::Percent(100.)),
      ..Default::default()
    },
    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    nine_patch_data: NinePatchData {
      nine_patch: nine_patch_handle,
      texture: texture_handle,
      ..Default::default()
    },
    ..Default::default()
  }
}

fn create_progress_bars(
  mut commands: Commands,
  mut query: Query<(Entity, &ProgressBarData), Added<ProgressBarData>>,
) {
  for (parent, progress_bar_data) in query.iter_mut() {
    let background = commands
      .spawn_bundle(create_ninepatch_bundle(
        progress_bar_data.nine_patch.clone(),
        progress_bar_data.background_texture.clone(),
        None,
      ))
      .id();
    let foreground = commands
      .spawn_bundle(create_ninepatch_bundle(
        progress_bar_data.nine_patch.clone(),
        progress_bar_data.foreground_texture.clone(),
        Some(progress_bar_data.percent),
      ))
      .insert(ProgressBarForeground {
        percent_mutex: progress_bar_data.percent_mutex.clone(),
      })
      .id();
    commands
      .entity(parent)
      .push_children(&[background, foreground]);
  }
}

fn update_progress_bars(
  mut parent_query: Query<&ProgressBarData>,
  mut child_query: Query<(&ProgressBarForeground, &mut Style)>,
) {
  for progress_bar_data in parent_query.iter_mut() {
    let mut percent = progress_bar_data.percent_mutex.lock().unwrap();
    *percent = progress_bar_data.percent;
  }

  for (progress_bar_data, mut style) in child_query.iter_mut() {
    let percent = *progress_bar_data.percent_mutex.lock().unwrap();
    style.size.width = Val::Percent(percent);
  }
}
