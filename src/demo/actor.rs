use std::f32::consts::PI;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    AppSystems,
    data::game_config::GameConfig,
    demo::ui::actions::SetActiveActionEvent,
    model::{actor::ActorId, actor_type::ActorTypeId, direction::Dir, game::Game},
    screens::Screen,
};

use super::{
    GameplayState,
    drag::{DragSource, StartDragEvent},
    tile::{HoveredActor, tile_coord_to_world_coord},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ActorEntities>();
    app.add_observer(on_actor_spawned);
    app.add_observer(on_actor_despawned);
    app.add_observer(on_spawn_actor);
    app.add_observer(on_actor_rotation_fixup);
    app.add_systems(
        Update,
        (actor_click, update_actions, rotate)
            .run_if(in_state(GameplayState::Placement).and(in_state(Screen::Gameplay)))
            .in_set(AppSystems::Update),
    );
}

pub const ACTOR_Z: f32 = 2.0;

#[derive(Resource, Default, Debug, Clone)]
pub struct ActorEntities(HashMap<ActorId, Entity>);

impl ActorEntities {
    pub fn get(&self, actor_id: &ActorId) -> Option<Entity> {
        self.0.get(actor_id).copied()
    }
}

pub fn on_actor_rotation_fixup(
    _: Trigger<ActorRotationFixupEvent>,
    mut q_actor: Query<(&ActorId, &mut Transform, &mut Sprite)>,
    game: Res<Game>,
) {
    for (actor, mut tr, mut sprite) in &mut q_actor {
        let actor = game.actor_view(actor).unwrap();
        match actor.actor.looks_to {
            Dir::Up => {
                tr.rotation = Quat::from_rotation_z(PI / 2.0);
                sprite.flip_x = false;
            }
            Dir::Down => {
                tr.rotation = Quat::from_rotation_z(-PI / 2.0);
                sprite.flip_x = false;
            }
            Dir::Left => {
                tr.rotation = Quat::from_rotation_z(0.);
                sprite.flip_x = true;
            }
            Dir::Right => {
                tr.rotation = Quat::from_rotation_z(0.);
                sprite.flip_x = false;
            }
        }
    }
}

#[derive(Event, Debug, Clone)]
pub struct ActorRotationFixupEvent;

fn update_actions(mut commands: Commands, hovered_actor: Res<HoveredActor>) {
    let (_actor_hover, actor_rotatable) = if let Some((_e, actor)) = &**hovered_actor {
        (true, actor.actor_type.rotatable)
    } else {
        (false, false)
    };
    commands.trigger(SetActiveActionEvent(
        "r_rotate".to_string(),
        actor_rotatable,
    ));
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpawnActorEvent {
    pub actor_type_id: ActorTypeId,
    pub coord: IVec2,
}

pub fn on_actor_spawned(
    trigger: Trigger<OnAdd, ActorId>,
    mut commands: Commands,
    config: Res<GameConfig>,
    q_actors: Query<&ActorId>,
    mut actor_entities: ResMut<ActorEntities>,
    game: Res<Game>,
) {
    let entity = trigger.target();
    let actor_id = *q_actors.get(entity).unwrap();
    actor_entities.0.insert(actor_id, entity);
    let actor_view = game.actor_view(&actor_id).unwrap();

    let translation = tile_coord_to_world_coord(actor_view.actor.coord, config.checker.tile_size);
    let rotation = match actor_view.actor.looks_to {
        Dir::Up => Quat::from_rotation_z(PI / 2.0),
        Dir::Down => Quat::from_rotation_z(-PI / 2.0),
        Dir::Left => Quat::from_rotation_z(0.),
        Dir::Right => Quat::from_rotation_z(0.),
    };
    let flip_x = matches!(actor_view.actor.looks_to, Dir::Left);

    commands.entity(entity).insert((
        StateScoped(Screen::Gameplay),
        Visibility::default(),
        Name::new(actor_view.actor_type.name.clone()),
        Transform::from_translation(translation.extend(ACTOR_Z)).with_rotation(rotation),
        Sprite {
            image: actor_view.actor_type.sprite_handle.clone().unwrap(),
            custom_size: Some(Vec2::splat(config.checker.tile_size)),
            flip_x,
            ..default()
        },
    ));
}

pub fn on_actor_despawned(
    trigger: Trigger<OnRemove, ActorId>,
    mut actor_entities: ResMut<ActorEntities>,
    q_actor: Query<&ActorId>,
) {
    let entity = trigger.target();
    let actor_id = q_actor.get(entity).unwrap();
    actor_entities.0.remove(actor_id);
}

pub fn on_spawn_actor(
    trigger: Trigger<SpawnActorEvent>,
    mut commands: Commands,
    mut game: ResMut<Game>,
) {
    let ev = trigger.event();
    if let Some(actor_id) = game.new_actor(&ev.actor_type_id, ev.coord) {
        commands.spawn(actor_id);
    } else {
        warn!("could not spawn actor");
    }
}

pub fn actor_click(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    hovered_actor: Res<HoveredActor>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some((entity, actor)) = &**hovered_actor {
            if actor.actor_type.dragable {
                commands.trigger(StartDragEvent {
                    source: DragSource::Board {
                        dragged_entity: *entity,
                        start_coord: actor.actor.coord,
                    },
                    actor_type_id: actor.actor.actor_type_id.clone(),
                });
            }
        }
    }
}

fn rotate(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    hovered_actor: Res<HoveredActor>,
    mut game: ResMut<Game>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        if let Some((_entity, actor)) = &**hovered_actor {
            if actor.actor_type.rotatable {
                game.rotate_actor(&actor.actor_id);
                commands.trigger(ActorRotationFixupEvent);
            }
        }
    }
}
