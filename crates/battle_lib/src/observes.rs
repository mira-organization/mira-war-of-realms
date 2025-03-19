use bevy::prelude::*;
use system::battle_commons::{BattleSelectedStatus, Slot};

/// Handles mouse click events on interactive objects.
///
/// Logs the entity that was clicked.
///
/// # Parameters
/// - `event`: The event containing information about the clicked entity.
pub fn on_mouse_click(
    event: Trigger<Pointer<Click>>,
    mut selected: ResMut<BattleSelectedStatus>,
    parent: Query<&Parent>,
    slot: Query<&Slot>,
) {
    let target = event.target;
    let parent_entity = parent.get(target).map(|p| p.get()).ok();

    if let Some((_, selected_entity)) = selected.selected {
        if let Some(parent_entity) = parent_entity {
            if selected_entity != parent_entity {
                let index = slot.get(parent_entity).unwrap();
                selected.selected = Some((index.0, parent_entity));
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