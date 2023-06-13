//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.
//#[cfg(target_os = "macos")]
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, CompositeAlphaMode,WindowResolution};

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    NewGame,
    Game,
    //GameMenu
}


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: ("Pet Fun").to_string(),
                resolution: WindowResolution::new(800., 600.).with_scale_factor_override(1.0),
                // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
                transparent: true,
                // Disabling window decorations to make it feel more like a widget than a window
                decorations: true,
                //#[cfg(target_os = "macos")]
                //composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                ..default()
            }),
            ..default()
        }))
        // Insert as resource the initial value for the settings resources
        //.insert_resource(DisplayQuality::Medium)
        //.insert_resource(Volume(7))
        
        .add_startup_system(setup)
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_state::<GameState>()
        // Adds the plugins for each state
        .add_plugin(splash::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

mod splash {
    use bevy::prelude::*;

    use super::{despawn_screen, GameState};

    // This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
            app
                // When entering the state, spawn everything needed for this screen
                .add_system(splash_setup.in_schedule(OnEnter(GameState::Splash)))
                // While in this state, run the `countdown` system
                .add_system(countdown.in_set(OnUpdate(GameState::Splash)))
                // When exiting the state, despawn everything that was spawned for this screen
                .add_system(
                    despawn_screen::<OnSplashScreen>.in_schedule(OnExit(GameState::Splash)),
                );
        }
    }

    // Tag component used to tag entities added on the splash screen
    #[derive(Component)]
    struct OnSplashScreen;

    // Newtype to use a `Timer` for this screen as a resource
    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("branding/welcome-page.png");
        // Display the logo
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    ..default()
                },
                OnSplashScreen,
            ))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: Style {
                        // This will set the logo to be 200px wide, and auto adjust its height
                        size: Size::new(Val::Px(512.0), Val::Auto),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                });
            });
        // Insert the timer as a resource
        commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }

    // Tick the timer, and change state when finished
    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Menu);
        }
    }
}

mod game {
    use super::{despawn_screen, GameState,  TEXT_COLOR};
   // #[cfg(target_os = "macos")]
    use bevy::{
        prelude::*,
    };
    // This plugin will contain the game. In this case, it's just be a screen that will
    // display the current settings for 5 seconds before returning to the menu
    
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum PlayMenuState {
        Show,
        FeedMenu,
        #[default]
        Disable,
    }
    
    pub struct GamePlugin;
    
    impl Plugin for GamePlugin {
        
        fn build(&self, app: &mut App ) {
            app.add_state::<PlayMenuState>()
            .add_systems((  
                game_setup.in_schedule(OnEnter(GameState::Game)),
                
                //game.in_set(OnUpdate(GameState::Game)),
                despawn_screen::<OnGameScreen>.in_schedule(OnExit(GameState::Game)),
            ))
            .add_systems((
                play_menu_show.run_if(in_state(GameState::Game)),
                play_menu_setup.in_schedule(OnEnter(PlayMenuState::Show)),
                despawn_screen::<OnPlayMenuScreen>.in_schedule(OnExit(PlayMenuState::Show)),
            ))
            .add_systems((play_menu_action, button_system).in_set(OnUpdate(PlayMenuState::Show)));
       
        }
    }


    // Tag component used to tag entities added on the game screen
    #[derive(Component)]
    struct OnGameScreen;

    #[derive(Component)]
    struct OnPlayMenuScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);


    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    #[derive(Component)]
    enum PlayMenuButtonAction {
        //PlayMenu,
        FeedMenu,
        WakeUpPet, //Wake up pet
        SleepPet,//Make pet sleep
        IdlePet, //Make pet into idle situation
        BackToMain,
    }
    
    fn game_setup(mut commands: Commands,asset_server: Res<AssetServer>,) {
    
        commands
        .spawn((
            SpriteBundle{
                texture: asset_server.load("textures/turtle-1.png"),
                ..default()
            },
            OnGameScreen,
        ));
            
        // Spawn a 5 seconds timer to trigger going back to the menu
        commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
        

    }

    // Tick the timer, and change state when finished
    fn game(
        time: Res<Time>,
        mut game_state: ResMut<NextState<GameState>>,
        mut timer: ResMut<GameTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Menu);
        }
    }
    // Tag component used to mark which setting is currently selected
    #[derive(Component)]
    struct SelectedOption;

    // This system handles changing all buttons color based on mouse interaction
    fn button_system(
        mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color, selected) in &mut interaction_query {
            *color = match (*interaction, selected) {
                (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    } 
 
    fn play_menu_show(  
        mouse_button_input: Res<Input<MouseButton>>,
        mut menu_state: ResMut<NextState<PlayMenuState>>,
        //game_state: ResMut<State<GameState>>,
    ) {          
                         
                if mouse_button_input.pressed(MouseButton::Right){
                    menu_state.set(PlayMenuState::Show);
                }

            
                
                    
    }   

   fn play_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(86.0), Val::Px(40.0)),
        margin: UiRect{
            left: Val::Px(7.0),
            right: Val::Px(7.0),
            top: Val::Px(8.0),
            bottom: Val::Px(8.0)
        },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        size: Size::new(Val::Px(30.0), Val::Auto),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        position: UiRect {
            left: Val::Px(10.0),
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
        },
        ..default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 25.0,
        color: TEXT_COLOR,
    };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnPlayMenuScreen,
            ))
            //Show five botton
            //Feed
            //Game
            //Wake or sleep
            //Idle
            //Back to main
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle{
                        style:Style { 
                            
                            flex_direction: FlexDirection::Row, 
                            position: UiRect{left:Val::Px(0.),top:Val::Px(142.),..default()},
                            size: Size{width:Val::Px(500.),height:Val::Px(56.)},
                            ..default()
                        },
                        
                        background_color: Color::ORANGE.into(),
                         ..default()
                    })
                    .with_children(|parent| {       
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                PlayMenuButtonAction::BackToMain,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section("Feed",button_text_style.clone(),));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                PlayMenuButtonAction::BackToMain,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section("Wake Up",button_text_style.clone(),));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                PlayMenuButtonAction::BackToMain,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section("Sleep",button_text_style.clone(),));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                PlayMenuButtonAction::BackToMain,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section("Idle",button_text_style.clone(),));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                PlayMenuButtonAction::BackToMain,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section("Back",button_text_style.clone(),));
                            });
                    });
                   
            });     
            
    }
    
    fn play_menu_action(interaction_query: Query<(&Interaction, &PlayMenuButtonAction),(Changed<Interaction>, With<Button>),>,
    mut play_menu_state: ResMut<NextState<PlayMenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
        
    ) {
  
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Clicked {
                match menu_button_action {
                   // PlayMenuButtonAction::Quit => app_exit_events.send(AppExit),
                    PlayMenuButtonAction::FeedMenu => play_menu_state.set(PlayMenuState::FeedMenu),
                    PlayMenuButtonAction::WakeUpPet => play_menu_state.set(PlayMenuState::FeedMenu),
                    PlayMenuButtonAction::SleepPet => play_menu_state.set(PlayMenuState::FeedMenu),
                    PlayMenuButtonAction::IdlePet => play_menu_state.set(PlayMenuState::FeedMenu),
                    PlayMenuButtonAction::BackToMain => {
                        
                        
                        play_menu_state.set(PlayMenuState::Disable);
                        game_state.set(GameState::Menu);
                        //exit play menu
                    },
                
                }
            }
        }
 
    }

}




mod menu {
    use bevy::{app::AppExit, prelude::*};

    use super::{despawn_screen,  GameState,  TEXT_COLOR};

    // This plugin manages the menu, with 5 different screens:
    // - a main menu with "New Game", "Settings", "Quit"
    // - a settings menu with two submenus and a back button
    // - two settings screen with a setting that can be set and a back button
    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app
                // At start, the menu is not enabled. This will be changed in `menu_setup` when
                // entering the `GameState::Menu` state.
                // Current screen in the menu is handled by an independent state from `GameState`
                .add_state::<MenuState>()
                .add_system(menu_setup.in_schedule(OnEnter(GameState::Menu)))
                // Systems to handle the main menu screen
                .add_systems((
                    main_menu_setup.in_schedule(OnEnter(MenuState::Main)),
                    despawn_screen::<OnMainMenuScreen>.in_schedule(OnExit(MenuState::Main)),
                ))

                // Systems to handle the new game menu screen
                .add_systems((
                    new_game_setup.in_schedule(OnEnter(MenuState::NewGame)),
                    despawn_screen::<OnNewGameScreen>.in_schedule(OnExit(MenuState::NewGame)),
                ))                
                // Systems to handle the continue play menu screen
                /* 
                .add_systems((
                    play_menu_setup.in_schedule(OnEnter(MenuState::PlayMenu)),
                    despawn_screen::<OnPlayMenuScreen>.in_schedule(OnExit(MenuState::PlayMenu)),
                ))
                // Systems to handle the feed menu screen
                .add_systems((
                    feed_menu_setup.in_schedule(OnEnter(MenuState::FeedMenu)),
                    //setting_button::<DisplayQuality>.in_set(OnUpdate(MenuState::SettingsDisplay)),
                    //setting_button::<Volume>.in_set(OnUpdate(MenuState::SettingsSound)),
                    despawn_screen::<OnFeedMenuScreen>.in_schedule(OnExit(MenuState::FeedMenu)),
                ))
                */
                // Systems to handle the settings menu screen
                .add_systems((
                    settings_menu_setup.in_schedule(OnEnter(MenuState::Settings)),
                    despawn_screen::<OnSettingsMenuScreen>.in_schedule(OnExit(MenuState::Settings)),
                ))

                // Common systems to all screens that handles buttons behaviour
                .add_systems((menu_action, button_system).in_set(OnUpdate(GameState::Menu)));
        }
    }

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        NewGame,
        PlayMenu,
        FeedMenu,
        Settings,
        Update,
        Transcation,
        #[default]
        Disabled,
    }

    // Tag component used to tag entities added on the main menu screen
    #[derive(Component)]
    struct OnMainMenuScreen;

    // Tag component used to tag entities added on the new game screen
    #[derive(Component)]
    struct OnNewGameScreen;

    // Tag component used to tag entities added on the play menu screen
    #[derive(Component)]
    struct OnPlayMenuScreen;

    // Tag component used to tag entities added on the feed menu screen
    #[derive(Component)]
    struct OnFeedMenuScreen;

    // Tag component used to tag entities added on the settings menu screen
    #[derive(Component)]
    struct OnSettingsMenuScreen;

    // Tag component used to tag entities added on the update screen
    #[derive(Component)]
    struct OnUpdateScreen;

    // Tag component used to tag entities added on the transaction screen
    #[derive(Component)]
    struct OnTransactionScreen;

    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    // Tag component used to mark which setting is currently selected
    #[derive(Component)]
    struct SelectedOption;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum MenuButtonAction {
        NewGame, //Create a new Game
        ContinueGame, //Continue the Game
        Settings,//Game settings
        Update, //Pet state update
        Transaction,//Buy or Sell pet
        BackToMainMenu,    
        Quit,
    }

    // This system handles changing all buttons color based on mouse interaction
    fn button_system(
        mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color, selected) in &mut interaction_query {
            *color = match (*interaction, selected) {
                (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    // This system updates the settings when a new value for a setting is selected, and marks
    // the button as the one currently selected
    

    
    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        // Common style for all buttons on the screen
        let button_style = Style {
            size: Size::new(Val::Px(250.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_style = Style {
            size: Size::new(Val::Px(30.0), Val::Auto),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            position: UiRect {
                left: Val::Px(10.0),
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
            },
            ..default()
        };
        let button_text_style = TextStyle {
            font: font.clone(),
            font_size: 25.0,
            color: TEXT_COLOR,
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnMainMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::DARK_GREEN.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Display the game name
                        parent.spawn(
                            TextBundle::from_section(
                                "Welcome to Window Pet!",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 30.0,
                                    color: TEXT_COLOR,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(20.0)),
                                ..default()
                            }),
                        );

                        // Display buttons for each action available from the main menu:
                        // - new game
                        // - settings
                        // - quit
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::NewGame,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/right.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "New Game",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::ContinueGame,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/right.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Continue",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Settings,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/wrench.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Settings", 
                                    button_text_style.clone()
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                //MenuButtonAction::Update,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/wrench.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Update", 
                                    button_text_style.clone()
                                ));
                            });       
                            parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                               // MenuButtonAction::Transaction,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/wrench.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style,
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Transcation", 
                                    button_text_style
                                ));
                            }); 
                    
                    
                    
                    });
                       
            });
    }
    //New game menu setup
    fn new_game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        commands
        .spawn((
            TextBundle::from_section(
                "This menu will be update later!",
                TextStyle {
                    font: font,
                    font_size: 50.0,
                    color: TEXT_COLOR,
                },
            )
            .with_style(Style {
                margin:UiRect::all(Val::Px(20.0)),
                ..default()
            }),
            OnNewGameScreen,
        ));
        //////////////////////////////////////////////////////////////////////

    }

    fn settings_menu_setup() {
       
    }

    fn menu_action(interaction_query: Query<(&Interaction, &MenuButtonAction),(Changed<Interaction>, With<Button>),>,
        mut app_exit_events: EventWriter<AppExit>,mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
      
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Clicked {
                match menu_button_action {
                    MenuButtonAction::Quit => app_exit_events.send(AppExit),
                    MenuButtonAction::NewGame => menu_state.set(MenuState::NewGame),
                    MenuButtonAction::ContinueGame => {
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                    
                    MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                    MenuButtonAction::Update => menu_state.set(MenuState::Update),
                    MenuButtonAction::Transaction => menu_state.set(MenuState::Transcation),
                    

                    MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                    
                    _=>menu_state.set(MenuState::Main),
                }
            }
        }
     
    }

}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}