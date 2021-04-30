#[allow(non_snake_case)]
#[allow(dead_code)]
use std::ops::Deref;

use gdnative::prelude::*;

pub trait Scene {
    fn get_child_node<'l, T>(&self, owner: &Node) -> TRef<'l, T::Base>
        where T: ChildNodeRef,
              <T as ChildNodeRef>::Base: gdnative::prelude::SubClass<gdnative::prelude::Node>
    {
        unsafe { T::get_node_from(owner) }
            .expect(format!("Failed to get `{}` as {}", T::path(), gdnative::api::Sprite::class_name()).deref())
    }
}

pub trait ChildNodeRef: Sized + 'static
{
    type Base: GodotObject;

    fn path<'l>() -> &'l str;

    unsafe fn get_node_from<'l>(owner: &Node) -> Option<TRef<'l, Self::Base>>
        where <Self as ChildNodeRef>::Base: gdnative::prelude::SubClass<gdnative::prelude::Node>
    {
        owner.get_node_as::<Self::Base>(Self::path())
    }
}

pub mod bullets {}

pub mod tanks {
    pub mod tank_tscn {
        use super::super::ChildNodeRef;

        pub const ANIM_INIT: &str = "init";
        pub const ANIM_MUZZLE_FLASH: &str = "muzzle_flash";

        pub struct BodyNode {}

        impl ChildNodeRef for BodyNode {
            type Base = gdnative::api::Sprite;
            fn path<'l>() -> &'l str { "Body" }
        }

        pub struct CollisionShape2DNode {}

        impl ChildNodeRef for CollisionShape2DNode {
            type Base = gdnative::api::CollisionShape2D;
            fn path<'l>() -> &'l str { "CollisionShape2D" }
        }

        pub struct TurretNode {}

        impl ChildNodeRef for TurretNode {
            type Base = gdnative::api::Sprite;
            fn path<'l>() -> &'l str { "Turret" }
        }

        pub struct TurretMuzzleNode {}

        impl ChildNodeRef for TurretMuzzleNode {
            type Base = gdnative::api::Position2D;
            fn path<'l>() -> &'l str { "Turret/Muzzle" }
        }

        pub struct TurretFlashNode {}

        impl ChildNodeRef for TurretFlashNode {
            type Base = gdnative::api::Sprite;
            fn path<'l>() -> &'l str { "Turret/Flash" }
        }

        pub struct GunTimerNode {}

        impl ChildNodeRef for GunTimerNode {
            type Base = gdnative::api::Timer;
            fn path<'l>() -> &'l str { "GunTimer" }
        }

        pub struct AnimationPlayerNode {}

        impl ChildNodeRef for AnimationPlayerNode {
            type Base = gdnative::api::AnimationPlayer;
            fn path<'l>() -> &'l str { "AnimationPlayer" }
        }

        pub struct ExplosionNode {}

        impl ChildNodeRef for ExplosionNode {
            type Base = gdnative::api::AnimatedSprite;
            fn path<'l>() -> &'l str { "Explosion" }
        }
    }
}

pub mod ui {}
