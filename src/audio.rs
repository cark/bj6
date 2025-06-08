use bevy::{audio::Volume, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.register_type::<SoundEffect>();
    app.insert_resource(MusicVolume::new(Volume::Linear(0.4)));
    app.insert_resource(SfxVolume::new(Volume::Linear(1.0)));

    app.add_systems(
        Update,
        (
            apply_global_volume.run_if(resource_changed::<GlobalVolume>),
            apply_music_volume.run_if(resource_changed::<MusicVolume>),
        ),
    );
    app.add_observer(on_music_added);
    app.add_observer(on_sfx_added);
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}

fn apply_music_volume(
    global_volume: Res<GlobalVolume>,
    music_volume: Res<MusicVolume>,
    mut audio_query: Query<&mut AudioSink, With<Music>>,
) {
    for mut sink in &mut audio_query {
        sink.set_volume(global_volume.volume * music_volume.volume);
    }
}

fn on_music_added(
    trigger: Trigger<OnAdd, Music>,
    mut q_playback: Query<(&mut PlaybackSettings,), With<Music>>, //&mut AudioSink
    // global_volume: Res<GlobalVolume>,
    music_volume: Res<MusicVolume>,
) {
    if let Ok((mut playback,)) = q_playback.get_mut(trigger.target()) {
        // if let Ok((mut playback, mut sink)) = q_playback.get_mut(trigger.target()) {
        playback.volume = music_volume.volume;

        // sink.set_volume(global_volume.volume * playback.volume * music_volume.volume);
        // sink.set_volume(global_volume.volume * playback.volume * music_volume.volume);
    }
}

fn on_sfx_added(
    trigger: Trigger<OnAdd, SoundEffect>,
    mut q_playback: Query<&mut PlaybackSettings, With<SoundEffect>>,
    sfx_volume: Res<SfxVolume>,
) {
    if let Ok(mut playback) = q_playback.get_mut(trigger.target()) {
        playback.volume = sfx_volume.volume;
    }
}

#[derive(Resource, Debug, Clone)]
pub struct MusicVolume {
    pub volume: Volume,
}

impl From<Volume> for MusicVolume {
    fn from(volume: Volume) -> Self {
        Self { volume }
    }
}

impl MusicVolume {
    pub fn new(volume: Volume) -> Self {
        Self { volume }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct SfxVolume {
    pub volume: Volume,
}

impl From<Volume> for SfxVolume {
    fn from(volume: Volume) -> Self {
        Self { volume }
    }
}

impl SfxVolume {
    pub fn new(volume: Volume) -> Self {
        Self { volume }
    }
}
