use std::collections::{BTreeMap, HashMap, HashSet};

use bevy::{
    camera::visibility::RenderLayers,
    diagnostic::{DiagnosticPath, DiagnosticsStore},
    prelude::*,
    text::{FontSmoothing, LineHeight},
};

// const TEXT_SIZE : usize = 16;
// const TEXT_MARGIN_TOP : f32 = -2.0;
// const TEXT_MARGIN_BOTTOM : f32 = -2.0;

#[macro_export]
macro_rules! overlay_text {
    ($events:ident ; $anchor:ident ; $layer:expr ; $key:ident : $( $text:expr ,( $( $arg:tt )* ) );+ $(;)? ) => {
        $events.trigger(DebugOverlayEvent::Set {
            key: stringify!($key).to_string(),
            val: $crate::debug_overlay::DebugRecord {
                record_type: $crate::debug_overlay::DebugRecordType::Text {
                    text: vec![
                        $(
                            $crate::overlay_text!(@parse_text $text ,( $( $arg )* )),
                        )+
                    ],
                },
                anchor: $crate::debug_overlay::OverlayAnchor::$anchor,
                layer: $layer,
            }
        });
    };

    ($events:ident ; $anchor:ident ; $key:ident : $( $text:expr ,( $( $arg:tt )* ) );+ $(;)? ) => {
        overlay_text!($events ; $anchor ; 0 ; $key : $( $text ,( $( $arg )* ) );+ );
    };

    ($events:ident ; $key:ident : $( $text:expr ,( $( $arg:tt )* ) );+ $(;)? ) => {
        overlay_text!($events ; $crate::debug_overlay::OverlayAnchor::BottomRight ; 0 ; $key : $( $text ,( $( $arg )* ) );+ );
    };

    (@parse_text $text:expr ,( $r:literal , $g:literal , $b:literal , $a:literal )) => {
        (bevy::prelude::Color::srgba_u8($r, $g, $b, $a), $text)
    };
    (@parse_text $text:expr ,( $r:literal , $g:literal , $b:literal )) => {
        (bevy::prelude::Color::srgba_u8($r, $g, $b, 255), $text)
    };
    (@parse_text $text:expr ,( $c:literal )) => {
        (bevy::prelude::Color::srgba_u8($c, $c, $c, 255), $text)
    };
    (@parse_text $text:expr) => {
        (bevy::prelude::Color::srgba_u8(255, 255, 255, 255), $text)
    };
}

pub struct DebugOverlayPlugin {
    pub enabled: bool,
    pub suppress_default: bool,
}

impl DebugOverlayPlugin {
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            suppress_default: false,
        }
    }
    pub fn suppress_default(mut self) -> Self {
        self.suppress_default = true;
        self
    }
}

impl Default for DebugOverlayPlugin {
    fn default() -> Self {
        Self {
            enabled: false,
            suppress_default: false,
        }
    }
}

impl Plugin for DebugOverlayPlugin {
    fn build(&self, _app: &mut App) {
        {
            _app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
                .insert_resource(DebugOverlay::enabled(self.enabled))
                .add_systems(PreStartup, init);
            if self.suppress_default {
                _app.add_systems(PreUpdate, resize_overlay);
                _app.add_observer(debug_overlay_tick);
            } else {
                _app.add_systems(PreStartup, init_default_sustems);
                _app.add_systems(PreUpdate, (resize_overlay, default_sustem_tick));
                _app.add_observer(debug_overlay_tick);
            }
            // _app.add_event::<DebugOverlayEvent>();
        }
    }
}

#[derive(Event)]
pub enum DebugOverlayEvent {
    Set { key: String, val: DebugRecord },
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum OverlayAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    TopCenter,
    BottomCenter,
    LeftCenter,
    RightCenter,

    // Follow entity on world
    Dynamic(Entity),
    // Fills entire screen
    Fill,
}

impl OverlayAnchor {
    pub fn is_bottom(&self) -> bool {
        matches!(
            self,
            OverlayAnchor::BottomLeft | OverlayAnchor::BottomRight | OverlayAnchor::BottomCenter
        )
    }
    pub fn is_right(&self) -> bool {
        matches!(
            self,
            OverlayAnchor::TopRight | OverlayAnchor::BottomRight | OverlayAnchor::RightCenter
        )
    }
    pub fn is_left(&self) -> bool {
        matches!(
            self,
            OverlayAnchor::TopLeft | OverlayAnchor::BottomLeft | OverlayAnchor::LeftCenter
        )
    }
    pub fn is_top(&self) -> bool {
        matches!(
            self,
            OverlayAnchor::TopLeft | OverlayAnchor::TopRight | OverlayAnchor::TopCenter
        )
    }

    pub fn justify_content(&self) -> JustifyContent {
        if self.is_bottom() {
            JustifyContent::End
        } else if self.is_center_vertical() {
            JustifyContent::Center
        } else {
            JustifyContent::Start
        }
    }

    pub fn flex_direction(&self) -> FlexDirection {
        if self.is_bottom() {
            FlexDirection::ColumnReverse
        } else {
            FlexDirection::Column
        }
    }

    pub fn is_center_horizontal(&self) -> bool {
        matches!(
            self,
            OverlayAnchor::Center | OverlayAnchor::TopCenter | OverlayAnchor::BottomCenter
        )
    }

    pub fn is_center_vertical(&self) -> bool {
        matches!(
            self,
            OverlayAnchor::Center | OverlayAnchor::LeftCenter | OverlayAnchor::RightCenter
        )
    }

    fn align_items(&self) -> AlignItems {
        if self.is_right() {
            AlignItems::End
        } else if self.is_center_horizontal() {
            AlignItems::Center
        } else {
            AlignItems::Start
        }
    }
    fn align_self(&self) -> AlignSelf {
        if self.is_bottom() {
            AlignSelf::End
        } else if self.is_center_vertical() {
            AlignSelf::Center
        } else {
            AlignSelf::Start
        }
    }

    pub fn to_node(&self) -> Node {
        let base = Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: self.justify_content(),
            flex_direction: self.flex_direction(),
            align_items: self.align_items(),
            align_self: self.align_self(),
            position_type: PositionType::Absolute,
            ..default()
        };

        base
    }
}

pub fn default_sustem_tick(
    // mut events: EventWriter<DebugOverlayEvent>,
    mut cmd: Commands,
    diagnostics_store: Option<Res<DiagnosticsStore>>,
) {
    let Some(diagnostics_store) = diagnostics_store else {
        error_once!(
            "Can't find DiagnosticsStore! make sure ure are add FrameTimeDiagnosticsPlugin!"
        );
        return;
    };
    let fps = diagnostics_store.get(&DiagnosticPath::const_new("fps"));
    if let Some(fps) = fps {
        let smoothed = fps.smoothed().unwrap_or(0.);
        let avg = fps.average().unwrap_or(0.);
        let get_fps_color = |x: f64| -> Color {
            if x > 59. {
                Color::srgba(0., 1., 0., 1.)
            } else if x > 49. {
                Color::srgba(0.2, 0.8, 0., 1.)
            } else if x > 39. {
                Color::srgba(0.4, 0.6, 0., 1.)
            } else if x > 10. {
                Color::srgba(0.6, 0.4, 0., 1.)
            } else if x > 5. {
                Color::srgba(0.8, 0.2, 0., 1.)
            } else {
                Color::srgba(0.1, 0., 0., 1.)
            }
        };

        cmd.trigger(DebugOverlayEvent::Set {
            key: "FPS_AVG".to_string(),
            val: DebugRecord {
                record_type: DebugRecordType::Text {
                    text: vec![
                        (Color::srgba(0.5, 0.5, 0.5, 1.), format!("avg. ")),
                        (get_fps_color(avg), format!("{:.1}", avg)),
                        (Color::srgba(0.8, 0.8, 0.8, 1.), format!(" fps")),
                    ],
                },
                anchor: OverlayAnchor::BottomRight,
                layer: 0,
            },
        });
        cmd.trigger(DebugOverlayEvent::Set {
            key: "FPS_SM".to_string(),
            val: DebugRecord {
                record_type: DebugRecordType::Text {
                    text: vec![
                        (get_fps_color(smoothed), format!("{:.1}", smoothed)),
                        (Color::srgba(0.8, 0.8, 0.8, 1.), format!(" fps")),
                    ],
                },
                anchor: OverlayAnchor::BottomRight,
                layer: 0,
            },
        });
    }
}

pub fn resize_overlay(
    inputs: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands,
    mut overlay: ResMut<DebugOverlay>,
    mut r: Single<&mut Node, With<DebugOverlayRoot>>,
    mut action_performed: Local<bool>,
) {
    if inputs.just_pressed(KeyCode::F3) {
        if !*action_performed {
            overlay.visible = !overlay.visible;
            match overlay.visible {
                true => r.display = Display::Flex,
                false => r.display = Display::None,
            }
        }
        *action_performed = false;
    }
    if inputs.pressed(KeyCode::ControlLeft) && inputs.pressed(KeyCode::ShiftLeft) {
        let d =
            inputs.just_pressed(KeyCode::Equal) as i32 - inputs.just_pressed(KeyCode::Minus) as i32;
        overlay.text_font.font_size += 1.0 * d as f32;
        cmd.trigger(DebugOverlayEvent::Set {
            key: "OVERLAY_SIZE".to_string(),
            val: DebugRecord {
                record_type: DebugRecordType::Text {
                    text: vec![
                        (Color::srgba(0.5, 0.5, 0.5, 1.), format!("Font size: ")),
                        (
                            Color::srgba(0.5, 0.5, 0.5, 1.),
                            overlay.text_font.font_size.to_string(),
                        ),
                    ],
                },
                anchor: OverlayAnchor::BottomRight,
                layer: 0,
            },
        });
    }
}

pub fn debug_overlay_tick(
    event: On<DebugOverlayEvent>,
    mut overlay: ResMut<DebugOverlay>,
    mut cmd: Commands,
) {
    if !overlay.visible {
        return;
    }
    let e = &*event;
    match e {
        DebugOverlayEvent::Set { key, val } => {
            overlay.set(&mut cmd, &key, &val);
        } // _ => {unimplemented!()}
    }
}

#[derive(Clone)]
pub enum DebugRecordType {
    Text {
        text: Vec<(Color, String)>,
    },
    Graph {
        data: Vec<f32>,
        size: usize,
        last_index: usize,
        max: f32,
        min: f32,
        marks: Vec<(f32, String)>,
    },
    Image {
        handle: Handle<Image>,
        size: Option<Vec2>,
        flip_x: bool,
        flip_y: bool,
        rect: Option<Rect>,
        label: Option<String>,
    },
}

impl DebugRecordType {
    fn to_node_bundle(&self, cmd: &mut Commands, font: &TextFont) -> (Entity, Vec<Entity>) {
        match self {
            DebugRecordType::Text { text } => {
                let mut children = vec![];
                let e = cmd
                    .spawn((font.clone(), Text::default()))
                    .with_children(|cmd| {
                        for (color, text) in text {
                            let c = cmd
                                .spawn((
                                    font.clone(),
                                    TextSpan::new(text.clone()),
                                    TextColor::from(*color),
                                    LineHeight::RelativeToFont(0.8),
                                ))
                                .id();
                            children.push(c);
                        }
                    })
                    .id();
                (e, children)
            }
            _ => unimplemented!(),
        }
    }
    fn update(
        &mut self,
        cmd: &mut Commands,
        record: &Self,
        entity: Entity,
        children: &mut Vec<Entity>,
        font: &TextFont,
    ) {
        match self {
            DebugRecordType::Text { text: _stored } => {
                // TODO: USE ANOTHER METHOD; MAYBE JUST REPLACE
                let DebugRecordType::Text { text: to_add } = record else {
                    warn!("Wrong record type passed!");
                    return;
                };

                let mut to_add = to_add.into_iter();
                let mut nc = vec![];
                for e in children.iter() {
                    if let Some((c, t)) = to_add.next() {
                        cmd.entity(*e).insert((
                            font.clone(),
                            TextSpan::new(t.clone()),
                            TextColor::from(c.clone()),
                        ));
                        nc.push(*e);
                    } else {
                        cmd.entity(*e).despawn();
                    }
                }
                *children = nc;

                while let Some((c, t)) = to_add.next() {
                    let e = cmd
                        .spawn((
                            font.clone(),
                            TextSpan::new(t.clone()),
                            TextColor::from(c.clone()),
                        ))
                        .id();
                    cmd.entity(entity).add_child(e);
                    children.push(e);
                }

                // // let diff = text.len() as i32 - new_text.len() as i32;
                // // if diff != 0 {warn!("Text length diff: {}", diff);return};

                // let mut to_add = vec![];
                // for (color, text) in new_text.iter() {
                //     to_add.push(cmd.spawn((
                //             TextFont { font: font.clone(), font_smoothing: FontSmoothing::None, ..default() },
                //         TextSpan::new(text.clone()),
                //         TextColor::from(*color),
                //     )).id());
                // }
                // cmd.entity(entity).replace_children(&to_add);

                // *text = new_text.clone();
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone)]
pub struct DebugRecord {
    pub record_type: DebugRecordType,
    pub anchor: OverlayAnchor,
    pub layer: i8,
}

pub struct StoredDebugRecord {
    record: DebugRecord,
    entity: Entity,
    children: Vec<Entity>,
}

#[derive(Component)]
pub struct DebugOverlayRoot;

pub fn init(mut cmd: Commands, asset_server: Res<AssetServer>, mut overlay: ResMut<DebugOverlay>) {
    overlay.text_font = TextFont {
        font: asset_server.load("fonts/kaivs_minegram_v1.ttf"),
        font_smoothing: FontSmoothing::None,
        font_size: 32.0,
        ..default()
    };

    overlay.root = cmd
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Absolute,
                display: if overlay.visible {
                    Display::Flex
                } else {
                    Display::None
                },
                ..default()
            },
            RenderLayers::layer(0),
            DebugOverlayRoot,
            Name::new("DebugRoot"),
        ))
        .id();
}

pub fn init_default_sustems(mut cmd: Commands) {
    cmd.trigger(DebugOverlayEvent::Set {
        key: "PKGINFO".to_string(),
        val: DebugRecord {
            record_type: DebugRecordType::Text {
                text: vec![(
                    Color::srgba(0.5, 0.5, 0.5, 1.),
                    format!("{} v{}", NAME, VERSION),
                )],
            },
            anchor: OverlayAnchor::BottomRight,
            layer: 0,
        },
    });
    cmd.trigger(DebugOverlayEvent::Set {
        key: "FPS_AVG".to_string(),
        val: DebugRecord {
            record_type: DebugRecordType::Text {
                text: vec![(Color::srgba(0.5, 0.5, 0.5, 1.), format!("avg. 0 fps"))],
            },
            anchor: OverlayAnchor::BottomRight,
            layer: 0,
        },
    });
    cmd.trigger(DebugOverlayEvent::Set {
        key: "FPS_SM".to_string(),
        val: DebugRecord {
            record_type: DebugRecordType::Text {
                text: vec![(Color::srgba(0.5, 0.5, 0.5, 1.), format!("0 fps"))],
            },
            anchor: OverlayAnchor::BottomRight,
            layer: 0,
        },
    });
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

pub struct LayerRecord {
    entity: Entity,
    anchors: HashMap<OverlayAnchor, Entity>,
}

impl LayerRecord {
    pub fn from_entity(e: Entity) -> Self {
        Self {
            entity: e,
            anchors: HashMap::new(),
        }
    }
}

#[derive(Resource)]
pub struct DebugOverlay {
    root: Entity,
    layers: BTreeMap<i8, LayerRecord>,
    visible: bool,
    records: HashMap<String, StoredDebugRecord>,
    _enabled_layers: HashSet<i8>,
    text_font: TextFont,
    bg_color: Color,
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self {
            root: Entity::PLACEHOLDER,
            text_font: Default::default(),
            layers: BTreeMap::new(),
            visible: false,
            records: HashMap::new(),
            _enabled_layers: HashSet::new(),
            bg_color: Color::srgba(0.1, 0.1, 0.1, 0.8),
        }
    }
}

impl DebugOverlay {
    fn enabled(enabled: bool) -> Self {
        Self {
            visible: enabled,
            ..default()
        }
    }
    fn layer_create_or_get<'a, 'b>(
        &'a mut self,
        cmd: &mut Commands,
        layer: i8,
    ) -> &'b mut LayerRecord
    where
        'a: 'b,
    {
        self.layers.entry(layer).or_insert_with(|| {
            let e = cmd
                .spawn((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    Name::new(format!("Layer {}", layer)),
                ))
                .id();
            cmd.entity(self.root).add_child(e);
            LayerRecord::from_entity(e)
        })
    }

    fn anchor_create_or_get(
        &mut self,
        cmd: &mut Commands,
        layer: i8,
        anchor: OverlayAnchor,
    ) -> &Entity {
        let layer = self.layer_create_or_get(cmd, layer);
        let overlay_anchor = anchor.clone();
        layer.anchors.entry(anchor).or_insert_with(|| {
            let e = cmd
                .spawn((
                    Name::new(format!("Anchor {overlay_anchor:?}")),
                    overlay_anchor.to_node(),
                ))
                .id();
            cmd.entity(layer.entity).add_child(e);
            e
        })
    }

    fn record_create_or_update(&mut self, cmd: &mut Commands, key: &str, record: &DebugRecord) {
        let anchor = *self.anchor_create_or_get(cmd, record.layer, record.anchor.clone());
        if let Some(stored_record) = self.records.get_mut(key) {
            stored_record.record.record_type.update(
                cmd,
                &record.record_type,
                stored_record.entity,
                &mut stored_record.children,
                &self.text_font,
            );
        } else {
            let (e, c) = record.record_type.to_node_bundle(cmd, &self.text_font);
            let e = cmd
                .entity(e)
                .insert(BackgroundColor(self.bg_color.clone()))
                .id();
            cmd.entity(anchor).add_child(e);
            let s = StoredDebugRecord {
                record: record.clone(),
                entity: e,
                children: c,
            };
            self.records.insert(key.to_owned(), s);
        };
    }

    fn set(&mut self, cmd: &mut Commands, key: &str, val: &DebugRecord) {
        self.record_create_or_update(cmd, key, val);
    }
}
