use bevy::prelude::*;

#[derive(Component)]
pub struct PowerGaugeText;

#[derive(Resource)]
pub struct Nova {
    pub power: f32,
}

impl Default for Nova {
    fn default() -> Nova {
        Nova { power: 0.0 }
    }
}

pub struct NovaPlugin;

impl Plugin for NovaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Nova>()
            .add_startup_system(init_power_gauge)
            .add_system(update_power_gauge)
            .add_system(update_power_value);
    }
}

fn init_power_gauge(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                margin: UiRect {
                    top: Val::Percent(10.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: asset_server
                            .load("fonts/Caskaydia Cove Nerd Font Complete Mono Bold.otf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style { ..default() }),
                PowerGaugeText,
            ));
        });
}

fn update_power_value(keyboard_input: Res<Input<KeyCode>>, mut nova: ResMut<Nova>) {
    if keyboard_input.pressed(KeyCode::Space) {
        nova.power += 0.1;
        if nova.power > 10.0 {
            nova.power = 10.0;
        }
    }
    if keyboard_input.just_released(KeyCode::Space) {
        nova.power = 0.0;
    }
}

fn update_power_gauge(mut query: Query<&mut Text, With<PowerGaugeText>>, nova: Res<Nova>) {
    if nova.is_changed() {
        if let Ok(mut text) = query.get_single_mut() {
            let power = nova.power.round() as usize;
            text.sections[0].value = "*".repeat(power);
        }
    }
}
