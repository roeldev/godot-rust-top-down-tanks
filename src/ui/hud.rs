// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::api::{AnimationPlayer, CanvasLayer, TextureProgress};
use gdnative::prelude::*;

use crate::ui::*;
use crate::utils::node::NodeRef;
use crate::utils::preload::*;

#[derive(Clone, Copy, PartialEq)]
enum HealthBarColor {
    Red,
    Yellow,
    Green,
}

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct Hud {
    healthbar_red_texture: Option<Ref<Texture>>,
    healthbar_yellow_texture: Option<Ref<Texture>>,
    healthbar_green_texture: Option<Ref<Texture>>,

    healthbar_color: HealthBarColor,

    // child node(s)
    healthbar_node: NodeRef<TextureProgress>,
    healthbar_tween_node: NodeRef<Tween>,
    healthbar_anim_node: NodeRef<AnimationPlayer>,
}

#[methods]
impl Hud {
    fn new(_owner: TRef<CanvasLayer>) -> Self {
        Hud {
            healthbar_red_texture: None,
            healthbar_yellow_texture: None,
            healthbar_green_texture: None,

            healthbar_color: HealthBarColor::Green,

            healthbar_node: NodeRef::new("Margin/HBoxContainer/HealthBar"),
            healthbar_tween_node: NodeRef::new("Margin/HBoxContainer/HealthBar/Tween"),
            healthbar_anim_node: NodeRef::new("AnimationPlayer"),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<CanvasLayer>) {
        self.healthbar_red_texture = try_preload::<Texture>(RES_HEALTHBAR_RED_TEXTURE, "");
        self.healthbar_yellow_texture = try_preload::<Texture>(RES_HEALTHBAR_YELLOW_TEXTURE, "");
        self.healthbar_green_texture = try_preload::<Texture>(RES_HEALTHBAR_GREEN_TEXTURE, "");

        let owner = owner.as_ref();
        self.healthbar_node.get_from(owner);
        self.healthbar_tween_node.get_from(owner);
        self.healthbar_anim_node.get_from(owner);
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Player_health_changed(&mut self, _owner: TRef<CanvasLayer>, value: f64) {
        if value < 25.0 {
            self.healthbar_color = HealthBarColor::Red;
        } else if value < 60.0 {
            self.healthbar_color = HealthBarColor::Yellow;
        } else {
            self.healthbar_color = HealthBarColor::Green;
        }

        let healthbar = self.healthbar_node.get_ref();
        let tween = self.healthbar_tween_node.get_ref();

        tween.interpolate_property(
            healthbar,
            "value",
            healthbar.value(),
            value,
            0.2,
            Tween::TRANS_LINEAR,
            Tween::EASE_IN_OUT,
            0.0,
        );
        tween.start();

        self.healthbar_anim_node
            .get_ref()
            .play(ANIM_HEALTHBAR_FLASH, -1.0, 1.0, false);
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_AnimationPlayer_animation_finished(&self, _owner: TRef<CanvasLayer>, anim_name: String) {
        if anim_name != ANIM_HEALTHBAR_FLASH {
            return;
        }

        let texture = match self.healthbar_color {
            HealthBarColor::Red => self.healthbar_red_texture.as_ref(),
            HealthBarColor::Yellow => self.healthbar_yellow_texture.as_ref(),
            HealthBarColor::Green => self.healthbar_green_texture.as_ref(),
        };
        if let Some(texture) = texture {
            self.healthbar_node.get_ref().set_progress_texture(texture);
        }
    }
}

const ANIM_HEALTHBAR_FLASH: &str = "healthbar_flash";
