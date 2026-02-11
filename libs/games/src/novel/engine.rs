use crate::{novel::plugin::{Actor, Background, NovelSoundEffect, NovelMusic}, prelude::*};

#[macro_export]
macro_rules! actors {
    (
        $(
            $variant:ident => $path:literal
        ),* $(,)?
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Default)]
            pub enum Actor {
                #[default]
                $(
                    $variant,
                )*
            }

            impl Actor {
                pub fn get_asset(&self, assets: &ActorsAssets) -> Handle<Image> {
                    match self {
                        $(
                            Actor::$variant => assets.[<$variant:snake>].clone(),
                        )*
                    }
                }
            }

            #[derive(AssetCollection, Resource)]
            pub struct ActorsAssets {
                $(
                    #[asset(path = $path)]
                    pub [<$variant:snake>]: Handle<Image>,
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! backgrounds {
    (
        $(
            $bg: ident => $path:literal
        ),* $(,)?
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Default)]
            pub enum Background {
                #[default]
                $(
                    $bg,
                )*
            }
            impl Background {
                pub fn get_asset(&self, assets: &BackgroundsAssets) -> Handle<Image> {
                    match self {
                        $(
                            Background::$bg => assets.[<$bg:snake>].clone(),
                        )*
                    }
                }
            }
            #[derive(AssetCollection, Resource)]
            pub struct BackgroundsAssets {
                $(
                    #[asset(path = $path)]
                    pub [<$bg:snake>]: Handle<Image>,
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! sound_effects {
    (
        $(
            $se: ident => $path:literal
        ),* $(,)?
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Default)]
            pub enum NovelSoundEffect {
                #[default]
                $(
                    $se,
                )*
            }
            impl NovelSoundEffect {
                pub fn get_asset(&self, assets: &NovelSoundEffectsAssets) -> Handle<AudioSource> {
                    match self {
                        $(
                            NovelSoundEffect::$se => assets.[<$se:snake>].clone(),
                        )*
                    }
                }
            }
            #[derive(AssetCollection, Resource)]
            pub struct NovelSoundEffectsAssets {
                $(
                    #[asset(path = $path)]
                    pub [<$se:snake>]: Handle<AudioSource>,
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! novel_music {
    (
        $(
            $music: ident => $path:literal
        ),* $(,)?
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Default, PartialEq, Eq)]
            pub enum NovelMusic {
                #[default]
                $(
                    $music,
                )*
            }
            impl NovelMusic {
                pub fn get_asset(&self, assets: &NovelMusicAssets) -> Handle<AudioSource> {
                    match self {
                        $(
                            NovelMusic::$music => assets.[<$music:snake>].clone(),
                        )*
                    }
                }
            }
            #[derive(AssetCollection, Resource)]
            pub struct NovelMusicAssets {
                $(
                    #[asset(path = $path)]
                    pub [<$music:snake>]: Handle<AudioSource>,
                )*
            }
        }
    };
}

#[derive(Default)]
pub struct ActorAppearance {
    pub actor: Actor,
    pub flip_x: bool,
    pub transform: Transform,
}

#[derive(Default)]
pub struct NovelStage {
    pub actors: Vec<ActorAppearance>,
    pub sfx: Option<NovelSoundEffect>,
    pub music: NovelMusic,
    pub speaker: String,
    pub bg: Background,
    pub text: String,
}

#[macro_export]
macro_rules! stages {
    (
        $(
            $bg:ident $music:ident  {
                $(
                    $actor:ident $((
                        $( $field:ident = $value:expr ),* $(,)?
                    ))?
                ),* $(,)?
                =>
                $(($speaker:literal))?
                $text:literal
                $(($sfx:ident))?
            }
        ),* $(,)?
    ) => {
        vec![
            $(
                NovelStage {
                    music: NovelMusic::$music,
                    $(speaker: $speaker.to_string(),)?
                    $(sfx: Some(NovelSoundEffect::$sfx),)?
                    actors: vec![
                        $(
                            {
                                #[allow(unused_mut)]
                                let mut appearance = ActorAppearance {
                                    actor: Actor::$actor,
                                    ..Default::default()
                                };
                                $(
                                    $(
                                        appearance.$field = $value;
                                    )*
                                )?
                                appearance
                            }
                        ),*
                    ],
                    bg: Background::$bg,
                    text: $text.to_string(),
                    ..Default::default()
                }
            ),*
        ]
    };
}



