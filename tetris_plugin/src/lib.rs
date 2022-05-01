mod components;
pub(crate) mod utils;

use bevy_tweening::TweeningPlugin;
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

use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;
// use iyes_loopless::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

/// Condition checking our timer
// fn tick_timer(mytimer: Res<TickEvent>) -> bool {
//     mytimer.0.just_finished()
// }

/// Timers gotta be ticked
fn tick_mytimer(mut mytimer: ResMut<TickEvent>, time: Res<Time>) {
    mytimer.0.tick(time.delta());
    // println!("timer: {:?}", mytimer.0.elapsed_secs());
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameStatus {
    WaitingForMain,
    Init,
    Running,
    Paused,
    Gameover,
    Loading,
}

pub struct TetrisPlugin<T> {
    pub plugin_init: T,
}

pub fn state_running(mut state: ResMut<State<GameStatus>>) {
    state.set(GameStatus::Running).unwrap();
}
pub fn state_init(mut state: ResMut<State<GameStatus>>) {
    state.set(GameStatus::Init).unwrap();
}

impl<T> Plugin for TetrisPlugin<T>
where
    T: StateData,
{
    fn build(&self, app: &mut App) {
        app.add_plugin(TweeningPlugin);
        // When the running states comes into the stack we load a board
        app.insert_resource(TickEvent(Timer::from_seconds(1.0, true)));
        app.add_state(GameStatus::WaitingForMain);
        // app.add_state_to_stage(CoreStage::PostUpdate, GameStatus::WaitingForMain);
        app.add_system(systems::events_to_state);
        app.add_system(game_command_handler::input);
        app.add_system_set(
            SystemSet::on_enter(self.plugin_init.clone())
                .label("map_app_state_to_plugin_state")
                .with_system(state_init),
        );
        app.add_system_set(
            SystemSet::on_enter(GameStatus::Init)
                .label("init_plugin")
                .with_system(create_board)
                .with_system(systems::spawn_debug_block)
                .with_system(state_running),
        );
        app.add_system_set(
            SystemSet::on_enter(GameStatus::Paused)
                .label("on_enter_pause")
                .with_system(systems::show_popup),
        );
        app.add_system_set(
            SystemSet::on_update(GameStatus::Paused)
                .label("on_update_pause")
                .with_system(systems::load_and_save::<T>.exclusive_system().at_end()),
        );
        app.add_system_set(
            SystemSet::on_pause(GameStatus::Paused)
                .label("on_pause_paused")
                .with_system(systems::hide_popup),
        );
        app.add_system_set(
            SystemSet::on_exit(GameStatus::Paused)
                .label("on_exit_paused")
                .with_system(systems::hide_popup),
        );
        app.add_system_set(SystemSet::on_enter(GameStatus::Running).label("on_enter_running"));
        app.add_system_set(
            SystemSet::on_update(GameStatus::Running)
                .label("running")
                .with_system(tick_mytimer)
                .with_system(systems::gameover)
                .with_system(systems::line.exclusive_system().at_end())
                .with_system(systems::spawn_tetromino)
                .with_system(systems::rotate)
                .with_system(systems::gogo)
                .with_system(systems::tock)
                .with_system(systems::animate)
                .with_system(systems::score)
                .with_system(input::input)
                .with_system(systems::load_and_save_warning),
        );
        app.add_system_set(
            SystemSet::on_enter(GameStatus::Gameover)
                .label("on_enter_gameover")
                .with_system(systems::cleanup_board),
        );
        app.add_event::<SpawnEvent>()
            .add_event::<GameCommand>()
            .add_event::<GameOverEvent>()
            .add_event::<RotateEvent>()
            .add_event::<ScoreEvent>()
            .add_event::<MoveEvent>();

        #[cfg(feature = "debug")]
        {
            // registering custom component to be able to edit it in inspector
            app.register_inspectable::<Tetromino>();
            app.register_inspectable::<CurrentTetromino>();
            app.register_inspectable::<Coordinates>();
        }
        log::info!("Loaded Tetris Plugin");
    }
}

// impl<T: StateData> Plugin for TetrisPlugin<T> {
//     fn build(&self, app: &mut App) {
//         app.add_plugin(TweeningPlugin);
//         // When the running states comes into the stack we load a board
//         app.insert_resource(TickEvent(Timer::from_seconds(1.0, true)));
//         app.insert_resource(self.pause_state.clone());
//         // app.add_startup_system(Self::create_board); // does not have BoardAsset Resource
//         app.add_system_set(
//             SystemSet::on_enter(self.running_state.clone())
//                 .with_system(systems::spawn_debug_block)
//                 .with_system(systems::create_board),
//         )
//         .add_system(systems::stop::<T>.run_on_event::<GameOverEvent>())
//         .add_system(tick_mytimer)
//         .add_system_set(
//             ConditionSet::new()
//                 // SystemSet::on_update(self.running_state.clone())
//                 .run_if(tick_timer)
//                 .run_if(game_running)
//                 // .label("tick")
//                 // .after(tick_mytimer)
//                 .with_system(systems::spawn_tetromino)
//                 .with_system(systems::rotate)
//                 .with_system(systems::gogo)
//                 .with_system(systems::line)
//                 .with_system(systems::load_and_save::<T>)
//                 .with_system(systems::tock)
//                 .into(),
//         )
//         .add_system_set(
//             ConditionSet::new()
//                 .run_if(tick_timer)
//                 .run_if(game_running)
//                 // .label("collapse")
//                 // .after("tick")
//                 .with_system(systems::line)
//                 .into(),
//         )
//         .add_system_set(
//             SystemSet::on_exit(self.running_state.clone()).with_system(Self::cleanup_board),
//         )
//         .add_event::<SpawnEvent>()
//         .add_event::<GameOverEvent>()
//         .add_event::<GameCommand>()
//         .add_event::<RotateEvent>()
//         .add_event::<MoveEvent>();

//         #[cfg(feature = "debug")]
//         {
//             // registering custom component to be able to edit it in inspector
//             app.register_inspectable::<Tetromino>();
//             app.register_inspectable::<CurrentTetromino>();
//             app.register_inspectable::<Coordinates>();
//         }
//         log::info!("Loaded Board Plugin");
//     }
// }
