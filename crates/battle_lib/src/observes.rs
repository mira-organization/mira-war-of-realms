use bevy::prelude::*;
use system::battle_commons::{BattleSelectedStatus};

/// Handles mouse click events on interactive objects.
///
/// Logs the entity that was clicked.
///
/// # Parameters
/// - `event`: The event containing information about the clicked entity.
pub fn on_mouse_click(
    event: Trigger<Pointer<Click>>,
    mut selected: ResMut<BattleSelectedStatus>,
    parent: Query<&Parent>
) {
    let target = event.target;
    let parent_entity = parent.get(target).map(|p| p.get()).ok();

    if let Some(selected_entity) = selected.selected {
        if let Some(parent_entity) = parent_entity {
            if selected_entity != parent_entity {
                selected.selected = Some(parent_entity);
            }
        }
    }
}

/// Handles mouse hover enter events on interactive objects.
///
/// Logs when the cursor enters an entity's interactive area.
///
/// # Parameters
/// - `event`: The event containing information about the entity being hovered over.
pub fn on_mouse_enter(_event: Trigger<Pointer<Over>>) {

}

/// Handles mouse hover leave events on interactive objects.
///
/// Logs when the cursor leaves an entity's interactive area.
///
/// # Parameters
/// - `event`: The event containing information about the entity being left.
pub fn on_mouse_leave(_event: Trigger<Pointer<Out>>) {

}