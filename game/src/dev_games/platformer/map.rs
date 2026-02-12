use crate::{dev_games::{platformer::plugin::{Player, PlayerSwitchSensor, platformer_player_white_layer, platformer_player_yellow_layer, player_color_white, player_color_yellow}}, prelude::*};


#[derive(Component, Reflect, Default, Eq, PartialEq)]
#[reflect(Component, Default)]
pub enum PlatformerSwitchableLayer {
    #[default]
    Yellow,
    White
}


#[derive(Component)]
pub struct OnYellowLayer;

pub fn swap(
    keys: Res<ButtonInput<KeyCode>>,
    players: Query<(Entity, &mut Sprite, &PlayerSwitchSensor, Option<&OnYellowLayer>), With<Player>>,
    mut layers: Query<(&mut Visibility, &PlatformerSwitchableLayer)>,
    mut casters: Query<(&mut ShapeCaster, &ShapeHits)>,
    mut cmd: Commands,
) {
    for (player, mut sprite, switch, o) in players {
        let Ok((mut map_shapecaster, hits)) = casters.get_mut(switch.0) else {continue;};
        if !keys.just_pressed(KeyCode::KeyQ){continue;};
        for hit in hits {
            if hit.distance < 0.1 {
                // todo!: sound
                return;
            }
        }
        let on_yellow = o.is_some();
        let layer = if on_yellow {
            platformer_player_yellow_layer()
        } else {
            platformer_player_white_layer()
        };
        let opposite_layer = if on_yellow {
            platformer_player_white_layer()
        } else {
            platformer_player_yellow_layer()
        };
        if on_yellow {
            sprite.color = player_color_yellow();
        } else {
            sprite.color = player_color_white();
        }
        map_shapecaster.query_filter = SpatialQueryFilter::from_mask(opposite_layer.filters & 0b110000);
        let Ok((mut shapecaster, _)) = casters.get_mut(player) else {continue;};
        shapecaster.query_filter = SpatialQueryFilter::from_mask(layer.filters & 0b110001);
        cmd.entity(player)
            .insert(layer);

        let show = Visibility::Visible;
        let hide = Visibility::Hidden;
        for (mut vis, layer) in layers.iter_mut() {
            if (layer == &PlatformerSwitchableLayer::Yellow) == on_yellow {
                *vis = show;
            } else {
                *vis = hide;
            }
        }
        if on_yellow {
            cmd.entity(player).remove::<OnYellowLayer>();
        } else {
            cmd.entity(player).insert(OnYellowLayer);
        }
    }
}

