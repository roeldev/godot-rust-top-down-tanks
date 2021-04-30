// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

#[allow(dead_code)]
#[inline]
pub fn instance_scene(scene: Ref<PackedScene>, edit_state: i64) -> Ref<Node> {
    unsafe { scene.assume_safe() }
        .instance(edit_state)
        .unwrap_or_else(|| panic!("Failed to create instance of `{:?}`", scene))
}

#[allow(dead_code)]
#[inline]
pub fn instance_scene_as<'l, U>(scene: Ref<PackedScene>, edit_state: i64) -> TRef<'l, U>
where
    U: SubClass<Node>,
{
    unsafe { instance_scene(scene.clone(), edit_state).assume_safe() }
        .cast::<U>()
        .unwrap_or_else(|| {
            panic!(
                "Failed to cast scene `{:?}` to `{}`",
                scene,
                U::class_name()
            )
        })
}
