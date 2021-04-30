// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::ops::Mul;

use gdnative::api::{CollisionShape2D, RectangleShape2D, StaticBody2D};
use gdnative::nativescript::property::{EnumHint, StringHint};
use gdnative::prelude::*;

use crate::utils::node::get_node_as;

#[derive(PartialEq)]
enum ObstacleType {
    Invalid,
    BarrelBlackSide,
    BarrelBlackTop,
    BarrelGreenSide,
    BarrelGreenTop,
    BarrelRedSide,
    BarrelRedTop,
    BarrelRustSide,
    BarrelRustTop,
    BarricadeMetal,
    BarricadeWood,
    FenceRed,
    FenceYellow,
    SandbagBeige,
    SandbagBeigeOpen,
    SandbagBrown,
    SandbagBrownOpen,
    TreeBrownLarge,
    TreeBrownSmall,
    TreeGreenLarge,
    TreeGreenSmall,
}

impl ObstacleType {
    #[inline]
    pub fn all() -> Vec<ObstacleType> {
        vec![
            Self::Invalid,
            Self::BarrelBlackSide,
            Self::BarrelBlackTop,
            Self::BarrelGreenSide,
            Self::BarrelGreenTop,
            Self::BarrelRedSide,
            Self::BarrelRedTop,
            Self::BarrelRustSide,
            Self::BarrelRustTop,
            Self::BarricadeMetal,
            Self::BarricadeWood,
            Self::FenceRed,
            Self::FenceYellow,
            Self::SandbagBeige,
            Self::SandbagBeigeOpen,
            Self::SandbagBrown,
            Self::SandbagBrownOpen,
            Self::TreeBrownLarge,
            Self::TreeBrownSmall,
            Self::TreeGreenLarge,
            Self::TreeGreenSmall,
        ]
    }

    #[inline]
    pub fn name(&self) -> &str {
        match self {
            Self::Invalid => "",
            Self::BarrelBlackSide => "barrelBlack_side",
            Self::BarrelBlackTop => "barrelBlack_top",
            Self::BarrelGreenSide => "barrelGreen_side",
            Self::BarrelGreenTop => "barrelGreen_top",
            Self::BarrelRedSide => "barrelRed_side",
            Self::BarrelRedTop => "barrelRed_top",
            Self::BarrelRustSide => "barrelRust_side",
            Self::BarrelRustTop => "barrelRust_top",
            Self::BarricadeMetal => "barricadeMetal",
            Self::BarricadeWood => "barricadeWood",
            Self::FenceRed => "fenceRed",
            Self::FenceYellow => "fenceYellow",
            Self::SandbagBeige => "sandbagBeige",
            Self::SandbagBeigeOpen => "sandbagBeige_open",
            Self::SandbagBrown => "sandbagBrown",
            Self::SandbagBrownOpen => "sandbagBrown_open",
            Self::TreeBrownLarge => "treeBrown_large",
            Self::TreeBrownSmall => "treeBrown_small",
            Self::TreeGreenLarge => "treeGreen_large",
            Self::TreeGreenSmall => "treeGreen_small",
        }
    }

    #[inline]
    pub fn rect(&self) -> Rect2 {
        match self {
            Self::Invalid => Rect2::zero(),
            Self::BarrelBlackSide => Rect2::new(Point2::new(532.0, 90.0), Size2::new(56.0, 40.0)),
            Self::BarrelBlackTop => Rect2::new(Point2::new(220.0, 89.0), Size2::new(48.0, 48.0)),
            Self::BarrelGreenSide => Rect2::new(Point2::new(476.0, 90.0), Size2::new(56.0, 40.0)),
            Self::BarrelGreenTop => Rect2::new(Point2::new(220.0, 137.0), Size2::new(48.0, 48.0)),
            Self::BarrelRedSide => Rect2::new(Point2::new(420.0, 94.0), Size2::new(56.0, 40.0)),
            Self::BarrelRedTop => Rect2::new(Point2::new(172.0, 89.0), Size2::new(48.0, 48.0)),
            Self::BarrelRustSide => Rect2::new(Point2::new(588.0, 90.0), Size2::new(56.0, 40.0)),
            Self::BarrelRustTop => Rect2::new(Point2::new(172.0, 137.0), Size2::new(48.0, 48.0)),
            Self::BarricadeMetal => Rect2::new(Point2::new(532.0, 130.0), Size2::new(56.0, 56.0)),
            Self::BarricadeWood => Rect2::new(Point2::new(72.0, 130.0), Size2::new(56.0, 56.0)),
            Self::FenceRed => Rect2::new(Point2::new(336.0, 443.0), Size2::new(32.0, 96.0)),
            Self::FenceYellow => Rect2::new(Point2::new(216.0, 550.0), Size2::new(32.0, 104.0)),
            Self::SandbagBeige => Rect2::new(Point2::new(164.0, 282.0), Size2::new(44.0, 64.0)),
            Self::SandbagBeigeOpen => Rect2::new(Point2::new(518.0, 350.0), Size2::new(55.0, 84.0)),
            Self::SandbagBrown => Rect2::new(Point2::new(622.0, 278.0), Size2::new(44.0, 64.0)),
            Self::SandbagBrownOpen => Rect2::new(Point2::new(596.0, 450.0), Size2::new(55.0, 84.0)),
            Self::TreeBrownLarge => Rect2::new(Point2::new(0.0, 654.0), Size2::new(128.0, 128.0)),
            Self::TreeBrownSmall => Rect2::new(Point2::new(694.0, 118.0), Size2::new(72.0, 72.0)),
            Self::TreeGreenLarge => Rect2::new(Point2::new(128.0, 654.0), Size2::new(128.0, 128.0)),
            Self::TreeGreenSmall => Rect2::new(Point2::new(694.0, 190.0), Size2::new(72.0, 72.0)),
        }
    }
}

impl From<String> for ObstacleType {
    #[inline]
    fn from(name: String) -> Self {
        let name = name.as_str();
        for typ in Self::all() {
            if typ.name().eq(name) {
                return typ;
            }
        }

        Self::Invalid
    }
}

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
#[register_with(Self::register)]
pub struct Obstacle {
    typ: ObstacleType,
}

#[methods]
impl Obstacle {
    fn register(builder: &ClassBuilder<Self>) {
        let obstacle_types = ObstacleType::all();
        let mut vec = Vec::<String>::new();
        vec.reserve(obstacle_types.len());
        for typ in obstacle_types {
            vec.push(typ.name().to_string());
        }

        builder
            .add_property::<String>("type_name")
            .with_hint(StringHint::Enum(EnumHint::new(vec)))
            .with_setter(Self::update)
            .with_getter(|t: &Obstacle, _| -> String { t.typ.name().to_string() })
            .done();
    }

    fn new(_owner: TRef<StaticBody2D>) -> Self {
        Obstacle {
            typ: ObstacleType::BarrelBlackSide,
        }
    }

    fn update(&mut self, owner: TRef<StaticBody2D>, type_name: String) {
        self.typ = ObstacleType::from(type_name.clone());
        if self.typ == ObstacleType::Invalid {
            if !type_name.eq("") {
                godot_warn!("Invalid ObstacleType `{}`", type_name.as_str());
            }
            return;
        }

        let owner = owner.as_ref();
        let sprite = get_node_as::<Sprite>(owner, "Sprite");
        sprite.set_region_rect(self.typ.rect());

        let rect_shape = RectangleShape2D::new();
        rect_shape.set_extents(sprite.region_rect().size.to_vector().mul(0.5));
        get_node_as::<CollisionShape2D>(owner, "CollisionShape2D").set_shape(rect_shape);
    }
}
