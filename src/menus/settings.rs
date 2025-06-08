//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};

use crate::{
    audio::{MusicVolume, SfxVolume},
    menus::Menu,
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );

    app.register_type::<GlobalVolumeLabel>();
    app.add_systems(
        Update,
        (
            update_global_volume_label,
            update_music_volume_label,
            update_sfx_volume_label,
        )
            .run_if(in_state(Menu::Settings)),
    );
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widget::center_ui_root("Settings Menu"),
        GlobalZIndex(10),
        StateScoped(Menu::Settings),
        children![
            widget::header("Settings"),
            settings_grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                widget::label("Master Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            global_volume_widget(),
            (
                widget::label("Music Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            music_volume_widget(),
            (
                widget::label("Sfx Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            sfx_volume_widget(),
        ],
    )
}

fn global_volume_widget() -> impl Bundle {
    (
        Name::new("Global Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_global_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), GlobalVolumeLabel)],
            ),
            widget::button_small("+", raise_global_volume),
        ],
    )
}

fn music_volume_widget() -> impl Bundle {
    (
        Name::new("Music Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_music_volume),
            (
                Name::new("Current Music Volume"),
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), MusicVolumeLabel)],
            ),
            widget::button_small("+", raise_music_volume),
        ],
    )
}

fn sfx_volume_widget() -> impl Bundle {
    (
        Name::new("Music Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_sfx_volume),
            (
                Name::new("Current Music Volume"),
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), SfxVolumeLabel)],
            ),
            widget::button_small("+", raise_sfx_volume),
        ],
    )
}

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

fn lower_global_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn raise_global_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn lower_music_volume(_: Trigger<Pointer<Click>>, mut volume: ResMut<MusicVolume>) {
    let linear = (volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    volume.volume = Volume::Linear(linear);
}

fn raise_music_volume(_: Trigger<Pointer<Click>>, mut volume: ResMut<MusicVolume>) {
    let linear = (volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
    volume.volume = Volume::Linear(linear);
}

fn lower_sfx_volume(_: Trigger<Pointer<Click>>, mut volume: ResMut<SfxVolume>) {
    let linear = (volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    volume.volume = Volume::Linear(linear);
}

fn raise_sfx_volume(_: Trigger<Pointer<Click>>, mut volume: ResMut<SfxVolume>) {
    let linear = (volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
    volume.volume = Volume::Linear(linear);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MusicVolumeLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SfxVolumeLabel;

fn update_global_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn update_music_volume_label(
    music_volume: Res<MusicVolume>,
    mut label: Single<&mut Text, With<MusicVolumeLabel>>,
) {
    let percent = 100.0 * music_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn update_sfx_volume_label(
    sfx_volume: Res<SfxVolume>,
    mut label: Single<&mut Text, With<SfxVolumeLabel>>,
) {
    let percent = 100.0 * sfx_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn go_back_on_click(
    _: Trigger<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}
