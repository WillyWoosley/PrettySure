use bevy::prelude::*;

pub struct TokenPlugin;

#[derive(Component)]
pub struct Token;
#[derive(Component)]
pub struct TokenSlot;

impl Plugin for TokenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Last, spawn_tokens);
    }
}

fn spawn_tokens(mut cmds: Commands, asset_server: Res<AssetServer>,
                query: Query<(&GlobalTransform, &Node), Added<TokenSlot>>) {
    for (slot_gt, slot_node) in query.iter() {
        let token_t = slot_gt.translation.clone() + Vec3::new(-400., -300., 10.);

        cmds.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(slot_node.size.clone()),
                ..Default::default()
            },
            transform: Transform {
                translation: token_t,
                ..Default::default()
            },
            texture: asset_server.load("icon.png"),
            ..Default::default()
        }).insert(Token);
    }
}

