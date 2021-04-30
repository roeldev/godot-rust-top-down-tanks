// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

pub use hud::Hud;
pub use unit_display::UnitDisplay;

mod hud;
mod unit_display;

const RES_HEALTHBAR_RED_TEXTURE: &str = "res://ui/barHorizontal_red_mid 200.png";
const RES_HEALTHBAR_YELLOW_TEXTURE: &str = "res://ui/barHorizontal_yellow_mid 200.png";
const RES_HEALTHBAR_GREEN_TEXTURE: &str = "res://ui/barHorizontal_green_mid 200.png";
