use bevy::prelude::*;

use crate::components::Cleanup;

pub fn cleanup(mut commands: Commands, mut query: Query<(Entity, &mut Cleanup)>, time: Res<Time>) {
    for (entity, mut cleanup) in query.iter_mut() {
        match cleanup.as_mut() {
            Cleanup::OnTimeout { timer } => {
                if timer.tick(time.delta()).finished() {
                    let Some(ent_commands) = commands.get_entity(entity) else {
                        continue;
                    };
                    ent_commands.despawn_recursive();
                }
            }
            Cleanup::OnlyEntity => {
                let Some(mut ent_commands) = commands.get_entity(entity) else {
                    continue;
                };
                ent_commands.despawn();
            }
            Cleanup::OnlyDescendants => {
                let Some(mut ent_commands) = commands.get_entity(entity) else {
                    continue;
                };
                ent_commands.despawn_descendants();
            }
            Cleanup::Recursive => {
                let Some(ent_commands) = commands.get_entity(entity) else {
                    continue;
                };
                ent_commands.despawn_recursive();
            }
        }
    }
}
