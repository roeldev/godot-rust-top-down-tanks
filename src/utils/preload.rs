// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use gdnative::api::Resource;
use gdnative::prelude::*;

#[allow(dead_code)]
#[inline]
pub fn try_preload<T>(path: &str, type_hint: &str) -> Option<Ref<T, Shared>>
where
    T: GodotObject<RefKind = <Resource as GodotObject>::RefKind>
        + gdnative::prelude::SubClass<Resource>,
{
    let mut type_hint = type_hint;
    if type_hint.is_empty() {
        type_hint = T::class_name()
    }

    ResourceLoader::godot_singleton()
        .load(path, type_hint, false)
        .map(|resource| unsafe { resource.assume_unique() })
        .map(|resource| resource.into_shared())
        .and_then(|resource| resource.cast::<T>())
}

#[allow(dead_code)]
#[inline]
pub fn preload<T>(path: &str) -> Ref<T, Shared>
where
    T: GodotObject<RefKind = <Resource as GodotObject>::RefKind>
        + gdnative::prelude::SubClass<Resource>,
{
    preload_with_hint(path, T::class_name())
}

#[allow(dead_code)]
#[inline]
pub fn preload_with_hint<T>(path: &str, type_hint: &str) -> Ref<T, Shared>
where
    T: GodotObject<RefKind = <Resource as GodotObject>::RefKind>
        + gdnative::prelude::SubClass<Resource>,
{
    try_preload(path, type_hint)
        .unwrap_or_else(|| panic!("Failed to preload `{}`", path.to_string()))
}

// pub struct ResourceRef<T>
//     where T: GodotObject<RefKind=<Resource as GodotObject>::RefKind> + gdnative::prelude::SubClass<Resource>
// {
//     resource: Option<Ref<T>>,
//     type_hint: String,
// }
//
// impl<T> ResourceRef<T>
//     where T: GodotObject<RefKind=<Resource as GodotObject>::RefKind> + gdnative::prelude::SubClass<Resource>
// {
//     #[allow(dead_code)]
//     #[inline]
//     pub fn get_type_hint(&self) -> &str { self.type_hint.as_str() }
//
//     #[allow(dead_code)]
//     pub fn preload(&mut self, path: &str) {
//         self.resource = Some(preload::<T>(path, self.type_hint.as_str()));
//     }
//
//     #[allow(dead_code)]
//     pub fn try_preload(&mut self, path: &str) {
//         self.resource = try_preload::<T>(path, self.type_hint.as_str());
//     }
//
//     #[allow(dead_code)]
//     pub fn get_ref(&self) -> &Ref<T> {
//         self.resource.as_ref().expect("Failed to get resource")
//     }
// }
//
// impl<T> Default for ResourceRef<T>
//     where T: GodotObject<RefKind=<Resource as GodotObject>::RefKind> + gdnative::prelude::SubClass<Resource> {
//     #[inline]
//     fn default() -> Self {
//         ResourceRef {
//             resource: None,
//             type_hint: String::from(T::class_name()),
//         }
//     }
// }
