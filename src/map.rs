// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::borrow::Borrow;

use gdnative::api::{Camera2D, TileMap};
use gdnative::prelude::*;

use crate::bullet::Bullet;
use crate::utils::node::NodeRef;
use crate::utils::preload::*;
use crate::utils::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Map {
    camera_node: NodeRef<Camera2D>,
    ground_node: NodeRef<TileMap>,
}

#[methods]
impl Map {
    fn new(_owner: TRef<Node2D>) -> Self {
        Map {
            camera_node: NodeRef::new("Player/Camera2D"),
            ground_node: NodeRef::new("Ground"),
        }
    }

    #[inline]
    pub fn set_camera_limits(&mut self) {
        let player_camera = self.camera_node.get_ref();
        let ground = self.ground_node.get_ref();

        let map_limits = ground.get_used_rect();
        let map_cellsize = ground.cell_size();

        player_camera.set_limit(0, (map_limits.min_x() * map_cellsize.x) as i64);
        player_camera.set_limit(2, (map_limits.max_x() * map_cellsize.x) as i64);
        player_camera.set_limit(1, (map_limits.min_y() * map_cellsize.y) as i64);
        player_camera.set_limit(3, (map_limits.max_y() * map_cellsize.y) as i64);
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        self.camera_node.get_from(owner);
        self.ground_node.get_from(owner);
        self.set_camera_limits();

        Input::godot_singleton().set_custom_mouse_cursor(
            preload::<Texture>("res://ui/crossair_black.png"),
            Input::CURSOR_ARROW,
            Vector2::new(16.0, 16.0),
        )
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Tank_shoot(
        &self,
        owner: TRef<Node2D>,
        bullet_scene: Ref<PackedScene, Shared>,
        position: Vector2,
        direction: Vector2,
    ) {
        let bullet_node = instance_scene(bullet_scene, PackedScene::GEN_EDIT_STATE_DISABLED);
        owner.add_child(bullet_node, false);

        Bullet::instance_from(bullet_node)
            .borrow()
            .map_mut(|bullet, node| bullet.start(node, position, direction))
            .expect("Failed to start bullet");
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Player_dead(&self, owner: TRef<Node2D>) {
        owner
            .get_tree()
            .map(|tree| unsafe { tree.assume_safe() })
            .and_then(|tree| tree.reload_current_scene().ok())
            .expect("Failed to reload");
    }
}
