// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::borrow::{Borrow, BorrowMut};

use gdnative::prelude::*;

use crate::tank::{BasicTank, DamageTaker, TankProperties};
use crate::utils::*;

pub const NAME: &str = "Player";

#[inline]
pub fn is_player_node(node: TRef<Node2D>) -> bool {
    node.name().to_string().eq(NAME)
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register)]
pub struct Player {
    properties: TankProperties,
}

#[methods]
impl Player {
    fn register(builder: &ClassBuilder<Self>) {
        Self::register_tank_properties(builder);
        Self::register_tank_signals(builder);
    }

    fn new(_owner: TRef<KinematicBody2D>) -> Self {
        Player {
            properties: TankProperties::new(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<KinematicBody2D>) {
        BasicTank::_ready(self, owner);
    }

    #[export]
    fn _physics_process(&mut self, owner: TRef<KinematicBody2D>, delta: f32) {
        BasicTank::_physics_process(self, owner, delta);
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_GunTimer_timeout(&mut self, owner: TRef<KinematicBody2D>) {
        BasicTank::_on_GunTimer_timeout(self, owner);
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Explosion_animation_finished(&self, owner: TRef<KinematicBody2D>) {
        BasicTank::_on_Explosion_animation_finished(self, owner);
    }
}

impl BasicTank<Self> for Player {
    #[inline]
    fn props(&self) -> &TankProperties {
        self.properties.borrow()
    }

    #[inline]
    fn props_mut(&mut self) -> &mut TankProperties {
        self.properties.borrow_mut()
    }

    #[inline]
    fn control(&mut self, owner: TRef<KinematicBody2D>, delta: f32) {
        self.properties
            .turret_node
            .get_ref()
            .look_at(owner.get_global_mouse_position());

        let mut rot_dir = 0.0;
        let input = Input::godot_singleton();
        if input.is_action_pressed("turn_right") {
            rot_dir += 1.0;
        }
        if input.is_action_pressed("turn_left") {
            rot_dir -= 1.0;
        }

        let rotation = owner.rotation() + (self.properties.rotation_speed * rot_dir * delta) as f64;
        owner.set_rotation(rotation);

        let mut velocity = Vector2::zero();
        if input.is_action_pressed("forward") {
            velocity.x = self.properties.max_speed;
            velocity = velocity.rotated(Angle::radians(rotation as f32));
        }
        if input.is_action_pressed("back") {
            velocity.x = -self.properties.max_speed / 2.0;
            velocity = velocity.rotated(Angle::radians(rotation as f32));
        }

        self.properties.velocity = velocity;

        if input.is_action_just_pressed("click") {
            self.shoot(owner);
        }
    }
}

impl InstanceFrom<Self, KinematicBody2D> for Player {}

impl DamageTaker<Self> for Player {}
