use bevy::{
    // dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};
use iyes_perf_ui::PerfUiPlugin;
use iyes_perf_ui::prelude::*;

// struct OverlayColor;
// impl OverlayColor {
//     const RED: Color = Color::srgb(1.0, 0.0, 0.0);
//     const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
// }

pub struct FpsCounterPlugin;
impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app
        // app.add_plugins(FpsOverlayPlugin {
        //     config: FpsOverlayConfig {
        //         text_config: TextFont {
        //             font_size: 42.0,
        //             ..default()
        //         },
        //         text_color: OverlayColor::RED,
        //         enabled: true,
        //     },
        // })
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        // .add_systems(Update, customize_config)
        ;
    }
}
fn setup(mut commands: Commands) {
    commands.spawn(PerfUiAllEntries::default());
    // commands.spawn((
    //     Text::new(concat!(
    //         "Press 1 to toggle the overlay color.\n",
    //         "Press 2 to decrease the overlay size.\n",
    //         "Press 3 to increase the overlay size.\n",
    //         "Press 4 to toggle the overlay visibility.",
    //     )),
    //     Node {
    //         position_type: PositionType::Absolute,
    //         bottom: Val::Px(12.0),
    //         left: Val::Px(12.0),
    //         ..default()
    //     },
    // ));
}
// fn customize_config(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
//     if input.just_pressed(KeyCode::Digit1) {
//         // Changing resource will affect overlay
//         if overlay.text_color == OverlayColor::GREEN {
//             overlay.text_color = OverlayColor::RED;
//         } else {
//             overlay.text_color = OverlayColor::GREEN;
//         }
//     }
//     if input.just_pressed(KeyCode::Digit2) {
//         overlay.text_config.font_size -= 2.0;
//     }
//     if input.just_pressed(KeyCode::Digit3) {
//         overlay.text_config.font_size += 2.0;
//     }
//     if input.just_pressed(KeyCode::Digit4) {
//         overlay.enabled = !overlay.enabled;
//     }
// }
