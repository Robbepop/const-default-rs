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

#![cfg(feature = "derive")]

use const_default::ConstDefault;
use core::{
    cell::{Cell, RefCell},
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

#[test]
fn struct_of_primitives_works() {
    #[derive(ConstDefault, Debug, Default, PartialEq)]
    pub struct TestType {
        field_0: bool,
        field_1: i8,
        field_2: i16,
        field_3: i32,
        field_4: i64,
        field_5: i128,
        field_6: isize,
        field_7: u8,
        field_8: u16,
        field_9: u32,
        field_10: u64,
        field_11: u128,
        field_12: usize,
        field_13: char,
    }
    assert_eq!(<TestType as ConstDefault>::DEFAULT, TestType::default());
}

#[test]
fn tuple_struct_of_primitives_works() {
    #[derive(ConstDefault, Debug, Default, PartialEq)]
    pub struct TestType(
        bool,
        i8,
        i16,
        i32,
        i64,
        i128,
        isize,
        u8,
        u16,
        u32,
        u64,
        u128,
        usize,
        char,
    );
    assert_eq!(<TestType as ConstDefault>::DEFAULT, TestType::default());
}

#[test]
fn struct_of_atomic_primitives_works() {
    #[derive(ConstDefault, Debug, Default)]
    pub struct TestType {
        field_1: AtomicBool,
        field_2: AtomicI8,
        field_3: AtomicI16,
        field_4: AtomicI32,
        field_5: AtomicI64,
        field_6: AtomicIsize,
        field_7: AtomicU8,
        field_8: AtomicU16,
        field_9: AtomicU32,
        field_10: AtomicU64,
        field_11: AtomicUsize,
        field_12: AtomicPtr<i32>,
    }
    let t1 = <TestType as ConstDefault>::DEFAULT;
    let t2 = TestType::default();
    use Ordering::SeqCst as O;
    assert_eq!(t1.field_1.load(O), t2.field_1.load(O));
    assert_eq!(t1.field_2.load(O), t2.field_2.load(O));
    assert_eq!(t1.field_3.load(O), t2.field_3.load(O));
    assert_eq!(t1.field_4.load(O), t2.field_4.load(O));
    assert_eq!(t1.field_5.load(O), t2.field_5.load(O));
    assert_eq!(t1.field_6.load(O), t2.field_6.load(O));
    assert_eq!(t1.field_7.load(O), t2.field_7.load(O));
    assert_eq!(t1.field_8.load(O), t2.field_8.load(O));
    assert_eq!(t1.field_9.load(O), t2.field_9.load(O));
    assert_eq!(t1.field_10.load(O), t2.field_10.load(O));
    assert_eq!(t1.field_11.load(O), t2.field_11.load(O));
    assert_eq!(t1.field_12.load(O), t2.field_12.load(O));
}

#[test]
fn tuple_struct_of_atomic_primitives_works() {
    #[derive(ConstDefault, Debug, Default)]
    pub struct TestType(
        AtomicBool,
        AtomicI8,
        AtomicI16,
        AtomicI32,
        AtomicI64,
        AtomicIsize,
        AtomicU8,
        AtomicU16,
        AtomicU32,
        AtomicU64,
        AtomicUsize,
        AtomicPtr<i32>,
    );
    let t1 = <TestType as ConstDefault>::DEFAULT;
    let t2 = TestType::default();
    use Ordering::SeqCst as O;
    assert_eq!(t1.0.load(O), t2.0.load(O));
    assert_eq!(t1.1.load(O), t2.1.load(O));
    assert_eq!(t1.2.load(O), t2.2.load(O));
    assert_eq!(t1.3.load(O), t2.3.load(O));
    assert_eq!(t1.4.load(O), t2.4.load(O));
    assert_eq!(t1.5.load(O), t2.5.load(O));
    assert_eq!(t1.6.load(O), t2.6.load(O));
    assert_eq!(t1.7.load(O), t2.7.load(O));
    assert_eq!(t1.8.load(O), t2.8.load(O));
    assert_eq!(t1.9.load(O), t2.9.load(O));
    assert_eq!(t1.10.load(O), t2.10.load(O));
    assert_eq!(t1.11.load(O), t2.11.load(O));
}

#[test]
fn struct_of_structs_works() {
    #[derive(ConstDefault, Debug, Default, PartialEq)]
    pub struct OuterStruct {
        field_1: InnerStruct,
        field_2: InnerStruct,
    }
    #[derive(ConstDefault, Debug, Default, PartialEq)]
    pub struct InnerStruct {
        field_1: i32,
        field_2: u32,
    }
    assert_eq!(
        <OuterStruct as ConstDefault>::DEFAULT,
        OuterStruct::default()
    );
}

#[test]
fn tuple_struct_of_structs_works() {
    #[derive(ConstDefault, Debug, Default, PartialEq)]
    pub struct OuterStruct(InnerStruct, InnerStruct);
    #[derive(ConstDefault, Debug, Default, PartialEq)]
    pub struct InnerStruct(i32, u32);
    assert_eq!(
        <OuterStruct as ConstDefault>::DEFAULT,
        OuterStruct::default()
    );
}

#[test]
fn struct_of_cell_types_works() {
    #[derive(ConstDefault, Debug, Default, PartialEq)]
    pub struct TestStruct {
        field_1: Cell<i32>,
        field_2: RefCell<i32>,
    }
}