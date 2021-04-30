// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::ops::{Add, Mul};

use gdnative::api::{AnimatedSprite, Area2D};
use gdnative::prelude::*;

use crate::tank;
use crate::utils::node::get_node_as;
use crate::utils::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Bullet {
    #[property(default = 750.0)]
    speed: f32,
    #[property(default = 10)]
    damage: u8,
    #[property(default = 1.0)]
    lifetime: f64,

    velocity: Vector2,
    exploding: bool,
}

#[methods]
impl Bullet {
    fn new(_owner: TRef<Area2D>) -> Self {
        Bullet {
            speed: 750.0,
            damage: 10,
            lifetime: 1.0,
            velocity: Vector2::zero(),
            exploding: false,
        }
    }

    pub fn start(&mut self, owner: TRef<Area2D>, position: Vector2, direction: Vector2) {
        owner.set_position(position);
        owner.set_rotation(direction.y.atan2(direction.x) as f64);
        self.velocity = direction.mul(self.speed);

        let timer = get_node_as::<Timer>(owner.as_ref(), "Lifetime");
        timer.set_wait_time(self.lifetime);
        timer.start(0.0);
    }

    pub fn explode(&mut self, owner: TRef<Area2D>) -> bool {
        if self.exploding {
            return false;
        }

        self.exploding = true;
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;

        let owner = owner.as_ref();
        get_node_as::<Sprite>(owner, "Sprite").hide();

        let explosion = get_node_as::<AnimatedSprite>(owner, "Explosion");
        explosion.show();
        explosion.play("smoke", false);

        true
    }

    #[export]
    fn _ready(&self, _owner: TRef<Area2D>) {}

    #[export]
    fn _process(&mut self, owner: TRef<Area2D>, delta: f32) {
        owner.set_position(owner.position().add(self.velocity.mul(delta)));
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Bullet_body_entered(&mut self, owner: TRef<Area2D>, body: Ref<Node>) {
        if self.explode(owner) {
            tank::take_damage(body, self.damage);
        }
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Lifetime_timeout(&mut self, owner: TRef<Area2D>) {
        self.explode(owner);
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Explosion_animation_finished(&self, owner: TRef<Area2D>) {
        owner.queue_free();
    }
}

impl InstanceFrom<Self, Area2D> for Bullet {}
