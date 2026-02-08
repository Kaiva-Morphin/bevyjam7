use crate::{games::novel::plugin::{Actor, Background}, prelude::*};

#[macro_export]
macro_rules! actors {
    (
        $(
            $name:literal : $variant:ident => $path:literal
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
                pub fn name(&self) -> &'static str {
                    match self {
                        $(
                            Actor::$variant => $name,
                        )*
                    }
                }

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

#[derive(Default)]
pub struct ActorAppearance {
    pub actor: Actor,
    pub flip_x: bool,
    pub pos: Vec3,
}

#[derive(Default)]
pub struct NovelStage {
    pub actors: Vec<ActorAppearance>,
    pub bg: Background,
    pub text: String,
}

#[macro_export]
macro_rules! stages {
    (
        $(
            $bg:ident {
                $(
                    $actor:ident $((
                        $( $field:ident = $value:expr ),* $(,)?
                    ))?
                ),* $(,)?
                =>
                $text:literal
            }
        ),* $(,)?
    ) => {
        vec![
            $(
                NovelStage {
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
                }
            ),*
        ]
    };
}



