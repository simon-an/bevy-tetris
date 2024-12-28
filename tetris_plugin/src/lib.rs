mod components;
pub(crate) mod utils;

use bevy_pkv::PkvStore;
pub(crate) use components::*;
mod resources;
pub use resources::*;
mod systems;
use states::GameLogicState;
use states::InGame;
pub(crate) use systems::*;
mod events;
pub use events::*;
mod bounds;
mod window;
pub(crate) use bounds::*;
mod game_command_handler;
mod input;
mod queries;

use bevy::log;
use bevy::prelude::*;

use states::GameStatus;

pub struct TetrisPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum TetrisSystemSet {
    BeforeRound,
    Round,
    AfterRound,
}

pub fn state_running(mut state: ResMut<NextState<GameStatus>>) {
    state.set(GameStatus::Running);
}

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(TweeningPlugin);
        // When the running states comes into the stack we load a board

        app.configure_sets(
            FixedUpdate,
            // chain() will ensure sets run in the order they are listed
            (
                TetrisSystemSet::BeforeRound,
                TetrisSystemSet::Round,
                TetrisSystemSet::AfterRound,
            )
                .chain(),
        );
        app.configure_sets(
            Update,
            // chain() will ensure sets run in the order they are listed
            (
                TetrisSystemSet::BeforeRound,
                TetrisSystemSet::Round,
                TetrisSystemSet::AfterRound,
            )
                .chain(),
        );

        app.insert_resource(Time::<Fixed>::from_seconds(0.5));
        app.insert_resource(TickCounter(0));
        app.insert_resource(NextShape(rand::random()));
        app.insert_resource(PkvStore::new("bevy-tetris", "bevy-tetris"));
        // app.init_state::<GameStatus>();
        app.add_sub_state::<states::GameStatus>();
        app.add_sub_state::<states::GameLogicState>();
        app.add_computed_state::<states::InGame>();

        app.add_systems(OnEnter(GameStatus::Init), (create_board, state_running));
        // #[cfg(feature = "debug")]
        // app.add_systems(OnEnter(GameStatus::Init), systems::systems::spawn_debug_block);

        app.add_systems(OnEnter(InGame), (systems::sidebar, systems::sidebar_left));
        app.add_systems(OnEnter(GameStatus::Paused), systems::show_popup);
        app.add_systems(OnExit(GameStatus::Paused), systems::hide_popup);
        app.add_systems(
            Update,
            (
                // input during spawning state can cause the map to be in an invalid state because spawning and move systems can run in parallel and mutate the map
                input::input.run_if(in_state(GameLogicState::Ticking)),
                systems::events_to_state,
                game_command_handler::input,
                systems::update_block_sprites_colors.run_if(in_state(GameLogicState::Ticking)),
                systems::update_block_sprites_translation.run_if(in_state(GameLogicState::Ticking)),
                systems::gameover,
                systems::load_and_save_warning.run_if(not(in_state(GameStatus::Paused))),
                load_and_save.run_if(in_state(GameStatus::Paused)),
                systems::animate,
                // Before ticking, rotate and move blocks from user input
                // Play error sound must be before rotate and move because it will use the wrong coordinates to predict collision
                (
                    systems::play_error_sound,
                    systems::rotate,
                    systems::move_current,
                )
                    .chain()
                    .in_set(TetrisSystemSet::BeforeRound)
                    .run_if(in_state(GameLogicState::Ticking)),
            ),
        );
        app.add_systems(
            FixedUpdate,
            ((
                (systems::spawn_tetromino, systems::spawn_preview)
                    .chain()
                    .run_if(in_state(GameLogicState::Spawning)),
                // detect collision before moving blocks automatically, so user can move blocks in the last line
                (systems::detect_collision)
                    .in_set(TetrisSystemSet::BeforeRound)
                    .run_if(in_state(GameLogicState::Ticking)),
                (systems::tick_counter, systems::tick)
                    .in_set(TetrisSystemSet::Round)
                    .run_if(in_state(GameLogicState::Ticking)),
                //  Remove line and score
                (
                    systems::convert_to_block,
                    systems::delete_line,
                    systems::score,
                    systems::update_score,
                )
                    .chain()
                    .run_if(in_state(GameLogicState::Cleaning)),
            )
                .run_if(in_state(GameStatus::Running)),),
        );
        app.add_systems(OnEnter(GameStatus::Gameover), systems::cleanup_board);
        app.add_event::<SpawnEvent>()
            .add_event::<GameCommand>()
            .add_event::<GameOverEvent>()
            .add_event::<RotateEvent>()
            .add_event::<ScoreEvent>()
            .add_event::<MoveEvent>();

        #[cfg(feature = "debug")]
        {
            // registering custom component to be able to edit it in inspector
            app.register_type::<Tetromino>();
            app.register_type::<CurrentTetromino>();
            app.register_type::<Coordinates>();
        }
        log::info!("Loaded Tetris Plugin");
    }
}
