mod components;
pub(crate) mod utils;

pub(crate) use components::*;
mod resources;
pub use resources::*;
mod systems;
pub(crate) use systems::*;
mod events;
pub use events::*;
mod bounds;
mod window;
pub(crate) use bounds::*;
mod game_command_handler;
mod input;

use bevy::log;
use bevy::prelude::*;

use states::GameStatus;
// use iyes_loopless::prelude::*;

// Condition checking our timer
// fn tick_timer(mytimer: Res<TickEvent>) -> bool {
//     mytimer.0.just_finished()
// }

// /// Timers gotta be ticked
fn tick_mytimer(mut mytimer: ResMut<Tick>, time: Res<Time>) {
    mytimer.0.tick(time.delta());
    // println!("timer: {:?}", mytimer.0.elapsed_secs());
}

pub struct TetrisPlugin;

pub fn state_running(mut state: ResMut<NextState<GameStatus>>) {
    state.set(GameStatus::Running);
}

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(TweeningPlugin);
        // When the running states comes into the stack we load a board
        
        app.init_resource::<Tick>();
        // app.init_state::<GameStatus>();
        app.add_sub_state::<states::GameStatus>();

        // app.add_state_to_stage(CoreStage::PostUpdate, GameStatus::WaitingForMain);
        app.add_systems(Update, systems::events_to_state);
        app.add_systems(Update, game_command_handler::input);
        app.add_systems(
            OnEnter(GameStatus::Init),
            (create_board, state_running),
        );
        // #[cfg(feature = "debug")]
        // app.add_systems(OnEnter(GameStatus::Init), systems::systems::spawn_debug_block);

        app.add_systems(OnEnter(GameStatus::Paused), systems::show_popup);
        // app.add_systems(
        //     SystemSet::on_update(GameStatus::Paused)
        //         .with_system(systems::load_and_save::exclusive_system().at_end()),
        // );
        app.add_systems(OnExit(GameStatus::Paused), systems::hide_popup);
        app.add_systems(
            Update,
            (
                tick_mytimer, // TODO
                systems::gameover,
                systems::line, //TODO .exclusive_system().at_end(),
                systems::spawn_tetromino,
                systems::rotate,
                systems::gogo,
                systems::tock,
                // systems::animate,
                systems::score,
                input::input,
                systems::load_and_save_warning,
            ),
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
