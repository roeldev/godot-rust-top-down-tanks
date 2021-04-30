// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::api::{Node2D, TextureProgress};
use gdnative::prelude::*;

use crate::ui::*;
use crate::utils::node::NodeRef;
use crate::utils::preload::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct UnitDisplay {
    healthbar_red_texture: Option<Ref<Texture>>,
    healthbar_yellow_texture: Option<Ref<Texture>>,
    healthbar_green_texture: Option<Ref<Texture>>,

    // child node(s)
    healthbar_node: NodeRef<TextureProgress>,
}

#[methods]
impl UnitDisplay {
    fn new(_owner: TRef<Node2D>) -> Self {
        UnitDisplay {
            healthbar_red_texture: None,
            healthbar_yellow_texture: None,
            healthbar_green_texture: None,

            healthbar_node: NodeRef::new("HealthBar"),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        self.healthbar_red_texture = Some(preload::<Texture>(RES_HEALTHBAR_RED_TEXTURE));
        self.healthbar_yellow_texture = Some(preload::<Texture>(RES_HEALTHBAR_YELLOW_TEXTURE));
        self.healthbar_green_texture = Some(preload::<Texture>(RES_HEALTHBAR_GREEN_TEXTURE));
        self.healthbar_node.get_from(owner.as_ref());
        owner.hide();
    }

    #[export]
    fn _process(&self, owner: TRef<Node2D>, _delta: f32) {
        owner.set_global_rotation(0.0);
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_health_changed(&mut self, owner: TRef<Node2D>, value: f64) {
        let mut change_texture: Option<&Ref<Texture>> = None;
        if value < 25.0 {
            change_texture = self.healthbar_red_texture.as_ref();
        } else if value < 60.0 {
            change_texture = self.healthbar_yellow_texture.as_ref();
        } else if value < 100.0 {
            owner.show();
        }

        let healthbar = self.healthbar_node.get_ref();
        healthbar.set_value(value);

        if let Some(texture) = change_texture {
            healthbar.set_progress_texture(texture);
        }
    }
}
