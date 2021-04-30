// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::prelude::*;

pub trait InstanceFrom<T, U>
where
    T: NativeClass + NativeClass<Base = U>,
    U: GodotObject + SubClass<Node>,
{
    #[inline]
    fn try_instance_from<'l>(node: Ref<Node>) -> Option<RefInstance<'l, T, Shared>> {
        unsafe { node.assume_safe() }
            .cast::<U>()?
            .cast_instance::<T>()
    }

    #[inline]
    fn instance_from<'l>(node: Ref<Node>) -> RefInstance<'l, T, Shared> {
        Self::try_instance_from(node)
            .unwrap_or_else(|| panic!("Failed to convert node {:?} to {}", node, T::class_name()))
    }
}
