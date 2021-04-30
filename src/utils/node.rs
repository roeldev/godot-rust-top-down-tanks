// Copyright (c) 2021, Roel Schut. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#![allow(dead_code)]

use gdnative::prelude::*;

#[inline]
pub unsafe fn get_parent_as<U>(node: &Node) -> Option<TRef<U>>
where
    U: SubClass<Node>,
{
    node.get_parent()?.assume_safe().cast::<U>()
}

#[inline]
pub fn with_parent_as<U, F, R>(node: &Node, mut func: F) -> Option<R>
where
    U: SubClass<Node>,
    F: FnMut(TRef<U>) -> R,
{
    match unsafe { get_parent_as::<U>(node) } {
        Some(parent) => Some(func(parent)),
        _ => None,
    }
}

#[inline]
pub fn get_node_as<'l, U>(owner: &Node, path: &str) -> TRef<'l, U>
where
    U: SubClass<Node>,
{
    unsafe { owner.get_node_as::<U>(path) }
        .unwrap_or_else(|| panic!("Failed to get {} as {}", path, U::class_name()))
}

#[inline]
pub fn with_node_as<U, F, R>(owner: &Node, path: &str, func: F) -> Option<R>
where
    U: SubClass<Node>,
    F: FnOnce(TRef<U>) -> R,
{
    owner
        .get_node_or_null(path)
        .map(|node| unsafe { node.assume_safe() })
        .and_then(|node| node.cast::<U>())
        .map(func)
}

pub struct NodeRef<T: GodotObject> {
    path: String,
    node: Option<Ref<T>>,
}

impl<T> NodeRef<T>
where
    T: GodotObject<RefKind = ManuallyManaged>
        + gdnative::prelude::SubClass<gdnative::prelude::Node>,
{
    pub fn new(path: impl ToString) -> Self {
        NodeRef {
            path: path.to_string(),
            node: None,
        }
    }

    #[inline]
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    #[inline]
    pub fn has_ref(&self) -> bool {
        self.node.is_some()
    }

    #[inline]
    pub fn get_from(&mut self, owner: &Node) -> Ref<T> {
        self.try_get_from(owner)
            .unwrap_or_else(|| panic!("Failed to find node `{}`", self.path))
    }

    #[inline]
    pub fn try_get_from(&mut self, owner: &Node) -> Option<Ref<T>> {
        if let Some(node_tref) = unsafe { owner.get_node_as::<T>(self.path()) } {
            self.node = Some(node_tref.claim())
        }

        self.node
    }

    #[inline]
    pub fn get_ref<'l>(&self) -> TRef<'l, T> {
        if self.node.is_none() {
            panic!(
                "Failed to get node `{}`, make sure it is found first.",
                self.path
            )
        }

        unsafe { self.node.unwrap().assume_safe() }
    }

    #[inline]
    pub fn get_ref_or_from(&mut self, owner: &Node) -> TRef<T> {
        if self.node.is_none() {
            self.get_from(owner);
        }

        self.get_ref()
    }
}
