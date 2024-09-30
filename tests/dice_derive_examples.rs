#![allow(dead_code, non_camel_case_types)]

use static_assertions::{assert_impl_all, assert_not_impl_all};

#[derive(Clone, Copy)]
struct NotDice;

#[derive(dicetest::Dice)]
struct UnitStruct;
assert_impl_all!(UnitStruct: dicetest::Dice);

#[derive(dicetest::Dice)]
struct Struct0 {}
assert_impl_all!(Struct0: dicetest::Dice);

#[derive(dicetest::Dice)]
struct TupleStruct0();
assert_impl_all!(TupleStruct0: dicetest::Dice);

#[derive(dicetest::Dice)]
struct Struct1 {
    x: u32,
}
assert_impl_all!(Struct1: dicetest::Dice);

#[derive(dicetest::Dice)]
struct TupleStruct1(u32);
assert_impl_all!(TupleStruct1: dicetest::Dice);

#[derive(dicetest::Dice)]
struct Struct2Named {
    x: u32,
    y: bool,
}
assert_impl_all!(Struct2Named: dicetest::Dice);

#[derive(dicetest::Dice)]
struct TupleStruct2(u32, bool);
assert_impl_all!(TupleStruct2: dicetest::Dice);

#[derive(dicetest::Dice)]
struct NamedStruct3 {
    x: u32,
    y: bool,
    z: char,
}
assert_impl_all!(NamedStruct3: dicetest::Dice);

#[derive(dicetest::Dice)]
struct TupleStruct3(u32, bool, char);
assert_impl_all!(TupleStruct3: dicetest::Dice);

#[derive(dicetest::Dice)]
struct Struct2WithStructs {
    struct1: TupleStruct0,
    struct2: TupleStruct2,
}
assert_impl_all!(Struct2WithStructs: dicetest::Dice);

#[derive(dicetest::Dice)]
struct Struct2Generic<A, B> {
    x: A,
    y: B,
}
assert_impl_all!(Struct2Generic<u8, bool>: dicetest::Dice);
assert_not_impl_all!(Struct2Generic<u8, NotDice>: dicetest::Dice);

#[derive(dicetest::Dice)]
struct Struct2ConstGeneric<const N: usize> {
    x: u32,
    ys: [bool; N],
}
assert_impl_all!(Struct2ConstGeneric<0>: dicetest::Dice);

#[derive(dicetest::Dice)]
struct Struct2GenericWithBounds<A: Clone, B: Clone> {
    x: A,
    y: B,
}
assert_impl_all!(Struct2GenericWithBounds<u8, bool>: dicetest::Dice);
assert_not_impl_all!(Struct2GenericWithBounds<u8, NotDice>: dicetest::Dice);

#[derive(dicetest::Dice)]
struct Struct2GenericWithBoundsAndWhereClause<A: Clone, B: Clone>
where
    A: Copy,
    B: Copy,
{
    x: A,
    y: B,
}
assert_impl_all!(Struct2GenericWithBoundsAndWhereClause<u8, bool>: dicetest::Dice);
assert_not_impl_all!(Struct2GenericWithBoundsAndWhereClause<u8, NotDice>: dicetest::Dice);

#[derive(dicetest::Dice)]
struct Struct2GenericAndConstGeneric<A, B, const N: usize> {
    x: A,
    ys: [B; N],
}
assert_impl_all!(Struct2GenericAndConstGeneric<u8, bool, 0>: dicetest::Dice);
assert_not_impl_all!(Struct2GenericAndConstGeneric<u8, NotDice, 0>: dicetest::Dice);

#[derive(dicetest::Dice)]
struct TupleStruct10(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8);

#[derive(dicetest::Dice)]
struct StructWithCow<'a> {
    x: std::borrow::Cow<'a, str>,
}
assert_impl_all!(StructWithCow<'static>: dicetest::Dice);

#[derive(dicetest::Dice)]
struct StructWithPhantomDataAndUnsizedType {
    x: std::marker::PhantomData<str>,
}
assert_impl_all!(StructWithPhantomDataAndUnsizedType : dicetest::Dice);

#[derive(dicetest::Dice)]
struct StructWithUnnecessaryDiceBound<A: dicetest::Dice> {
    a: A,
}
assert_impl_all!(StructWithUnnecessaryDiceBound<u8>: dicetest::Dice);

#[derive(dicetest::Dice)]
#[dice(bound = "A: dicetest::Dice, C: dicetest::Dice")]
struct StructWithPhantomData<A, B, C> {
    a: A,
    b: std::marker::PhantomData<B>,
    c: C,
}
assert_impl_all!(StructWithPhantomData<u32, u8, bool>: dicetest::Dice);
assert_impl_all!(StructWithPhantomData<u32, NotDice, bool>: dicetest::Dice);
assert_not_impl_all!(StructWithPhantomData<u32, NotDice, NotDice>: dicetest::Dice);
assert_not_impl_all!(StructWithPhantomData<u32, u8, NotDice>: dicetest::Dice);
assert_not_impl_all!(StructWithPhantomData<NotDice, NotDice, bool>: dicetest::Dice);
assert_not_impl_all!(StructWithPhantomData<NotDice, u8, bool>: dicetest::Dice);

#[derive(dicetest::Dice)]
#[dice(bound = "A: dicetest::Dice, C: dicetest::Dice")]
struct StructWithPhantomDataAndWhereClause<A, B, C>
where
    A: Clone,
    B: Copy,
{
    a: A,
    b: std::marker::PhantomData<B>,
    c: C,
}
assert_impl_all!(StructWithPhantomDataAndWhereClause<u32, u8, bool>: dicetest::Dice);
assert_impl_all!(StructWithPhantomDataAndWhereClause<u32, NotDice, bool>: dicetest::Dice);
assert_not_impl_all!(StructWithPhantomDataAndWhereClause<u32, NotDice, NotDice>: dicetest::Dice);
assert_not_impl_all!(StructWithPhantomDataAndWhereClause<u32, u8, NotDice>: dicetest::Dice);
assert_not_impl_all!(StructWithPhantomDataAndWhereClause<NotDice, NotDice, bool>: dicetest::Dice);
assert_not_impl_all!(StructWithPhantomDataAndWhereClause<NotDice, u8, bool>: dicetest::Dice);

#[derive(dicetest::Dice)]
struct StructWithCustomDie {
    x: u32,
    #[dice(die = dicetest::dice::just(true))]
    y: bool,
}

#[derive(dicetest::Dice)]
struct TupleStructWithCustomDie(u32, #[dice(die = dicetest::dice::just(true))] bool);

#[derive(dicetest::Dice)]
pub struct PubStruct {
    pub x: u32,
}
assert_impl_all!(PubStruct: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumUnit1 {
    A,
}
assert_impl_all!(EnumUnit1: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumUnit2 {
    A,
    B,
}
assert_impl_all!(EnumUnit2: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumUnit3 {
    A,
    B,
    C,
}
assert_impl_all!(EnumUnit3: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumTuple3 {
    A(u8),
    B(u8, bool),
    C(u8, bool, char),
}
assert_impl_all!(EnumTuple3: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumNamed3 {
    A { x: u8 },
    B { x: u8, y: bool },
    C { x: u8, y: bool, z: char },
}
assert_impl_all!(EnumNamed3: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumMixed {
    A,
    B(u8),
    C { x: u32 },
}
assert_impl_all!(EnumMixed: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumConstGeneric<const N: usize> {
    A(u32),
    B { bs: [bool; N] },
}
assert_impl_all!(EnumConstGeneric<0>: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumGeneric<A, B> {
    A(A),
    B { b: B },
}
assert_impl_all!(EnumGeneric<u8, bool>: dicetest::Dice);
assert_not_impl_all!(EnumGeneric<u8, NotDice>: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumGenericWithBounds<A: Clone, B: Clone> {
    A(A),
    B { b: B },
}
assert_impl_all!(EnumGenericWithBounds<u8, bool>: dicetest::Dice);
assert_not_impl_all!(EnumGenericWithBounds<u8, NotDice>: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumGenericWithBoundsAndWhereClause<A: Clone, B: Clone>
where
    A: Copy,
    B: Copy,
{
    A(A),
    B { b: B },
}
assert_impl_all!(EnumGenericWithBoundsAndWhereClause<u8, bool>: dicetest::Dice);
assert_not_impl_all!(EnumGenericWithBoundsAndWhereClause<u8, NotDice>: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumGenericAndConstGeneric<A, B, const N: usize> {
    A(A),
    B { bs: [B; N] },
}
assert_impl_all!(EnumGenericAndConstGeneric<u8, bool, 0>: dicetest::Dice);
assert_not_impl_all!(EnumGenericAndConstGeneric<u8, NotDice, 0>: dicetest::Dice);

#[derive(dicetest::Dice)]
struct r#struct {
    r#type: u8,
}
assert_impl_all!(r#struct: dicetest::Dice);

#[derive(dicetest::Dice)]
enum r#enum {
    r#type,
    r#for(r#struct),
    r#while { r#struct: r#struct },
}
assert_impl_all!(r#enum: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumWeighted {
    A,
    #[dice(weight = 0)]
    B,
    #[dice(weight = 2)]
    C,
}
assert_impl_all!(EnumUnit3: dicetest::Dice);

#[derive(dicetest::Dice)]
enum EnumWithCustomDie {
    A,
    B {
        x: u32,
        #[dice(die = dicetest::dice::just(true))]
        y: bool,
    },
    C(u32, #[dice(die = dicetest::dice::just(false))] bool),
}
