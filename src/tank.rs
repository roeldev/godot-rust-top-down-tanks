// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::borrow::Borrow;

use gdnative::api::{AnimatedSprite, AnimationPlayer, CollisionShape2D, Position2D, Sprite};
use gdnative::nativescript::Map;
use gdnative::prelude::user_data::MapMut;
use gdnative::prelude::*;

use crate::enemies::*;
use crate::player::Player;
use crate::utils::node::{get_node_as, NodeRef};
use crate::utils::InstanceFrom;

// pub const ANIM_INIT: &str = "init";
pub const ANIM_MUZZLE_FLASH: &str = "muzzle_flash";

pub const SIGNAL_SHOOT: &str = "shoot";
pub const SIGNAL_HEALTH_CHANGED: &str = "health_changed";
pub const SIGNAL_DEAD: &str = "dead";

pub struct TankProperties {
    pub bullet_scene: Ref<PackedScene>,
    pub max_speed: f32,
    pub rotation_speed: f32,
    pub gun_cooldown: f64,
    pub max_health: u8,

    pub velocity: Vector2,
    pub health: u8,
    pub can_shoot: bool,
    pub alive: bool,

    pub body_node: NodeRef<Sprite>,
    pub gun_timer_node: NodeRef<Timer>,
    pub turret_node: NodeRef<Sprite>,
    pub turret_muzzle_node: NodeRef<Position2D>,
    pub turret_flash_node: NodeRef<Sprite>,
    pub anim_player_node: NodeRef<AnimationPlayer>,
}

impl TankProperties {
    pub fn new() -> Self {
        TankProperties {
            // exported
            bullet_scene: PackedScene::new().into_shared(),
            max_speed: 200.0,
            rotation_speed: 1.0,
            gun_cooldown: 0.5,
            max_health: 100,

            velocity: Vector2::zero(),
            health: 100,
            can_shoot: true,
            alive: true,

            // child node(s)
            body_node: NodeRef::new("Body"),
            gun_timer_node: NodeRef::new("GunTimer"),
            turret_node: NodeRef::new("Turret"),
            turret_muzzle_node: NodeRef::new("Turret/Muzzle"),
            turret_flash_node: NodeRef::new("Turret/Flash"),
            anim_player_node: NodeRef::new("AnimationPlayer"),
        }
    }
}

impl Default for TankProperties {
    fn default() -> Self {
        Self::new()
    }
}

pub trait BasicTank<C>
where
    C: NativeClass + BasicTank<C>,
{
    fn register_tank_properties(builder: &ClassBuilder<C>)
    where
        <C as gdnative::prelude::NativeClass>::UserData: MapMut,
        <C as gdnative::prelude::NativeClass>::UserData: Map,
        Self: Sized,
    {
        let default = TankProperties::new();

        builder
            .add_property::<Ref<PackedScene>>("bullet_scene")
            .with_setter(|t: &mut C, _, v: Ref<PackedScene>| t.props_mut().bullet_scene = v)
            .with_getter(|t: &C, _| -> Ref<PackedScene> { t.props().bullet_scene.clone() })
            .done();

        builder
            .add_property("max_speed")
            .with_default(default.max_speed)
            .with_setter(|t: &mut C, _, v: f32| t.props_mut().max_speed = v)
            .with_getter(|t: &C, _| -> f32 { t.props().max_speed })
            .done();

        builder
            .add_property("rotation_speed")
            .with_default(default.rotation_speed)
            .with_setter(|t: &mut C, _, v: f32| t.props_mut().rotation_speed = v)
            .with_getter(|t: &C, _| -> f32 { t.props().rotation_speed })
            .done();

        builder
            .add_property("gun_cooldown")
            .with_default(default.gun_cooldown)
            .with_setter(|t: &mut C, _, v: f64| t.props_mut().gun_cooldown = v)
            .with_getter(|t: &C, _| -> f64 { t.props().gun_cooldown })
            .done();

        builder
            .add_property("max_health")
            .with_default(default.max_health)
            .with_setter(|t: &mut C, _, v: u8| t.props_mut().max_health = v)
            .with_getter(|t: &C, _| -> u8 { t.props().max_health })
            .done();
    }

    fn register_tank_signals(builder: &ClassBuilder<C>) {
        let scene = Variant::from_object::<Ref<PackedScene>>(PackedScene::new().into_shared());

        builder.add_signal(Signal {
            name: SIGNAL_SHOOT,
            args: &[
                SignalArgument {
                    name: "bullet",
                    default: scene.clone(),
                    export_info: ExportInfo::new(scene.get_type()),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "position",
                    default: Variant::from_vector2(Vector2::zero().borrow()),
                    export_info: ExportInfo::new(VariantType::Vector2),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "direction",
                    default: Variant::from_vector2(Vector2::zero().borrow()),
                    export_info: ExportInfo::new(VariantType::Vector2),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });

        builder.add_signal(Signal {
            name: SIGNAL_HEALTH_CHANGED,
            args: &[SignalArgument {
                name: "percentage",
                default: Variant::from_f64(100.0),
                export_info: ExportInfo::new(VariantType::F64),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        builder.add_signal(Signal {
            name: SIGNAL_DEAD,
            args: &[],
        });
    }

    fn props(&self) -> &TankProperties;
    fn props_mut(&mut self) -> &mut TankProperties;
    fn control(&mut self, owner: TRef<KinematicBody2D>, delta: f32);

    #[inline]
    fn emit_signal_shoot(
        &self,
        owner: &KinematicBody2D,
        bullet: &Ref<PackedScene>,
        pos: Vector2,
        dir: Vector2,
    ) {
        owner.emit_signal(
            SIGNAL_SHOOT,
            &[
                Variant::from_object(bullet),
                Variant::from_vector2(pos.borrow()),
                Variant::from_vector2(dir.borrow()),
            ],
        );
    }

    #[inline]
    fn emit_signal_health_changed(&self, owner: &KinematicBody2D, percentage: f64) {
        owner.emit_signal(SIGNAL_HEALTH_CHANGED, &[Variant::from_f64(percentage)]);
    }

    #[inline]
    fn emit_signal_dead(&self, owner: TRef<KinematicBody2D>) {
        owner.emit_signal(SIGNAL_DEAD, &[]);
    }

    fn shoot(&mut self, owner: TRef<KinematicBody2D>) {
        if !self.props().can_shoot {
            return;
        }

        self.props_mut().can_shoot = false;
        self.props()
            .anim_player_node
            .get_ref()
            .play(ANIM_MUZZLE_FLASH, -1.0, 1.0, false);
        self.props().gun_timer_node.get_ref().start(0.0);

        self.emit_signal_shoot(
            owner.as_ref(),
            self.props().bullet_scene.borrow(),
            self.props().turret_muzzle_node.get_ref().global_position(),
            Vector2::new(1.0, 0.0).rotated(Angle::radians(
                self.props().turret_node.get_ref().global_rotation() as f32,
            )),
        );
    }

    fn take_damage(&mut self, owner: TRef<KinematicBody2D>, amount: u8) {
        let health = (self.props().health - amount).max(0);
        self.props_mut().health = health;

        let health = health as f64 / self.props().max_health as f64;
        self.emit_signal_health_changed(owner.as_ref(), health * 100.0);

        if health <= 0.0 {
            self.explode(owner);
        }
    }

    fn explode(&mut self, owner: TRef<KinematicBody2D>) {
        self.props_mut().alive = false;
        // owner.set_physics_process(false);

        unsafe {
            get_node_as::<CollisionShape2D>(owner.as_ref(), "CollisionShape2D")
                .call_deferred("set_disabled", &[Variant::from_bool(true)]);
        }

        self.props().turret_node.get_ref().hide();
        self.props().body_node.get_ref().hide();

        let explosion = get_node_as::<AnimatedSprite>(owner.as_ref(), "Explosion");
        explosion.show();
        explosion.play("fire", false);
    }

    #[inline]
    fn _ready(&mut self, owner: TRef<KinematicBody2D>) {
        let mut props = self.props_mut();

        let owner = owner.as_ref();
        props.body_node.get_from(owner);
        props.turret_node.get_from(owner);
        props.turret_muzzle_node.get_from(owner);
        props.turret_flash_node.get_from(owner);
        props.anim_player_node.get_from(owner);

        props.health = props.max_health;
        props
            .gun_timer_node
            .get_ref_or_from(owner)
            .set_wait_time(props.gun_cooldown);

        self.emit_signal_health_changed(owner, 100.0);
    }

    #[inline]
    fn _physics_process(&mut self, owner: TRef<KinematicBody2D>, delta: f32) {
        if !self.props().alive {
            return;
        }

        self.control(owner, delta);
        owner.move_and_slide(
            self.props().velocity,
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );
    }

    #[allow(non_snake_case)]
    #[inline]
    fn _on_GunTimer_timeout(&mut self, _owner: TRef<KinematicBody2D>) {
        self.props_mut().can_shoot = true;
    }

    #[allow(non_snake_case)]
    #[inline]
    fn _on_Explosion_animation_finished(&self, owner: TRef<KinematicBody2D>) {
        owner.emit_signal(SIGNAL_DEAD, &[]);
        owner.queue_free();
    }
}

#[inline]
pub fn take_damage(target: Ref<Node>, damage: u8) {
    if !unsafe { target.assume_safe() }.is_in_group("damage_taker") {
        return;
    }

    if Player::try_take_damage(target, damage) {
        return;
    }
    if EnemyTank::try_take_damage(target, damage) {
        return;
    }
    if GunTurret::try_take_damage(target, damage) {
        return;
    }

    panic!(
        "Cannot take damage, `target` {} is not checked",
        unsafe { target.assume_safe() }.name()
    )
}

pub trait DamageTaker<C>:
    InstanceFrom<C, gdnative::prelude::KinematicBody2D> + BasicTank<C>
where
    C: NativeClass<Base = gdnative::prelude::KinematicBody2D> + BasicTank<C>,
{
    #[inline]
    fn try_take_damage(node: Ref<Node>, damage: u8) -> bool
    where
        <C as gdnative::prelude::NativeClass>::UserData: MapMut,
    {
        let target = Self::try_instance_from(node);
        if target.is_none() {
            return false;
        }

        target
            .unwrap()
            .borrow()
            .map_mut(|target, node| {
                target.take_damage(node, damage);
                true
            })
            .ok()
            .unwrap_or_else(|| {
                panic!(
                    "Failed to apply damage to {}",
                    unsafe { node.assume_safe() }.name()
                )
            })
    }
}
