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

use const_default::ConstDefault;
use core::{
    cell::{Cell, RefCell},
    fmt::Debug,
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
        Ordering,
    },
};

/// Checks if both `ConstDefault` and `Default` implementations yield the same outcome.
fn compare_default_impls<T>()
where
    T: ConstDefault + Default + PartialEq + Debug,
{
    assert_eq!(<T as ConstDefault>::DEFAULT, <T as Default>::default());
}

macro_rules! compare_default_impls_for {
    ( $( $ty:ty ),* $(,)? ) => {{
        $(
            compare_default_impls::<$ty>();
        )*
    }};
}

#[test]
fn primitive_impls_work() {
    #[rustfmt::skip]
    compare_default_impls_for!(
        bool, char,
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize,
    );
}

#[test]
fn tuple_impls_work() {
    #[rustfmt::skip]
    compare_default_impls_for!(
        (),
        (i8,),
        (i8, i16),
        (i8, i16, i32),
        (i8, i16, i32, i64),
        (i8, i16, i32, i64, i128),
        (i8, i16, i32, i64, i128, isize),
        (i8, i16, i32, i64, i128, isize, u8),
        (i8, i16, i32, i64, i128, isize, u8, u16),
        (i8, i16, i32, i64, i128, isize, u8, u16, u32),
        (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64),
        (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128),
        (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize),
    );
}

macro_rules! compare_default_impls_for_arrays {
    ( $( $n:literal ),* $(,)? ) => {{
        $(
            compare_default_impls::<[(); $n]>();
        )*
    }};
}

#[test]
fn array_impls_work() {
    #[rustfmt::skip]
    compare_default_impls_for_arrays!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        30, 31, 32,
    );
}
