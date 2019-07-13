#![feature(slice_patterns)]

mod components;
mod mapgen;
mod meta_dispatcher;
mod resources;
mod systems;

use specs::{
    prelude::*,
    saveload::{MarkedBuilder, U64Marker, U64MarkerAllocator},
};
use tcod::{
    colors,
    input::{self, Event, Key, Mouse},
};

use crate::{
    components::*,
    meta_dispatcher::MetaDispatcher,
    resources::{
        input_action::InputAction,
        map::Map,
        menu::{Menu, MenuKind},
        messages::Messages,
        state::State,
        targeting::Targeting,
        ui::{self, UIConfig, UIState, PANEL_HEIGHT},
    },
    systems::{save::Synthetic, *},
};

fn build_dispatcher<'a, 'b>() -> MetaDispatcher<'a, 'b> {
    let mut meta = MetaDispatcher::new();

    meta.once(
        DispatcherBuilder::new()
            .with(LocationHistorySystem, "location_history", &[])
            .with(InputSystem, "input", &[])
            .with(MenuSystem, "menu", &["input"])
            .build(),
    );

    meta.run_while(
        |world| {
            let state = world.read_resource::<State>();
            if *state != State::Game {
                return false;
            }
            let energy = world.read_storage::<Energy>();
            let entity = world.entities();
            let player = world.read_storage::<Player>();
            let action = world.read_storage::<Action>();
            let input_action = world.read_resource::<InputAction>();
            // Keep running the think-act loop as long as
            (&entity, &energy)
                .join()
                .find(|(entity, energy)| {
                    // there is at least one entity with enough energy to act
                    if !energy.can_act() {
                        false
                    } else {
                        let is_player = player.get(*entity).is_some();
                        // If that entity is the player
                        if is_player {
                            // tick only if the player has input an action (otherwise we just wait)
                            let is_waiting_for_input = action
                                .get(*entity)
                                .map(|action| *action == Action::WaitForInput)
                                .unwrap_or(false);
                            if !is_waiting_for_input {
                                return true;
                            }
                            let has_input = *input_action != InputAction::Noop;
                            has_input
                        } else {
                            true
                        }
                    }
                })
                .is_some()
        },
        DispatcherBuilder::new()
            .with(AISystem, "ai", &[])
            .with(MoveAndMeleeSystem, "move_and_melee", &["ai"])
            .with(CollisionSystem, "collision", &["move_and_melee"])
            .with(MovementSystem, "movement", &["move_and_melee", "collision"])
            .with(SkipSystem, "skip", &["ai"])
            .with(DropSystem, "drop", &["ai"])
            .with(UseItemSystem, "use_item", &["ai"])
            .with(PickUpSystem, "pick_up", &["ai"])
            .with(MonsterDeathSystem, "monster_death", &["move_and_melee"])
            .with(PlayerDeathSystem, "player_death", &["move_and_melee"])
            .build(),
    );

    meta.once(
        DispatcherBuilder::new()
            .with(FovSystem, "fov", &[])
            .with(FogOfWarSystem, "fog_of_war", &["fov"])
            .with(TimeSystem, "time", &[])
            .with_thread_local(RenderSystem)
            .build(),
    );

    meta
}

fn setup_ecs(world: &mut World, dispatcher: &mut MetaDispatcher) {
    world.add_resource::<Option<Targeting>>(None);
    world.add_resource::<Option<Menu>>(None);
    world.add_resource(U64MarkerAllocator::new());
    world.register::<Item>();
    world.register::<U64Marker>();
    world.register::<Map>();
    world.register::<Messages>();
    world.register::<Synthetic>();
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
}

fn create_fov_map(world: &mut World) {
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
            always_visible: false,
        })
        .with(Collider::new())
        .with(Player::new())
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
        .with(Action::WaitForInput)
        .with(Ai::Player)
        .with(Energy::new())
        .marked::<U64Marker>()
        .build();
}

fn next_level(world: &mut World) {
    // TODO This should probably move into a system at some point
    // First, clean up everything from the current level
    let entities: Vec<Entity> = (
        &world.entities(),
        &world.read_storage::<Position>(),
        !&world.read_storage::<Player>(),
    )
        .join()
        .map(|j| j.0)
        .collect();
    world.delete_entities(&entities).unwrap();

    // Then create the new map
    new_map(world);
    create_fov_map(world);

    // And move the player to the spawn point of the new map
    let player_entity: Entity = (&world.entities(), &world.read_storage::<Player>())
        .join()
        .next()
        .unwrap()
        .0;
    let spawn_point = world.read_resource::<Map>().spawn_point.clone();
    world
        .write_storage::<Position>()
        .insert(player_entity, spawn_point)
        .unwrap();
}

fn get_action(world: &World) -> InputAction {
    *world.read_resource::<InputAction>()
}

fn exited(world: &World) -> bool {
    get_action(world) == InputAction::Exit
}

fn window_closed(world: &World) -> bool {
    let ui = world.read_resource::<UIState>();
    let consoles_mutex = ui.consoles.clone();
    let consoles = &mut *consoles_mutex.lock().unwrap();
    consoles.root.window_closed()
}

fn game_loop(world: &mut World, dispatcher: &mut MetaDispatcher) {
    dispatcher.dispatch(&world);
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
        dispatcher.dispatch(&world);
        match get_action(world) {
            InputAction::NewGame => new_game(world),
            InputAction::LoadGame => load_game(world),
            InputAction::MainMenu => {
                save_game(world);
                end_game(world);
                main_menu(world);
                dispatcher.dispatch(&world);
            }
            InputAction::NextLevel => next_level(world),
            _ => (),
        }
    }
}

fn end_game(world: &mut World) {
    world.write_resource::<Messages>().clear();
    world.delete_all();
    world.maintain();
}

fn new_game(world: &mut World) {
    end_game(world);
    new_map(world);
    create_fov_map(world);
    spawn_player(world);
    welcome_message(world);
    world.add_resource(State::Game);
    world.maintain();
}

fn save_game(world: &mut World) {
    SavePrepSystem.run_now(&world.res);
    world.maintain();
    SaveSystem.run_now(&world.res);
    world.maintain();
}

fn load_game(world: &mut World) {
    // Start from a clean state
    end_game(world);

    // Uncool: create fake instances so that the resources exist.
    // This might mean it'd be better to maybe make all these resources Option<_>?
    world.add_resource(Map::empty());
    world.add_resource(Messages::new(0));

    // Do the actual loading
    LoadSystem.run_now(&world.res);
    create_fov_map(world);

    // Start the game
    world.add_resource(State::Game);
    world.maintain();
}

fn main_menu(world: &mut World) {
    world.add_resource(InputAction::MainMenu);
    MenuSystem.run_now(&world.res);
    world.add_resource(InputAction::Noop);
}

fn main() {
    let mut world = World::new();
    let mut dispatcher = build_dispatcher();
    setup_ecs(&mut world, &mut dispatcher);
    initialize_ui(&mut world);

    main_menu(&mut world);
    game_loop(&mut world, &mut dispatcher);
}
