// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

mod bullet;
mod enemies;
mod map;
mod obstacle;
pub mod player;
pub mod tank;
mod ui;
mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<map::Map>();
    handle.add_tool_class::<obstacle::Obstacle>();
    handle.add_class::<bullet::Bullet>();
    handle.add_class::<player::Player>();
    handle.add_class::<enemies::EnemyTank>();
    handle.add_class::<enemies::GunTurret>();
    handle.add_class::<ui::Hud>();
    handle.add_class::<ui::UnitDisplay>();
}

godot_init!(init);
