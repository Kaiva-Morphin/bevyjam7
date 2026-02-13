use std::time::Duration;

use bevy::{ecs::entity_disabling::Disabled, text::{FontSmoothing, LineHeight}};

use crate::{dev_games::miami::{player::PlayerDisabled, plugin::{MiamiAssets, STATE}}, prelude::*};



#[derive(Component)]
pub struct DialogRot;
#[derive(Component)]
pub struct DialogHead;
#[derive(Component)]
pub struct DialogHeadShadow;

#[derive(Component)]
pub struct TopDialog;

#[derive(Component)]
pub struct BottomDialog;

#[derive(Component)]
pub struct BgDialog;

#[derive(Component)]
pub struct DialogLabel;

#[derive(Component)]
pub struct DialogState {
    state: usize, 
    dialogs: Vec<(String, Speaker)>
}


#[derive(Component)]
pub struct DialogShadowLabel;


pub enum Speaker {
    Pacman,
    Freddy,
}

impl Speaker {
    pub fn to_asset(&self, assets: &Res<super::plugin::MiamiAssets>) -> Handle<Image> {
        match self {
            Self::Pacman => assets.dialog_pac.clone(),
            Self::Freddy => assets.dialog_faz.clone(),
        }
    }
}

pub fn start_dialog(
    cmd: &mut Commands,
    assets: &Res<super::plugin::MiamiAssets>,
    cam: Entity,
    dialogs: Vec<(String, Speaker)>,
){
    if dialogs.len() == 0 {return;}
    let initial_text = dialogs[0].0.clone();
    let speaker = dialogs[0].1.to_asset(assets);
    let main_in = Tween::new(
        EaseFunction::SineOut,
        Duration::from_secs_f32(0.3),
        UiTransformTranslationPxLens {
            start: vec2(300., -10.),
            end: vec2(100., -10.),
        }
    );

    let char_in = Tween::new(
        EaseFunction::SineOut,
        Duration::from_secs_f32(0.3),
        UiTransformTranslationPxLens {
            start: vec2(300., -25.),
            end: vec2(-20., -25.),
        }
    );
    let shadow_in = Tween::new(
        EaseFunction::SineOut,
        Duration::from_secs_f32(0.3),
        UiTransformTranslationPxLens {
            start: vec2(300., -18.),
            end: vec2(-12., -18.),
        }
    );

    let bottom_in = Tween::new(
        EaseFunction::SineOut,
        Duration::from_secs_f32(0.3),
        UiTransformTranslationPxLens {
            start: vec2(0., 200.),
            end: vec2(0., 0.),
        }
    );

    let top_in = Tween::new(
        EaseFunction::SineOut,
        Duration::from_secs_f32(0.3),
        UiTransformTranslationPxLens {
            start: vec2(0., -200.),
            end: vec2(0., 0.),
        }
    );

    cmd.spawn(
        (DialogState{
            state: 0,
            dialogs: dialogs,
        }, DespawnOnExit(STATE))
    );

    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("DialogBottom"),
        UiTargetCamera(cam),
        BackgroundColor(Color::BLACK),
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(30.0),
            position_type: PositionType::Absolute,
            display: Display::Flex,
            bottom: Val::Px(0.0),
            padding: UiRect::left(Val::Px(10.0)),
            ..Default::default()
        },
        BottomDialog,
        TweenAnim::new(bottom_in),
        children![
            (
                Text::new(initial_text.clone()),
                TextColor::from(Srgba::new(0.144, 0.665, 0.992, 1.000)),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 33.0,
                    font_smoothing: FontSmoothing::None,
                    ..default()
                },
                LineHeight::RelativeToFont(0.7),
                DialogLabel,
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
                Text::new(initial_text),
                TextColor::from(Srgba::new(0.582, 0.095, 1.000, 1.000)),
                DialogShadowLabel,
                UiTransform {
                    translation: Val2::px(1.5, 1.5),
                    ..default()
                },
                TextFont {
                    font: assets.font.clone(),
                    font_size: 33.0,
                    font_smoothing: FontSmoothing::None,
                    ..default()
                },
                LineHeight::RelativeToFont(0.7),
                DialogLabel,
                Node {
                    top: Val::Px(20.0),
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                }
            )
        ],
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("DialogTop"),
        UiTargetCamera(cam),
        BackgroundColor(Color::BLACK),
        Node{
            width: Val::Percent(100.),
            height: Val::Percent(15.),
            position_type: PositionType::Absolute,
            display: Display::Flex,
            top: Val::Px(0.0),
            ..Default::default()
        },
        TopDialog,
        TweenAnim::new(top_in),
    ));
    cmd.spawn((
        UiTargetCamera(cam),
        DespawnOnExit(STATE),
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            ..Default::default()
        },
        TweenAnim::new(main_in),
        BgDialog,
        Name::new("DialogMain"),
        ZIndex(-3),
        
        children![
            (
            BackgroundGradient::from(LinearGradient {
                color_space: InterpolationColorSpace::Oklaba,
                stops: vec![
                    ColorStop::new(Color::srgba_u8(32, 0, 255, 255), percent(12.)),
                    ColorStop::new(Color::srgba_u8(200, 10, 40, 255), percent(100.)),
                ],
                ..default()
            }),
            BorderColor{left: Color::WHITE, ..default()},
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(-30.0),
                width: Val::Percent(45.0),
                height: Val::Percent(120.0),
                border: UiRect::left(Val::Px(1.0)),
                display: Display::Flex,
                // position: Position::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            UiTransform {
                rotation: Rot2 { cos: 1.33, sin: -0.21 },
                ..default()
            }),
        ]
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("DialogCharacter"),
        ZIndex(5),
        TweenAnim::new(char_in),
        ImageNode {
            image: speaker.clone(),
            ..Default::default()
        },
        DialogRot,
        DialogHead,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(40.0),
            top: Val::Percent(40.0),
            ..default()
        },
        UiTransform {
            scale: Vec2::splat(2.5),
            ..default()
        }
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("DialogCharacterShadow"),
        TweenAnim::new(shadow_in),
        ImageNode {
            image: speaker,
            color: Color::linear_rgba(0.0, 0.0, 0.0, 0.5),
            ..Default::default()
        },
        DialogRot,
        DialogHeadShadow,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(40.0),
            top: Val::Percent(40.0),
            ..default()
        },
        ZIndex(4),
        UiTransform {
            scale: Vec2::splat(2.5),
            ..default()
        }
    ));
}

#[derive(Component)]
pub struct PrevHead;

pub fn tick_dialog(
    mut state: Query<(Entity, &mut DialogState)>,
    mut anim: Query<(&ImageNode, &mut UiTransform), (With<DialogRot>, Without<DialogShadowLabel>, Without<PrevHead>)>,
    mut shadow_anim : Query<&mut UiTransform, (With<DialogShadowLabel>, Without<DialogRot>)>,
    mut texts: Query<&mut Text, With<DialogLabel>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands,
    assets: Res<MiamiAssets>,

    disabled_q: Query<Entity, With<PlayerDisabled>>,
    main_q: Query<Entity, (With<BgDialog>, Without<PrevHead>)>,
    top_q: Query<Entity, (With<TopDialog>, Without<PrevHead>)>,
    bottom_q: Query<Entity, (With<BottomDialog>, Without<PrevHead>)>,
    char_q: Query<Entity, (With<DialogHead>, Without<PrevHead>)>,
    char_s_q: Query<Entity, (With<DialogHeadShadow>, Without<PrevHead>)>,
){
    for (_, mut t) in anim.iter_mut() {
        t.rotation = Rot2::radians((time.elapsed_secs() * 2.0).sin() * 0.15);
    }
    for mut t in shadow_anim.iter_mut() {
        let v = Vec2::ONE * (time.elapsed_secs() * 3.0).cos() * 0.3 + 1.5;
        t.translation = Val2::px(v.x, v.y);
    }
    let Some((e, mut s)) = state.iter_mut().next() else {return;};
    if keys.just_pressed(KeyCode::Space) {
        if s.state >= s.dialogs.len() {return;}
        s.state = s.state + 1;
        if s.state >= s.dialogs.len() {
            for e in disabled_q.iter() {
                cmd.entity(e).remove::<PlayerDisabled>();
            }
            for e in main_q.iter() {
                let main_out = Tween::new(
                    EaseFunction::SineIn,
                    Duration::from_secs_f32(0.3),
                    UiTransformTranslationPxLens {
                        start: vec2(100., -10.),
                        end: vec2(400., -10.),
                    }
                );
                cmd.entity(e).insert(TweenAnim::new(main_out));
            }
            for e in top_q.iter() {
                let top_out = Tween::new(
                    EaseFunction::SineIn,
                    Duration::from_secs_f32(0.3),
                    UiTransformTranslationPxLens {
                        start: vec2(0., 0.),
                        end: vec2(0., -200.),
                    }
                );
                cmd.entity(e).insert(TweenAnim::new(top_out));
            }
            for e in bottom_q.iter() {
                let bottom_out = Tween::new(
                EaseFunction::SineIn,
                Duration::from_secs_f32(0.3),
                UiTransformTranslationPxLens {
                        start: vec2(0., 0.),
                        end: vec2(0., 200.),
                    }
                );
                cmd.entity(e).insert(TweenAnim::new(bottom_out));
            }

            for e in char_q.iter() {
                let char_out = Tween::new(
                    EaseFunction::SineIn,
                    Duration::from_secs_f32(0.3),
                    UiTransformTranslationPxLens {
                        start: vec2(-20., -25.),
                        end: vec2(300., -25.),
                    }
                );
                cmd.entity(e).insert(TweenAnim::new(char_out));
            }

            for e in char_s_q.iter() {
                let shadow_out = Tween::new(
                    EaseFunction::SineIn,
                    Duration::from_secs_f32(0.3),
                    UiTransformTranslationPxLens {
                        start: vec2(-12., -18.),
                        end: vec2(300., -18.),
                    }
                );
                cmd.entity(e).insert(TweenAnim::new(shadow_out));
            }

            cmd.entity(e).despawn();
        } else {
            for mut text in texts.iter_mut() {
                text.0 = s.dialogs[s.state].0.clone();
            }
            for (image, _) in anim.iter() {
                let next_image = s.dialogs[s.state].1.to_asset(&assets);
                if next_image != image.image {
                    let char_in = Tween::new(
                        EaseFunction::SineOut,
                        Duration::from_secs_f32(0.3),
                        UiTransformTranslationPxLens {
                            start: vec2(300., -25.),
                            end: vec2(-20., -25.),
                        }
                    );
                    let shadow_in = Tween::new(
                        EaseFunction::SineOut,
                        Duration::from_secs_f32(0.3),
                        UiTransformTranslationPxLens {
                            start: vec2(300., -18.),
                            end: vec2(-12., -18.),
                        }
                    );
                    for e in char_q.iter() {
                        let char_out = Tween::new(
                            EaseFunction::SineIn,
                            Duration::from_secs_f32(0.3),
                            UiTransformTranslationPxLens {
                                start: vec2(-20., -25.),
                                end: vec2(300., -25.),
                            }
                        );
                        cmd.entity(e).insert((TweenAnim::new(char_out), PrevHead));
                    }

                    for e in char_s_q.iter() {
                        let shadow_out = Tween::new(
                            EaseFunction::SineIn,
                            Duration::from_secs_f32(0.3),
                            UiTransformTranslationPxLens {
                                start: vec2(-12., -18.),
                                end: vec2(300., -18.),
                            }
                        );
                        cmd.entity(e).insert((TweenAnim::new(shadow_out), PrevHead));
                    }
                    cmd.spawn((
                        DespawnOnExit(STATE),
                        Name::new("DialogCharacter"),
                        ZIndex(5),
                        TweenAnim::new(char_in),
                        ImageNode {
                            image: next_image.clone(),
                            ..Default::default()
                        },
                        DialogRot,
                        DialogHead,
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Px(40.0),
                            top: Val::Percent(40.0),
                            ..default()
                        },
                        UiTransform {
                            scale: Vec2::splat(2.5),
                            ..default()
                        }
                    ));
                    cmd.spawn((
                        DespawnOnExit(STATE),
                        Name::new("DialogCharacterShadow"),
                        TweenAnim::new(shadow_in),
                        ImageNode {
                            image: next_image,
                            color: Color::linear_rgba(0.0, 0.0, 0.0, 0.5),
                            ..Default::default()
                        },
                        DialogRot,
                        DialogHeadShadow,
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Px(40.0),
                            top: Val::Percent(40.0),
                            ..default()
                        },
                        ZIndex(4),
                        UiTransform {
                            scale: Vec2::splat(2.5),
                            ..default()
                        }
                    ));
                }
                
                // image.image = next_
            }
        }
    }
}