// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::borrow::{Borrow, BorrowMut};

use gdnative::api::{CircleShape2D, CollisionShape2D, Node2D, PathFollow2D, RayCast2D};
use gdnative::prelude::*;
use interpolation::Lerp;

use crate::player;
use crate::tank::{BasicTank, DamageTaker, TankProperties};
use crate::utils::node::{get_node_as, get_parent_as, NodeRef};
use crate::utils::*;

use super::traits::TargetShooter;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register)]
pub struct EnemyTank {
    #[property(default = 2.0)]
    turret_speed: f32,
    #[property(default = 500.0)]
    detect_radius: f64,

    properties: TankProperties,
    speed: f32,
    target: Option<Ref<Node2D>>,

    // child node(s)
    lookahead1_node: NodeRef<RayCast2D>,
    lookahead2_node: NodeRef<RayCast2D>,
}

#[methods]
impl EnemyTank {
    fn register(builder: &ClassBuilder<Self>) {
        Self::register_tank_properties(builder);
        Self::register_tank_signals(builder);
    }

    fn new(_owner: TRef<KinematicBody2D>) -> Self {
        EnemyTank {
            turret_speed: 2.0,
            detect_radius: 500.0,

            properties: TankProperties::new(),
            speed: 0.0,
            target: None,

            lookahead1_node: NodeRef::new("LookAhead1"),
            lookahead2_node: NodeRef::new("LookAhead2"),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<KinematicBody2D>) {
        BasicTank::_ready(self, owner);

        let owner = owner.as_ref();
        self.lookahead1_node.get_from(owner);
        self.lookahead2_node.get_from(owner);

        let circle_shape = CircleShape2D::new();
        circle_shape.set_radius(self.detect_radius);
        get_node_as::<CollisionShape2D>(owner, "DetectRadius/CollisionShape2D")
            .set_shape(circle_shape);
    }

    //noinspection DuplicatedCode
    #[export]
    fn _process(&mut self, owner: TRef<KinematicBody2D>, delta: f32) {
        if let Some(target) = self.target {
            self.engage_target(
                owner,
                unsafe { target.assume_safe() },
                self.turret_speed * delta,
            );
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: TRef<KinematicBody2D>, delta: f32) {
        BasicTank::_physics_process(self, owner, delta)
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_GunTimer_timeout(&mut self, owner: TRef<KinematicBody2D>) {
        BasicTank::_on_GunTimer_timeout(self, owner);
    }

    //noinspection DuplicatedCode
    #[allow(non_snake_case)]
    #[export]
    fn _on_DetectRadius_body_entered(&mut self, _owner: TRef<KinematicBody2D>, body: Ref<Node2D>) {
        if player::is_player_node(unsafe { body.assume_safe() }) {
            self.target = Some(body);
        }
    }

    //noinspection DuplicatedCode
    #[allow(non_snake_case)]
    #[export]
    fn _on_DetectRadius_body_exited(&mut self, _owner: TRef<KinematicBody2D>, body: Ref<Node2D>) {
        if player::is_player_node(unsafe { body.assume_safe() }) {
            self.target = None;
        }
    }

    #[allow(non_snake_case)]
    #[export]
    fn _on_Explosion_animation_finished(&self, owner: TRef<KinematicBody2D>) {
        BasicTank::_on_Explosion_animation_finished(self, owner);
    }
}

impl BasicTank<Self> for EnemyTank {
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
        if self.lookahead1_node.get_ref().is_colliding()
            || self.lookahead2_node.get_ref().is_colliding()
        {
            self.speed = self.speed.lerp(&0.0, &0.1);
        } else {
            self.speed = self.speed.lerp(&self.properties.max_speed, &0.05);
        }

        if let Some(parent) = unsafe { get_parent_as::<PathFollow2D>(owner.as_ref()) } {
            parent.set_offset(parent.offset() + (self.speed * delta) as f64);
            owner.set_position(Vector2::zero());
        }
    }
}

impl InstanceFrom<Self, KinematicBody2D> for EnemyTank {}

impl DamageTaker<Self> for EnemyTank {}

impl TargetShooter<Self> for EnemyTank {}
