use bevy::prelude::*;
use system::commons::{Character, Enemy, TurnOrder};
use system::states::{GameState, InGameState};
use crate::setup::{spawn_entities};

pub struct BattleTurnLogicPlugin;

impl Plugin for BattleTurnLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), setup_battle_order
            .after(spawn_entities));
        app.add_systems(Update, battle_order_system
            .run_if(in_state(GameState::InGame(InGameState::Battle))));
    }
}

fn setup_battle_order(
    mut turn_order: ResMut<TurnOrder>,
    characters: Query<(Entity, &Character)>,
    enemies: Query<(Entity, &Enemy)>
) {
    let mut participants: Vec<(Entity, f64)> = Vec::new();
    for (entity, character) in characters.iter() {
        participants.push((entity, character.current_stats.speed));
    }

    for (entity, enemy) in enemies.iter() {
        participants.push((entity, enemy.current_stats.speed));
    }

    participants.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    turn_order.order = participants.into_iter().map(|(entity, _)| entity).collect();
    info!("Turn logic start, entries: [{}]", turn_order.order.len());
}

fn battle_order_system(
    mut turn_order: ResMut<TurnOrder>,
    characters: Query<(&Character, &Name)>,
    enemies: Query<(&Enemy, &Name)>,
) {
    if turn_order.order.is_empty() || !turn_order.next {
        return;
    }

    turn_order.next = false;
    let current_entity = turn_order.order[turn_order.current_index];

    let (name, is_alive) = characters
        .get(current_entity)
        .map(|(stats, name)| (name.clone(), stats.current_stats.hp > 0.0))
        .or_else(|_| {
            enemies
                .get(current_entity)
                .map(|(stats, name)| (name.clone(), stats.current_stats.hp > 0.0))
        })
        .unwrap_or((Name::new(String::from("Unknown")), false));

    if !is_alive {
        let index = turn_order.current_index;
        turn_order.order.remove(index);
        if turn_order.order.is_empty() {
            return;
        }
    } else {
        info!("{} ist am Zug!", name);
    }

    turn_order.current_index = (turn_order.current_index + 1) % turn_order.order.len();
}
