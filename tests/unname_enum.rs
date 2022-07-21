// @author:    olinex
// @time:      2022/07/19

// self mods

// use other mods
use enum_group::EnumGroup;

// use self mods


#[derive(EnumGroup)]
enum TestUnnameEnum {
    #[groups(number, integer, order, comparable)]
    I8(i8),
    #[groups(number, integer, order, comparable)]
    I16(i16),
    #[groups(number, float, order, comparable)]
    Float32(f32),
    #[groups(string, comparable)]
    Str(String),
    Unknown,
}

#[test]
fn test_is_number() {
    assert!(TestUnnameEnum::I8(0i8).is_number());
    assert!(TestUnnameEnum::I16(0i16).is_number());
    assert!(TestUnnameEnum::Float32(0f32).is_number());
    assert!(!TestUnnameEnum::Str(String::new()).is_number());
    assert!(!TestUnnameEnum::Unknown.is_number());
}

#[test]
fn test_is_integer() {
    assert!(TestUnnameEnum::I8(0i8).is_integer());
    assert!(TestUnnameEnum::I16(0i16).is_integer());
    assert!(!TestUnnameEnum::Float32(0f32).is_integer());
    assert!(!TestUnnameEnum::Str(String::new()).is_integer());
    assert!(!TestUnnameEnum::Unknown.is_number());
}

#[test]
fn test_is_order() {
    assert!(TestUnnameEnum::I8(0i8).is_order());
    assert!(TestUnnameEnum::I16(0i16).is_order());
    assert!(TestUnnameEnum::Float32(0f32).is_order());
    assert!(!TestUnnameEnum::Str(String::new()).is_order());
    assert!(!TestUnnameEnum::Unknown.is_number());
}

#[test]
fn test_is_comparable() {
    assert!(TestUnnameEnum::I8(0i8).is_comparable());
    assert!(TestUnnameEnum::I16(0i16).is_comparable());
    assert!(TestUnnameEnum::Float32(0f32).is_comparable());
    assert!(TestUnnameEnum::Str(String::new()).is_comparable());
    assert!(!TestUnnameEnum::Unknown.is_number());
}