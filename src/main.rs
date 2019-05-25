use specs::*;
use systems::*;
use tcod::colors;
use tcod::input::Mouse;
use tcod::input::{self, Event, Key};

mod components;
mod mapgen;
mod resources;
mod systems;

use crate::components::*;
use crate::resources::map::Map;
use crate::resources::menu::{Menu, MenuKind};
use crate::resources::messages::Messages;
use crate::resources::targeting::Targeting;
use crate::resources::ui::{self, UIConfig, UIState, PANEL_HEIGHT};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    NewGame,
    LoadGame,
    MainMenu,
    Exit,
}

impl Default for PlayerAction {
    fn default() -> PlayerAction {
        PlayerAction::DidntTakeTurn
    }
}

fn build_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    // TODO define a separate dispatcher for the main menu, and so get rid of Option<> resources
    DispatcherBuilder::new()
        .with(InputSystem, "input", &[])
        .with(LocationHistorySystem, "location_history", &[])
        .with_barrier() // Player turn
        .with(AttackSystem, "player_attack", &["input"])
        .with(CollisionSystem, "player_collision", &["player_attack"])
        .with(
            MovementSystem,
            "player_movement",
            &["player_collision", "location_history"],
        )
        .with(FovSystem, "fov", &["player_movement"])
        .with(MonsterDeathSystem, "monster_death", &["player_attack"])
        .with_barrier() // Monster turn
        .with(AISystem, "ai", &[])
        .with(AttackSystem, "monster_attack", &["ai"])
        .with(CollisionSystem, "monster_collision", &["monster_attack"])
        .with(MovementSystem, "monster_movement", &["monster_collision"])
        .with(PlayerDeathSystem, "player_death", &["monster_attack"])
        .with_barrier() // Rendering
        .with(FogOfWarSystem, "fog_of_war", &["fov"])
        .with_thread_local(RenderSystem)
        .build()
}

fn setup_ecs(world: &mut World, dispatcher: &mut Dispatcher) {
    world.add_resource(PlayerAction::default());
    world.add_resource::<Option<Targeting>>(None);
    world.add_resource::<Option<Menu>>(None);
    world.add_resource(Messages::new(PANEL_HEIGHT as usize));
    world.register::<Item>();
    dispatcher.setup(&mut world.res);
}

fn welcome_message(world: &mut World) {
    world.write_resource::<Messages>().push(
        "Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.",
        colors::RED,
    );
}

fn initialize_ui(world: &mut World) {
    let ui_config = UIConfig::new();
    let ui_state = ui::init(ui_config);
    world.add_resource(ui_state);
    world.add_resource(Mouse::default());
}

fn new_map(world: &mut World) {
    Map::new_random(world);
    let fov_map = systems::fov::new_fov_map(&world.read_resource::<Map>().tiles);
    world.add_resource(fov_map);
}

fn spawn_player(world: &mut World) {
    let spawn_point = world.read_resource::<Map>().spawn_point.clone();
    world
        .create_entity()
        .with(spawn_point)
        .with(Velocity::new())
        .with(Visual {
            char: '@',
            color: colors::WHITE,
        })
        .with(Collider)
        .with(Player)
        .with(Name::new("player"))
        .with(PreviousPosition { x: -1, y: -1 })
        .with(Living {
            alive: true,
            max_hp: 30,
            hp: 30,
            defense: 2,
        })
        .with(Power(5))
        .with(Inventory::new())
        .build();
}

fn get_action(world: &World) -> PlayerAction {
    *world.read_resource::<PlayerAction>()
}

fn exited(world: &World) -> bool {
    get_action(world) == PlayerAction::Exit
}

fn window_closed(world: &World) -> bool {
    let ui = world.read_resource::<UIState>();
    let consoles_mutex = ui.consoles.clone();
    let consoles = &mut *consoles_mutex.lock().unwrap();
    consoles.root.window_closed()
}

fn game_loop(world: &mut World, dispatcher: &mut Dispatcher) {
    dispatcher.dispatch(&world.res);
    while !exited(world) && !window_closed(world) {
        world.maintain();
        {
            match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
                Some((_, Event::Mouse(m))) => {
                    *world.write_resource() = m;
                    *world.write_resource::<Option<Key>>() = None;
                }
                Some((_, Event::Key(k))) => *world.write_resource() = Some(k),
                _ => *world.write_resource::<Option<Key>>() = None,
            }
        }
        dispatcher.dispatch(&world.res);
        if get_action(world) == PlayerAction::NewGame {
            end_game(world);
            new_game(world);
            dispatcher.dispatch(&world.res);
        }
        if get_action(world) == PlayerAction::MainMenu {
            end_game(world);
            main_menu(world);
            dispatcher.dispatch(&world.res);
        }
    }
}

fn end_game(world: &mut World) {
    world.write_resource::<Messages>().clear();
    world.delete_all();
    world.maintain();
}

fn new_game(world: &mut World) {
    new_map(world);
    spawn_player(world);
    welcome_message(world);
    world.maintain();
}

fn main_menu(world: &mut World) {
    *world.write_resource::<Option<Menu>>() = Some(Menu {
        items: vec![
            "Play a new game".to_string(),
            "Continue last game".to_string(),
            "Quit".to_string(),
        ],
        header: "".to_string(),
        width: 24,
        kind: MenuKind::Main,
    })
}

fn main() {
    let mut world = World::new();
    let mut dispatcher = build_dispatcher();
    setup_ecs(&mut world, &mut dispatcher);
    initialize_ui(&mut world);

    main_menu(&mut world);
    game_loop(&mut world, &mut dispatcher);
}
