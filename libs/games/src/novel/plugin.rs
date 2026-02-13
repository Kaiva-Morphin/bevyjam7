use std::time::Duration;

use crate::{actors, backgrounds, global_music::plugin::NewBgMusic, novel::engine::NovelStage, novel_music, prelude::*, properties::{AppState, LastState}, sound_effects, stages};
use bevy::{audio::{PlaybackMode, Volume}, text::{FontSmoothing, LineHeight}};
use bevy_asset_loader::prelude::AssetCollection;
use crate::novel::engine::*;



const STATE: AppState = AppState::Novel;
const NEXT_STATE: AppState = AppState::Novel2Fnaf;

const CHARS_PER_SECOND : f32 = 20.0;

pub struct NovelPlugin;


#[derive(Resource)]
pub struct NovelState {
    pub stages: Vec<NovelStage>,
    pub current_stage: usize,
    pub chars_shown: usize,
    pub chars_total: usize,
    pub t: Timer,
    pub current_music: NovelMusic,
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
    #[asset(path = "fonts/kaivs_minegram_v1.ttf")]
    font: Handle<Font>,
    #[asset(path = "fonts/kaivs_minegram_v1-italic.ttf")]
    italic: Handle<Font>,
}


actors! {
    Freddy => "images/novel/actors/faz.png",
    FreddyNight => "images/novel/actors/faz_night.png",
    CJ => "images/novel/actors/cj.png",
    Bob => "images/novel/actors/bobux.png",
    Ass => "images/novel/actors/assasino.png",
    Ass2 => "images/novel/actors/assasino1.png",
    Bal => "images/novel/actors/ballerina.png",
    Bal2 => "images/novel/actors/ballerina1.png",
    Time => "images/novel/actors/time.png",
    Rust => "images/novel/actors/rust.png",
    Go => "images/novel/actors/golang.png",
}

backgrounds! {
    StreetAutumnNight => "images/novel/bg/Street_Autumn_Night.png",
    LivingroomDark => "images/novel/bg/Livingroom_Dark.png",
    BedroomNight => "images/novel/bg/Bedroom_Night_Dark.png",
    KitchenNight => "images/novel/bg/Kitchen_Night.png",
    GroveStreet => "images/novel/bg/grove.jpg",
    Computer => "images/novel/bg/supercomputer.png",
    ComputerWithBevy => "images/novel/bg/supercomputer&bevy.png",
}

sound_effects! {
    PipeFall => "sounds/novel/metal-pipe-falling-sound.mp3",
    Ur => "sounds/novel/ur.mp3",
    Ururur => "sounds/novel/ururur.mp3",
    Sip => "sounds/novel/252274__reitanna__sipping2.wav",
}

novel_music! {
    Journey => "sounds/novel/poopie pack_journey.wav",
    Battle => "sounds/novel/poopie pack_boss battle.wav",
    Silence => "sounds/novel/silence.mp3",
    AssasinoBattle => "sounds/novel/Overhaul.mp3",
    MyOnlyLove => "sounds/novel/My Only Love.mp3",
}

const LEFT : Transform = Transform::from_translation(Vec3::new(-150.0, 0.0, 0.0));
const RIGHT : Transform = Transform::from_translation(Vec3::new(150.0, 0.0, 0.0));

impl Default for NovelState {
    fn default() -> Self {
        Self {
            chars_shown: 0,
            chars_total: 0,
            current_music: NovelMusic::Journey,
            t: Timer::from_seconds(1.0 / CHARS_PER_SECOND, TimerMode::Repeating),
            current_stage: 0,
            stages: stages! {
                StreetAutumnNight Journey {
                    => "What a nice evening!"
                },
                StreetAutumnNight Journey {=> "I think I should walk around a bit more..."},
                GroveStreet Battle {
                    CJ (flip_x = true, transform = Transform::from_scale(Vec3::splat(0.5)))
                    => ("CJ") "Aye, whatcha doin here man?"
                },
                GroveStreet Battle {
                    CJ (flip_x = true, transform = Transform::from_scale(Vec3::splat(0.5)))
                    => ("CJ") "This ain't your hood"
                },
                GroveStreet Battle {
                    CJ (flip_x = true, transform = Transform::from_scale(Vec3::splat(0.5)))
                    => ("CJ") "Looking for trouble?"
                },
                GroveStreet Journey {
                    CJ (transform = RIGHT.with_scale(Vec3::splat(0.5))),
                    Bob (transform = LEFT),
                    => ("Bobby") "Chill mate, guy buys stuff from me \nLet him off the hook"
                },
                GroveStreet Journey {
                    CJ (transform = RIGHT.with_scale(Vec3::splat(0.5))),
                    Bob (transform = LEFT),
                    => ("CJ") "Fine Bobby, but only this time"
                },
                GroveStreet Journey {
                    CJ (flip_x = true, transform = Transform::from_scale(Vec3::splat(0.5)))
                    => ("CJ") "Now get the hell outta here"
                },
                StreetAutumnNight Journey {=> "Well, that was one hell of an encounter"},
                StreetAutumnNight Journey {=> "I'd better head home..."},
                LivingroomDark Journey {
                    =>
                    "Gotta get some coffee..."
                },
                LivingroomDark Silence {
                    =>
                    "What's that sound?" (Ur)
                },
                LivingroomDark Silence {
                    Freddy
                    => ("Freddy") "ur ur \n urur" (Ururur)
                },
                LivingroomDark Silence {
                    Freddy (flip_x = true)
                    => ""
                },
                LivingroomDark Silence {
                    Freddy (flip_x = true, transform = RIGHT)
                    => ""
                },
                LivingroomDark Silence {
                    => ""
                },
                LivingroomDark Silence {
                    => "..."
                },
                KitchenNight Journey {
                    => "Anyway... What coffee do I have here?"
                },
                KitchenNight AssasinoBattle {
                    => "Oh right, these two"
                },
                KitchenNight AssasinoBattle {
                    Bal (transform = RIGHT.with_scale(Vec3::splat(0.3))),
                    Ass (transform = LEFT.with_scale(Vec3::splat(0.2))),
                    => ("Ballerina Cappuccina") "Oh no, darling...\nHe's here for us..."
                },
                KitchenNight AssasinoBattle {
                    Bal (transform = RIGHT.with_scale(Vec3::splat(0.3))),
                    Ass (transform = LEFT.with_scale(Vec3::splat(0.2))),
                    => ("Ballerina Cappuccina") "No, my love, I can't lose you!\nYou're the love of my life!"
                },
                KitchenNight AssasinoBattle {
                    Bal2 (transform = RIGHT.with_scale(Vec3::splat(0.3))),
                    Ass2 (transform = LEFT.with_scale(Vec3::splat(0.2))),
                    => ("Cappuccino Assassino") "I have to let him drink me, so you can live another day...\nGoodbye my love..."
                },
                KitchenNight AssasinoBattle {
                    Bal2 (transform = RIGHT.with_scale(Vec3::splat(0.3))),
                    => ("Ballerina Cappuccina") "NOOOOOOOO" (Sip)
                },
                Computer Silence {
                    => "Hmm, I guess I should do some work..."
                },
                Computer MyOnlyLove {
                    Time (transform = Transform::from_xyz(0., 35., 0.).with_scale(Vec3::splat(0.15))),
                    => "Oh yes, it's time for bevy jam!"
                },
                Computer MyOnlyLove {
                    => "Let's see what's the topic this time"
                },
                Computer MyOnlyLove {
                    => "..."
                },
                Computer MyOnlyLove {
                    => "Bevy editor from another world!"
                },
                Computer MyOnlyLove {
                    => "What an interesting choice!\nI guess any game engine needs an editor..."
                },
                Computer MyOnlyLove {
                    => "But Rust is hard...\nShould I even study rust?"
                },
                Computer Battle {
                    Rust (transform = Transform::from_xyz(115., 30., 0.).with_scale(Vec3::splat(0.25))),
                    Go (transform = Transform::from_xyz(-130., 15., 0.).with_scale(Vec3::splat(0.3)), flip_x = true),
                    => ("Go") "There's no point in rust\nBackend can be done with kotlin or golang"
                },
                Computer Battle {
                    Rust (transform = Transform::from_xyz(115., 30., 0.).with_scale(Vec3::splat(0.25))),
                    Go (transform = Transform::from_xyz(-130., 15., 0.).with_scale(Vec3::splat(0.3)), flip_x = true),
                    => ("Rust") "Rust is definitely faster\nGo even has a garbage collector\nKotlin has a ton of legacy code behind it"
                },
                Computer Silence {
                    => "So what did YOU choose?"
                },
                ComputerWithBevy Silence {
                    => "?"
                },
                ComputerWithBevy Silence {
                    => "..."
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
    fn bg_transform(&self) -> &Transform {
        &self.stages[self.current_stage].bg_transform
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

    fn speaker(&self) -> &String {
        &self.stages[self.current_stage].speaker
    }

    fn sfx(&self) -> Option<&NovelSoundEffect> {
        self.stages[self.current_stage].sfx.as_ref()
    }

    fn music(&self) -> &NovelMusic {
        &self.stages[self.current_stage].music
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
struct SpeakerNode;

#[derive(Component)]
struct TextNode;

#[derive(Component)]
struct ActorSprite;

pub fn setup(
    mut cmd: Commands,
    mut latest: ResMut<LastState>,
    novel: Res<NovelAssets>,
    cam: Query<Entity, With<WorldCamera>>,
    bg: Res<BackgroundsAssets>,
    music: Res<NovelMusicAssets>,
    sfx: Res<NovelSoundEffectsAssets>,
){
    cmd.spawn((
        NewBgMusic{handle: None, instant_translation: true},
    ));
    let s = NovelState::default().inited();
    let cam = cam.iter().next().expect("No cam!");
    let slicer = TextureSlicer {
        border: BorderRect::all(2.0),
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
            children![
                (
                    Text::new(s.speaker()),
                    TextFont {
                        font: novel.font.clone(),
                        font_size: 22.0,
                        font_smoothing: FontSmoothing::None,
                        ..default()
                    },
                    LineHeight::RelativeToFont(0.6),
                    TextColor::WHITE,
                    SpeakerNode,
                    ZIndex(1),
                    Node {
                        top: Val::Px(4.0),
                        left: Val::Px(4.0),
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        margin: UiRect::horizontal(Val::Px(10.0)),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }
                ),
                (
                    Text::new(s.speaker()),
                    TextFont {
                        font: novel.font.clone(),
                        font_size: 22.0,
                        font_smoothing: FontSmoothing::None,
                        ..default()
                    },
                    UiTransform {
                        translation: Val2::px(1.5, 1.5),
                        ..default()
                    },
                    LineHeight::RelativeToFont(0.6),
                    TextColor::from(Srgba::new(0.0, 0.0, 0.0, 0.8)),
                    SpeakerNode,
                    Node {
                        top: Val::Px(4.0),
                        left: Val::Px(4.0),
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        margin: UiRect::horizontal(Val::Px(10.0)),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }
                ),
                (
                    Text::new(""),
                    TextFont {
                        font: novel.italic.clone(),
                        font_size: 22.0,
                        font_smoothing: FontSmoothing::None,
                        ..default()
                    },
                    LineHeight::RelativeToFont(0.7),
                    TextColor::WHITE,
                    TextNode,
                    ZIndex(1),
                    Node {
                        top: Val::Px(20.0),
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
                        font: novel.italic.clone(),
                        font_size: 22.0,
                        font_smoothing: FontSmoothing::None,
                        ..default()
                    },
                    LineHeight::RelativeToFont(0.7),
                    TextNode,
                    Node {
                        top: Val::Px(20.0),
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        margin: UiRect::horizontal(Val::Px(10.0)),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }
                )
            ]
        )]
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        CurrentMusic,
        PlaybackSettings{
            mode: PlaybackMode::Loop,
            volume: Volume::SILENT,
            ..default()
        },
        AudioPlayer::new(s.music().get_asset(&music)),
    ));
    for eff in s.sfx().iter() {
        cmd.spawn((
            DespawnOnExit(STATE),
            CurrentMusic,
            PlaybackSettings{
                mode: PlaybackMode::Once,
                volume: Volume::SILENT,
                ..default()
            },
            AudioPlayer::new(eff.get_asset(&sfx)),
        ));
    }
    cmd.insert_resource(s);
}

#[derive(Component)]
struct CurrentMusic;

#[derive(Component)]
struct PrevMusic;

fn tick(    
    (bg,
    actors,
    music,
    sfx) : (
        Res<BackgroundsAssets>,
        Res<ActorsAssets>,
        Res<NovelMusicAssets>,
        Res<NovelSoundEffectsAssets>,
    ),
    mut state: ResMut<NovelState>,
    mut next: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cmd: Commands,
    mut t_q: Query<&mut Text>,
    s_q: Query<Entity, With<SpeakerNode>>,
    r_q: Query<Entity, With<TextNode>>,
    mut bg_q: Query<(&mut Sprite, &mut Transform), With<BackgroundSprite>>,
    sprite_q: Query<Entity, With<ActorSprite>>,
    current_q: Query<Entity, With<CurrentMusic>>,
    prev_q: Query<Entity, With<PrevMusic>>,
    mut music_q: Query<&mut AudioSink>,
){
    let dt = time.dt();
    for e in current_q.iter() {
        if let Ok(mut s) = music_q.get_mut(e) {
            let volume: Volume = s.volume();
            if volume.to_linear() < 1.0 {
                s.set_volume(Volume::Linear(volume.to_linear() + (NOVEL_MUSIC_INTERPOLATION * dt)));
            }
        }
    }
    for e in prev_q.iter() {
        if let Ok(mut s) = music_q.get_mut(e) {
            let volume: Volume = s.volume();
            if volume.to_linear() > 0.0 {
                s.set_volume(Volume::Linear(volume.to_linear() - (NOVEL_MUSIC_INTERPOLATION * dt)));
            }
            if volume.to_linear() <= 0.01 {
                cmd.entity(e).despawn();
            }
        }
    }

    let pressed = keyboard_input.just_pressed(KeyCode::Space);
    if state.is_finished() {
        if pressed {
            info!("DONBASS");
            cmd.spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                .observe(await_screenshot_and_translate(NEXT_STATE));
        }
        return;
    }
    if state.is_all_chars_shown() {
        if pressed {
            let current  = state.music().clone();
            next_stage(&mut cmd, &mut state, &mut t_q, &s_q, &r_q, &mut bg_q, &sprite_q, &bg, &actors);
            let next = state.music().clone();
            for eff in state.sfx().iter() {
                cmd.spawn((
                    DespawnOnExit(STATE),
                    CurrentMusic,
                    PlaybackSettings{
                        mode: PlaybackMode::Once,
                        volume: Volume::SILENT,
                        ..default()
                    },
                    AudioPlayer::new(eff.get_asset(&sfx)),
                ));
            }
            if current != next {
                for e in current_q.iter() {
                    cmd.entity(e).remove::<CurrentMusic>().insert(PrevMusic);
                }
                cmd.spawn((
                    DespawnOnExit(STATE),
                    CurrentMusic,
                    PlaybackSettings{
                        mode: PlaybackMode::Loop,
                        volume: Volume::SILENT,
                        ..default()
                    },
                    AudioPlayer::new(state.music().get_asset(&music)),
                ));
            }

        }
        return;
    }
    if pressed {
        for t in r_q.iter() {
            if let Ok(mut t) = t_q.get_mut(t) {
                t.0 = state.full_text().to_string();
            }
        }
        state.read_all_text();
        return;
    }
    state.t.tick(Duration::from_secs_f32(dt));
    if state.t.just_finished() {
        state.chars_shown += 1;
        for t in r_q.iter() {
            if let Ok(mut t) = t_q.get_mut(t) {
                t.0 = state.text().to_string();
            }
        }
        state.t.reset();
    }
}


fn next_stage(
    cmd: &mut Commands,
    state: &mut ResMut<NovelState>,
    t_q: &mut Query<&mut Text>,
    s_q: &Query<Entity, With<SpeakerNode>>,
    r_q: &Query<Entity, With<TextNode>>,
    bg_q: &mut Query<(&mut Sprite, &mut Transform), With<BackgroundSprite>>,
    a_q: &Query<Entity, With<ActorSprite>>,
    bg: &Res<BackgroundsAssets>,
    actors: &Res<ActorsAssets>,
) {
    state.next_stage();
    for e in s_q.iter() {
        if let Ok(mut t) = t_q.get_mut(e) {
            t.0 = state.speaker().to_string();
        }
    }
    for e in r_q.iter() {
        if let Ok(mut t) = t_q.get_mut(e) {
            t.0 = "".to_string();
        }
    }

    for e in a_q.iter() {
        cmd.entity(e).despawn();
    }
    for (mut sprite, mut t) in bg_q.iter_mut() {
        sprite.image = state.bg().get_asset(bg);
        *t = state.bg_transform().clone();
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

fn cleanup(
    mut cmd: Commands
){
    cmd.remove_resource::<NovelState>();
}
