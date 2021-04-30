// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

use crate::tank::BasicTank;

pub trait TargetShooter<C>: BasicTank<C>
where
    C: NativeClass + BasicTank<C>,
{
    fn engage_target(
        &mut self,
        owner: TRef<KinematicBody2D>,
        target: TRef<Node2D>,
        rotation_speed: f32,
    ) {
        let target_dir = (target.global_position() - owner.global_position()).normalize();

        let turret = self.props().turret_node.get_ref();
        let current_dir =
            Vector2::new(1.0, 0.0).rotated(Angle::radians(turret.global_rotation() as f32));

        let vec = current_dir.lerp(target_dir, rotation_speed);
        turret.set_global_rotation(vec.y.atan2(vec.x) as f64);

        if target_dir.dot(current_dir) > 0.9 {
            self.shoot(owner);
        }
    }
}
