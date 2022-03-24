use bevy::prelude::*;

use crate::AppState;

// Hardcoded for now for predetermined screen size
const OFFSET_X: f32 = -400.;
const OFFSET_Y: f32 = -300.;

pub struct TokenPlugin;

#[derive(Default, Component)]
struct Token;
#[derive(Component)]
pub struct TokenSlot;
#[derive(Default, Component)]
struct Draggable;
#[derive(Component)]
struct Dragged;
#[derive(Default, Component)]
struct SideLength {
    x_len: f32,
    y_len: f32,
}

#[derive(Default, Bundle)]
struct TokenBundle {
    token: Token,
    draggable: Draggable,
    sides: SideLength,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl Plugin for TokenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Last, spawn_tokens)
           .add_system_set(
               SystemSet::on_update(AppState::Game).with_system(up_draggable)
                                                   .with_system(down_draggable)
                                                   .with_system(drag_token)
           );
        
    }
}

// Spawns all tokens in the appropriate UI TokenSlots
fn spawn_tokens(mut cmds: Commands, asset_server: Res<AssetServer>,
                query: Query<(&GlobalTransform, &Node), Added<TokenSlot>>) {
    for (slot_gt, slot_node) in query.iter() {
        let token_t = slot_gt.translation + Vec3::new(OFFSET_X, OFFSET_Y, 0.);

        cmds.spawn_bundle(TokenBundle {
            token: Token,
            draggable: Draggable,
            sides: SideLength {
                x_len: slot_node.size.x,
                y_len: slot_node.size.y,
            },
            transform: Transform {
                translation: token_t,
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(slot_node.size.clone()),
                    ..Default::default()
                },
                texture: asset_server.load("icon.png"),
                ..Default::default()
            });
        });
    }
}

// Sets a Draggable element to Dragged if it was clicked
fn up_draggable(btn_press: Res<Input<MouseButton>>,
                mut draggable_query: Query<(Entity, &SideLength, &mut Transform), 
                                        With<Draggable>>,
                dragged_query: Query<With<Dragged>>,
                mut cmds: Commands,
                windows: Res<Windows>,
) {
    if btn_press.just_pressed(MouseButton::Left) && dragged_query.iter().len() == 0 {
        // Find current cursor coords
        let window = windows.get_primary().unwrap();
        let cursor_coords = if let Some(cursor) = window.cursor_position() {
            cursor + Vec2::new(OFFSET_X, OFFSET_Y)
        } else {
            return
        };
        
        // Check to see if click was on any Draggable
        for (entity_id, bounds, mut drag_t) in draggable_query.iter_mut() {
            if in_bounds(&cursor_coords, bounds, &drag_t.translation) {
                cmds.entity(entity_id).insert(Dragged);
                drag_t.translation.z += 1.; // So Dragged above other Draggables
                break; // To ensure only one token dragged at a time
            }
        }
    }
}

// Put down a Dragged element on a left click
fn down_draggable(btn_press: Res<Input<MouseButton>>,
                  dragged_query: Query<Entity, With<Dragged>>,
                  mut cmds: Commands,
) {
    if btn_press.just_pressed(MouseButton::Left) {
        // Stop the entity from being dragged
        for entity_id in dragged_query.iter() {
            cmds.entity(entity_id).remove::<Dragged>();
        }
    }
}

// Moves a token being dragged by the cursor by setting its Translation to
// match that of the cursor
fn drag_token(mut cursor_move: EventReader<CursorMoved>,
              mut dragged_query: Query<&mut Transform, With<Dragged>>,
) {
    for mut drag_t in dragged_query.iter_mut() {
        for movement in cursor_move.iter() {
            let new_t = movement.position + Vec2::new(OFFSET_X, OFFSET_Y);
            drag_t.translation.x = new_t.x;
            drag_t.translation.y = new_t.y;
        }
    }
}

// Determines if the click location was within bounds
fn in_bounds(cursor_pos: &Vec2, sides: &SideLength, center: &Vec3) -> bool {
    let min_x = center.x - (sides.x_len / 2.);
    let max_x = center.x + (sides.x_len / 2.);
    let min_y = center.y - (sides.y_len / 2.);
    let max_y = center.y + (sides.y_len / 2.);

    cursor_pos.x < max_x && cursor_pos.x > min_x &&
    cursor_pos.y < max_y && cursor_pos.y > min_y
}

