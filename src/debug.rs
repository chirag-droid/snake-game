use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use iyes_perf_ui::{diagnostics::{PerfUiEntryFPS, PerfUiEntryFPSWorst}, PerfUiPlugin, PerfUiRoot};

use crate::GameState;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                PerfUiPlugin
            ));
            app.add_systems(OnEnter(GameState::Playing), add_perf_ui);
            app.add_systems(OnExit(GameState::Playing), remove_perf_ui);
        }
    }
}

fn add_perf_ui(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot {
            display_labels: true,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));
}

fn remove_perf_ui(mut commands: Commands, query: Query<Entity, With<PerfUiRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
