// Copyright 2021 Robin Freyler
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    all(feature = "alloc", feature = "unstable"),
    feature(const_btree_new)
)]
#![allow(clippy::declare_interior_mutable_const)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "derive")]
pub use const_default_derive::ConstDefault;

use core::{
    cell::{Cell, RefCell, UnsafeCell},
    iter::{self, Empty},
    marker::{PhantomData, PhantomPinned},
    mem::{ManuallyDrop, MaybeUninit},
    num::Wrapping,
    ptr,
    sync::atomic::{
        AtomicBool,
        AtomicI16,
        AtomicI32,
        AtomicI64,
        AtomicI8,
        AtomicIsize,
        AtomicPtr,
        AtomicU16,
        AtomicU32,
        AtomicU64,
        AtomicU8,
        AtomicUsize,
    },
    time::Duration,
};

#[cfg(feature = "alloc")]
use alloc::{
    borrow::Cow,
    borrow::ToOwned,
    collections::LinkedList,
    string::String,
    vec::Vec,
};

#[cfg(all(feature = "alloc", feature = "unstable"))]
use alloc::collections::{BTreeMap, BTreeSet};

/// Implements a compilation time default value for the implemented type.
///
/// # Note
///
/// Unlike the [`Default`] trait implementation the `DEFAULT` of implementations
/// of this trait can be used in constant evaluation contexts.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "std")]
/// # const _: () = {
/// # use const_default::ConstDefault;
/// const VEC: Vec<u8> = <Vec<u8> as ConstDefault>::DEFAULT;
/// # };
/// ```
///
/// The above code works while the below code does not:
///
/// ```compile_fail
/// const VEC: Vec<u8> = <Vec<u8> as Default>::default();
/// ```
pub trait ConstDefault {
    /// The constant default value.
    const DEFAULT: Self;
}

macro_rules! impl_const_default_for_integer {
    ( $( $prim:ty ),* ) => {
        $(
            impl ConstDefault for $prim {
                const DEFAULT: Self = 0;
            }
        )*
    };
}
impl_const_default_for_integer!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

macro_rules! impl_const_default_for_atomic_integer {
    ( $( $atomic_integer:ty ),* ) => {
        $(
            impl ConstDefault for $atomic_integer {
                const DEFAULT: Self = Self::new(0);
            }
        )*
    };
}
impl_const_default_for_atomic_integer!(
    AtomicI8,
    AtomicI16,
    AtomicI32,
    AtomicI64,
    AtomicIsize,
    AtomicU8,
    AtomicU16,
    AtomicU32,
    AtomicU64,
    AtomicUsize
);

impl ConstDefault for AtomicBool {
    const DEFAULT: Self = Self::new(false);
}

impl<T> ConstDefault for AtomicPtr<T> {
    const DEFAULT: Self = Self::new(ptr::null_mut());
}

macro_rules! impl_const_default_for_float {
    ( $( $prim:ty ),* ) => {
        $(
            impl ConstDefault for $prim {
                const DEFAULT: Self = 0.0;
            }
        )*
    };
}
impl_const_default_for_float!(f32, f64);

impl<T, const N: usize> ConstDefault for [T; N]
where
    T: ConstDefault,
{
    const DEFAULT: Self = [<T as ConstDefault>::DEFAULT; N];
}

impl<'a, T> ConstDefault for &'a [T]
where
    T: 'a,
{
    const DEFAULT: Self = &[];
}

macro_rules! impl_const_default_for_tuple {
    ( $( $ty:ident ),* ) => {
        impl< $($ty),* > ConstDefault for ( $($ty ,)* )
        where
            $(
                $ty: ConstDefault
            ),*
        {
            const DEFAULT: Self = (
                $(
                    <$ty as ConstDefault>::DEFAULT,
                )*
            );
        }
    };
}
impl_const_default_for_tuple!();
impl_const_default_for_tuple!(T1);
impl_const_default_for_tuple!(T1, T2);
impl_const_default_for_tuple!(T1, T2, T3);
impl_const_default_for_tuple!(T1, T2, T3, T4);
impl_const_default_for_tuple!(T1, T2, T3, T4, T5);
impl_const_default_for_tuple!(T1, T2, T3, T4, T5, T6);
impl_const_default_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_const_default_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_const_default_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_const_default_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_const_default_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_const_default_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12
);

impl ConstDefault for bool {
    const DEFAULT: Self = false;
}

impl ConstDefault for char {
    const DEFAULT: Self = '\x00';
}

impl ConstDefault for &str {
    const DEFAULT: Self = "";
}

impl<T> ConstDefault for Option<T> {
    const DEFAULT: Self = None;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl ConstDefault for String {
    const DEFAULT: Self = Self::new();
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> ConstDefault for Vec<T> {
    const DEFAULT: Self = Self::new();
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> ConstDefault for LinkedList<T> {
    const DEFAULT: Self = Self::new();
}

impl<T> ConstDefault for Cell<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = Self::new(<T as ConstDefault>::DEFAULT);
}

impl<T> ConstDefault for ManuallyDrop<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = Self::new(<T as ConstDefault>::DEFAULT);
}

impl<T> ConstDefault for MaybeUninit<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = Self::new(<T as ConstDefault>::DEFAULT);
}

impl<T> ConstDefault for UnsafeCell<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = Self::new(<T as ConstDefault>::DEFAULT);
}

impl<T> ConstDefault for RefCell<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = Self::new(<T as ConstDefault>::DEFAULT);
}

impl<T> ConstDefault for Wrapping<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = Self(<T as ConstDefault>::DEFAULT);
}

impl ConstDefault for Duration {
    const DEFAULT: Self = Self::from_secs(0);
}

impl<T> ConstDefault for Empty<T> {
    const DEFAULT: Self = iter::empty();
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, T> ConstDefault for Cow<'a, T>
where
    T: ToOwned + ?Sized + 'a,
    <T as ToOwned>::Owned: ConstDefault,
{
    const DEFAULT: Self =
        Self::Owned(<<T as ToOwned>::Owned as ConstDefault>::DEFAULT);
}

#[cfg(all(any(feature = "std", feature = "alloc"), feature = "unstable"))]
impl<K: Ord, V> ConstDefault for BTreeMap<K, V> {
    const DEFAULT: Self = Self::new();
}

#[cfg(all(any(feature = "std", feature = "alloc"), feature = "unstable"))]
impl<T: Ord> ConstDefault for BTreeSet<T> {
    const DEFAULT: Self = Self::new();
}

impl<T> ConstDefault for PhantomData<T> {
    const DEFAULT: Self = Self;
}

impl ConstDefault for PhantomPinned {
    const DEFAULT: Self = Self;
}

impl<T> ConstDefault for *const T {
    const DEFAULT: Self = ptr::null();
}

impl<T> ConstDefault for *mut T {
    const DEFAULT: Self = ptr::null_mut();
}
