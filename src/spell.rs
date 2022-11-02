use crate::loading::ImageAssets;
use crate::GameState;
use bevy::{prelude::*, ui::FocusPolicy};

pub struct SpellPlugin;

#[derive(Component)]
pub struct Spell {
    pub is_active: bool,
}

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(create_spell))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_spell_click)
                    .with_system(handle_spell_reset),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Spawned)
                    .with_system(handle_spell_click)
                    .with_system(handle_spell_reset),
            );
    }
}

fn create_spell(mut commands: Commands, images: Res<ImageAssets>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(75.0), Val::Px(75.0)),
                position: UiRect {
                    left: Val::Percent(92.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("Spell"))
        .insert(Spell { is_active: false })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    image: images.spell_sprite.clone().into(),
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_scale(Vec3 {
                        x: 0.5,
                        y: 0.5,
                        z: 0.5,
                    }),
                    ..Default::default()
                })
                .insert(FocusPolicy::Pass);
        });
}

fn handle_spell_click(
    mut interaction_query: Query<(&mut Spell, &mut Children, &Interaction), Changed<Interaction>>,
    mut image_query: Query<&mut UiImage>,
    images: Res<ImageAssets>,
) {
    for (mut spell, children, interaction) in interaction_query.iter_mut() {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();

        match interaction {
            Interaction::Clicked => {
                spell.is_active = !spell.is_active;
                if spell.is_active {
                    image.0 = images.spell_selected_sprite.clone().into();
                } else {
                    image.0 = images.spell_sprite.clone().into();
                }
            }
            Interaction::Hovered | Interaction::None => {}
        }
    }
}

fn handle_spell_reset(
    mut spell_query: Query<(&Spell, &mut Children)>,
    mut image_query: Query<&mut UiImage>,
    images: Res<ImageAssets>,
) {
    if let Ok((spell, children)) = spell_query.get_single_mut() {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();

        if spell.is_active {
            image.0 = images.spell_selected_sprite.clone().into();
        } else {
            image.0 = images.spell_sprite.clone().into();
        }
    };
}
