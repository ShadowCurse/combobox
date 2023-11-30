use bevy::prelude::*;

use crate::Score;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hud_setup);
        app.add_systems(Update, hud_update);
    }
}

#[derive(Component)]
struct UiScore;

fn hud_setup(asset_server: Res<AssetServer>, mut command: Commands) {
    let score_text_style = TextStyle {
        font: asset_server.load("fonts/monaco.ttf"),
        font_size: 20.0,
        color: Color::hex("faa307").unwrap(),
    };

    command
        .spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|builder| {
            // score
            builder
                .spawn(TextBundle {
                    text: Text::from_section("Score: ---", score_text_style)
                        .with_alignment(TextAlignment::Left),
                    ..default()
                })
                .insert(UiScore);
        });
}

fn hud_update(score: Res<Score>, mut ui_score: Query<&mut Text, With<UiScore>>) {
    let mut text = ui_score.single_mut();
    let str = format!("Score: {}", score.score);
    text.sections[0].value = str;
}
