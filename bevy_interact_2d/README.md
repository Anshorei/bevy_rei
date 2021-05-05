# Bevy Interact 2D

**Work in progress**

Plugin library for the Bevy game engine to easily add mouse interactions to 2d games.

Can help you with:
- Hovering
- Clicking
- Dragging & Droppings

## Using Interact2D

Add the interaction plugin, or use the `InteractionDebugPlugin` instead when debugging.
```
App::build()
  .add_plugin(InteractionPlugin)
```

Spawn a camera with an interaction source with a number of interaction groups.
```
commands
  .spawn_bundle(OrthographicCameraBundle::new_2d())
  .insert(InteractionSource {
    groups: vec![Group(0), Group(1)],
    ..Default::default()
  })
```

Spawn an interactable entity
```
commands
  .spawn()
  .insert(Interactable {
    groups: vec![Group(0)],
    bounding_box: (Vec2::new(0., 0.), Vec2::new(10., 10.)),
    ..Default::default()
  })
```

Now you can create a system that uses the interaction state
```
fn interaction_system(
  mut commands: Commands,
  mouse_button_input: Res<Input<MouseButton>>,
  interaction_state: Res<InteractionState>,
) {
  if !mouse_button_input.just_released(MouseButton::Left) {
    return;
  }

  for (entity, coords) in interaction_state.get_group(Group(0)).iter() {
    // Do something
  }
}
```
