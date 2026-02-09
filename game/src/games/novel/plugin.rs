use std::time::Duration;

use crate::{actors, backgrounds, games::{novel::engine::*, plugin::{AppState, LastState}}, prelude::*, stages};
use bevy::{color::palettes::css::RED, text::FontSmoothing};
use bevy_asset_loader::prelude::AssetCollection;
use camera::WorldUiRoot;



const STATE: AppState = AppState::Novel;
const NEXT_STATE: AppState = AppState::PacmanEnter;

const CHARS_PER_SECOND : f32 = 20.0;

pub struct NovelPlugin;


#[derive(Resource)]
pub struct NovelState {
    pub stages: Vec<NovelStage>,
    pub current_stage: usize,
    pub chars_shown: usize,
    pub chars_total: usize,
    pub t: Timer,
}


impl Plugin for NovelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, tick.run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup)
            ;
    }
}

#[derive(Resource, AssetCollection)]
pub struct NovelAssets {
    #[asset(path = "images/novel/bg_pink.png")]
    bg_pink: Handle<Image>,
    #[asset(path = "fonts/kaivs_minegram_v1-italic.ttf")]
    font: Handle<Font>,
}


actors! {
    "Freddy" : Freddy => "images/novel/actors/faz.png",
    "Freddy" : FreddyNight => "images/novel/actors/faz_night.png",
    "CJ" : CJ => "images/novel/actors/cj.png",
    "Bobux" : Bob => "images/novel/actors/bobux.png",
    "Assasino" : Ass => "images/novel/actors/assasino.png",
    "Bellerino" : Bal => "images/novel/actors/ballerina.png",
    "Time" : Time => "images/novel/actors/time.png",
    "Rust" : Rust => "images/novel/actors/rust.jpg",
    "Go" : Go => "images/novel/actors/golang.png",
}

backgrounds!{
    StreetAutumnNight => "images/novel/bg/Street_Autumn_Night.png",
    LivingroomDark => "images/novel/bg/Livingroom_Dark.png",
    BedroomNight => "images/novel/bg/Bedroom_Night_Dark.png",
    KitchenNight => "images/novel/bg/Kitchen_Night.png",
    GroveStreet => "images/novel/bg/grove.jpg",
    Computer => "images/novel/bg/supercomputer.jpg",
}

const LEFT : Vec3 = Vec3::new(-150.0, 0.0, 0.0);
const RIGHT : Vec3 = Vec3::new(150.0, 0.0, 0.0);
impl Default for NovelState {
    fn default() -> Self {
        Self {
            chars_shown: 0,
            chars_total: 0,
            t: Timer::from_seconds(1.0 / CHARS_PER_SECOND, TimerMode::Repeating),
            current_stage: 0,
            stages: stages!{
                StreetAutumnNight {=> "What a nice evening!"},
                StreetAutumnNight {=> "I think I should walk around a bit more..."},
                GroveStreet {
                    CJ (flip_x = true, transform = Transform::from_scale(Vec3::splat(0.5)))
                    => "Aye, whatcha doin here man?"
                },
                GroveStreet {
                    CJ (flip_x = true, transform = Transform::from_scale(Vec3::splat(0.5)))
                    => "This ain't your hood"
                },
                GroveStreet {
                    CJ (flip_x = true, transform = Transform::from_scale(Vec3::splat(0.5)))
                    => "Looking for trouble?"
                },
                GroveStreet {
                    CJ (transform = Transform::from_translation(RIGHT).with_scale(Vec3::splat(0.5))),
                    Bob (transform = Transform::from_translation(LEFT)),
                    => "Chill mate, guy buys stuff from me \nLet him off the hook"
                },
                GroveStreet {
                    CJ (transform = Transform::from_translation(RIGHT).with_scale(Vec3::splat(0.5))),
                    Bob (transform = Transform::from_translation(LEFT)),
                    => "Fine Bobby, but only this time"
                },
                GroveStreet {
                    CJ (flip_x = true, transform = Transform::from_scale(Vec3::splat(0.5)))
                    => "Now get the hell outa here"
                },
                StreetAutumnNight {=> "Well, that was one hell of an encounter"},
                StreetAutumnNight {=> "I'd better head home..."},
                LivingroomDark {
                    =>
                    "Gotta get some coffee..."
                },
                LivingroomDark {
                    =>
                    "What's that sound?"
                },
                LivingroomDark {
                    Freddy
                    => "ur ur \n urur"
                },
                LivingroomDark {
                    Freddy (flip_x = true)
                    => ""
                },
                LivingroomDark {
                    Freddy (flip_x = true, transform = Transform::from_translation(RIGHT))
                    => ""
                },
                LivingroomDark {
                    => ""
                },
                LivingroomDark {
                    => "..."
                },
                KitchenNight {
                    => "What coffee do I have here?\nOh right, these two"
                },
                KitchenNight {
                    Bal (transform = Transform::from_translation(RIGHT).with_scale(Vec3::splat(0.3))),
                    Ass (transform = Transform::from_translation(LEFT).with_scale(Vec3::splat(0.2))),
                    => "Oh no, darling...\nHe's here for us..."
                },
                KitchenNight {
                    Bal (transform = Transform::from_translation(RIGHT).with_scale(Vec3::splat(0.3))),
                    Ass (transform = Transform::from_translation(LEFT).with_scale(Vec3::splat(0.2))),
                    => "No, my love, I can't lose you!\nYou're the love of my life!"
                },
                KitchenNight {
                    Bal (transform = Transform::from_translation(RIGHT).with_scale(Vec3::splat(0.3))),
                    Ass (transform = Transform::from_translation(LEFT).with_scale(Vec3::splat(0.2))),
                    => "I have to let him drink me, so you can live another day...\nGoodbye my love..."
                },
                KitchenNight {
                    Bal (transform = Transform::from_translation(RIGHT).with_scale(Vec3::splat(0.3))),
                    => "NOOOOOOOO"
                },
                Computer {
                    => "Hmm, I guess I should do some work..."
                },
                Computer {
                    Time,
                    => "Oh yes, it's time for bevy jam!"
                },
                Computer {
                    => "Let's see what's the topic this time"
                },
                Computer {
                    => "..."
                },
                Computer {
                    => "Bevy editor from another world"
                },
                Computer {
                    => "What an interesting choice.\nI guess any game engine needs an editor..."
                },
                Computer {
                    => "But Rust is hard...\nshould I even study rust?"
                },
                Computer {
                    Rust (transform = Transform::from_translation(RIGHT)),
                    Go (transform = Transform::from_translation(LEFT)),
                    => "There's no point in rust\nBackend can be done with kotlin or golang"
                },
                Computer {
                    Rust (transform = Transform::from_translation(RIGHT)),
                    Go (transform = Transform::from_translation(LEFT)),
                    => "Rust is definitely faster\nGo even has a garbage collector\nKotlin has a ton of legacy code behind it"
                },
                Computer {
                    Rust (transform = Transform::from_translation(RIGHT)), // todo: rotate
                    Go (transform = Transform::from_translation(LEFT)),
                    => "So what did you choose?"
                },
            },
        }
    }
}



impl NovelState {
    fn next_stage(&mut self) {
        self.current_stage += 1;
        self.chars_shown = 0;
        self.chars_total = self.full_text().chars().count();
        self.t = Timer::from_seconds(1.0 / CHARS_PER_SECOND, TimerMode::Repeating);
    }
    fn bg(&self) -> &Background {
        &self.stages[self.current_stage].bg
    }
    fn actors(&self) -> &Vec<ActorAppearance> {
        &self.stages[self.current_stage].actors
    }
    fn text(&self) -> &str {
        let s = &self.stages[self.current_stage].text;
        let end = s
            .char_indices()
            .nth(self.chars_shown)
            .map(|(i, _)| i)
            .unwrap_or(s.len());
        &s[..end]
    }
    fn full_text(&self) -> &str {
        &self.stages[self.current_stage].text
    }
    fn read_all_text(&mut self) {
        self.chars_shown = self.chars_total;
    }
    fn is_finished(&self) -> bool {
        self.current_stage == self.stages.len() - 1 && self.is_all_chars_shown()
    }
    fn is_all_chars_shown(&self) -> bool {
        self.chars_shown == self.chars_total
    }
    fn init(&mut self) {
        self.current_stage = 0;
        self.chars_shown = 0;
        self.chars_total = self.full_text().chars().count();
        self.t = Timer::from_seconds(1.0 / CHARS_PER_SECOND, TimerMode::Repeating);
    }
    fn inited(mut self) -> Self {
        self.init();
        self
    }
}





#[derive(Component)]
struct BackgroundSprite;

#[derive(Component)]
struct TextNode;

#[derive(Component)]
struct ActorSprite;

fn setup(
    mut cmd: Commands,
    mut latest: ResMut<LastState>,
    novel: Res<NovelAssets>,
    cam: Query<Entity, With<WorldCamera>>,
    bg: Res<BackgroundsAssets>,
){
    let s = NovelState::default().inited();
    let cam = cam.iter().next().expect("No cam!");
    let slicer = TextureSlicer {
        border: BorderRect::all(0.0),
        center_scale_mode: SliceScaleMode::Tile { stretch_value: 2.0 },
        sides_scale_mode: SliceScaleMode::Tile { stretch_value: 2.0 },
        max_corner_scale: 1.0,
    };
    latest.state = STATE;
    cmd.spawn((
        DespawnOnExit(STATE),
        BackgroundSprite,
        Sprite{
            image: s.bg().get_asset(&bg),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., -1.0)
    ));
    cmd.insert_resource(s);
    cmd.spawn((
        DespawnOnExit(STATE),
        UiTargetCamera(cam),
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            ..Default::default()
        }, 
        children![(
            ImageNode {
                image: novel.bg_pink.clone(),
                image_mode: NodeImageMode::Sliced(slicer.clone()),
                ..default()
            },
            Node {
                height: Val::Percent(25.0),
                width: Val::Percent(80.0),
                margin: UiRect::bottom(Val::Percent(2.0)),
                position_type: PositionType::Relative,
                ..Default::default()
            },
            children![(
                Text::new(""),
                TextFont {
                    font: novel.font.clone(),
                    font_size: 22.0,
                    font_smoothing: FontSmoothing::None,
                    ..default()
                },
                TextColor::WHITE,
                TextNode,
                ZIndex(1),
                Node {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                }
            ),
            (
                Text::new(""),
                TextColor::from(Srgba::new(0.0, 0.0, 0.0, 0.8)),
                UiTransform {
                    translation: Val2::px(1.5, 1.5),
                    ..default()
                },
                TextFont {
                    font: novel.font.clone(),
                    font_size: 22.0,
                    font_smoothing: FontSmoothing::None,
                    ..default()
                },
                TextNode,
                Node {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                }
            )]
        )]
    ));
}

fn next_stage(
    mut cmd: Commands,
    state: &mut ResMut<NovelState>,
    t_q: &mut Query<&mut Text, With<TextNode>>,
    bg_q: &mut Query<&mut Sprite, With<BackgroundSprite>>,
    a_q: &Query<Entity, With<ActorSprite>>,
    bg: &Res<BackgroundsAssets>,
    actors: &Res<ActorsAssets>,
) {
    state.next_stage();
    for mut t in t_q.iter_mut() {
        t.0 = "".to_string();
    }
    for e in a_q.iter() {
        cmd.entity(e).despawn();
    }
    for mut sprite in bg_q.iter_mut() {
        sprite.image = state.bg().get_asset(bg);
    }
    for appearance in state.actors() {
        cmd.spawn((
            DespawnOnExit(STATE),
            ActorSprite,
            Sprite {
                image: appearance.actor.get_asset(&actors),
                flip_x: appearance.flip_x,
                ..default()
            },
            appearance.transform,
        ));
    }
}

fn tick(    
    bg: Res<BackgroundsAssets>,
    actors: Res<ActorsAssets>,
    mut state: ResMut<NovelState>,
    mut next: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cmd: Commands,
    mut t_q: Query<&mut Text, With<TextNode>>,
    mut bg_q: Query<&mut Sprite, With<BackgroundSprite>>,
    sprite_q: Query<Entity, With<ActorSprite>>,
){
    let pressed = keyboard_input.just_pressed(KeyCode::Space);
    if state.is_finished() {
        if pressed {
            next.set(NEXT_STATE);
        }
        return;
    }
    if state.is_all_chars_shown() {
        if pressed {
            next_stage(cmd, &mut state, &mut t_q, &mut bg_q, &sprite_q, &bg, &actors);
        }
        return;
    }
    if pressed {
        for mut t in t_q.iter_mut() {
            t.0 = state.full_text().to_string();
        }
        state.read_all_text();
        return;
    }
    let dt = time.dt();
    state.t.tick(Duration::from_secs_f32(dt));
    if state.t.just_finished() {
        state.chars_shown += 1;
        for mut t in t_q.iter_mut() {
            t.0 = state.text().to_string();
        }
        state.t.reset();
    }
}


fn cleanup(
    mut cmd: Commands
){
    cmd.remove_resource::<NovelState>();
}
