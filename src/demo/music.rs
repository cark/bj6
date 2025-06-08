use bevy::prelude::*;

use crate::{audio::music, demo::level::LevelAssets, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // app.insert_resource(PreferedSong(Song::Song1));
    app.add_systems(OnEnter(Screen::Gameplay), init_songs);
    app.add_systems(OnExit(Screen::Gameplay), remove_songs);
    // app.add_systems(
    //     Update,
    //     update_song_volumes.run_if(in_state(Screen::Gameplay)),
    // );

    // app.add_observer(on_to_song);
}

// #[derive(Event, Debug, Clone, Copy)]
// pub struct ToSongEvent(pub Song);

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Song {
//     Song1,
//     Song2,
// }

// #[derive(Resource, Debug, Clone)]
// struct PreferedSong(Song);

#[derive(Component, Clone, Copy, Debug)]
struct Song1Marker;
// #[derive(Component, Clone, Copy, Debug)]
// struct Song2Marker;

fn init_songs(mut commands: Commands, assets: Res<LevelAssets>) {
    commands.spawn((Song1Marker, music(assets.song1.clone())));
    // commands.spawn((Song1Marker, music(assets.song2.clone())));
}

fn remove_songs(mut commands: Commands, query: Query<Entity, With<Song1Marker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
// fn remove_songs(
//     mut commands: Commands,
//     query: Query<Entity, Or<(With<Song1Marker>, With<Song2Marker>)>>,
// ) {
//     for entity in query.iter() {
//         commands.entity(entity).despawn();
//     }
// }

// fn on_to_song(trigger: Trigger<ToSongEvent>, mut prefered_song: ResMut<PreferedSong>) {
//     prefered_song.0 = trigger.event().0;
// }

// fn update_song_volumes(
//     q_sink1: Single<&mut AudioSink, (With<Song1Marker>, Without<Song2Marker>)>,
//     q_sink2: Single<&mut AudioSink, (With<Song2Marker>, Without<Song1Marker>)>,
//     prefered_song: Res<PreferedSong>,
//     global_volume: Res<GlobalVolume>,
//     music_volume: Res<MusicVolume>,
// ) {
//     dbg!(prefered_song.0);
//     let mut sink1 = q_sink1.into_inner();
//     let mut sink2 = q_sink2.into_inner();

//     if prefered_song.0 == Song::Song1 {
//         sink1.set_volume(global_volume.volume * music_volume.volume);
//     } else {
//         sink1.set_volume(Volume::Linear(0.0));
//     }
//     if prefered_song.0 == Song::Song2 {
//         sink2.set_volume(global_volume.volume * music_volume.volume);
//     } else {
//         sink2.set_volume(Volume::Linear(0.0));
//     }
// }

// fn on_to_song(
//     trigger: Trigger<ToSongEvent>,
//     mut q_sink1: Query<&mut AudioSink, (With<Song1Marker>, Without<Song2Marker>)>,
//     mut q_sink2: Query<&mut AudioSink, (With<Song2Marker>, Without<Song1Marker>)>,
//     global_volume: Res<GlobalVolume>,
//     music_volume: Res<MusicVolume>,
// ) {
//     let ev = trigger.event();
//     if let Ok(mut sink1) = q_sink1.single_mut() {
//         if ev.0 == Song::Song1 {
//             sink1.set_volume(global_volume.volume * music_volume.volume);
//         } else {
//             sink1.set_volume(Volume::Linear(0.0));
//         }
//     }
//     if let Ok(mut sink2) = q_sink2.single_mut() {
//         if ev.0 == Song::Song2 {
//             sink2.set_volume(global_volume.volume * music_volume.volume);
//         } else {
//             sink2.set_volume(Volume::Linear(0.0));
//         }
//     }
// }
