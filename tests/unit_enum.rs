// @author:    olinex
// @time:      2022/07/19

// self mods

// use other mods
use enum_group::EnumGroup;

// use self mods

#[derive(EnumGroup)]
enum TestUnitEnum {
    #[groups(odd)]
    One,

    #[groups(even, prime)]
    Two,

    #[groups(odd, prime)]
    Three,

    Unknown,
}

#[derive(EnumGroup)]
enum TestClikeUnitEnum {
    #[groups(odd)]
    One = 1,

    #[groups(even, prime)]
    Two = 2,

    #[groups(odd, prime)]
    Three = 3,

    Unknown = 0,
}

#[test]
fn test_is_odd() {
    assert!(TestUnitEnum::One.is_odd());
    assert!(!TestUnitEnum::Two.is_odd());
    assert!(TestUnitEnum::Three.is_odd());
    assert!(!TestUnitEnum::Unknown.is_odd());

    assert!(TestClikeUnitEnum::One.is_odd());
    assert!(!TestClikeUnitEnum::Two.is_odd());
    assert!(TestClikeUnitEnum::Three.is_odd());
    assert!(!TestClikeUnitEnum::Unknown.is_odd());
}

#[test]
fn test_is_even() {
    assert!(!TestUnitEnum::One.is_even());
    assert!(TestUnitEnum::Two.is_even());
    assert!(!TestUnitEnum::Three.is_even());
    assert!(!TestUnitEnum::Unknown.is_even());

    assert!(!TestClikeUnitEnum::One.is_even());
    assert!(TestClikeUnitEnum::Two.is_even());
    assert!(!TestClikeUnitEnum::Three.is_even());
    assert!(!TestClikeUnitEnum::Unknown.is_even());
}

#[test]
fn test_is_prime() {
    assert!(!TestUnitEnum::One.is_prime());
    assert!(TestUnitEnum::Two.is_prime());
    assert!(TestUnitEnum::Three.is_prime());
    assert!(!TestUnitEnum::Unknown.is_prime());

    assert!(!TestClikeUnitEnum::One.is_prime());
    assert!(TestClikeUnitEnum::Two.is_prime());
    assert!(TestClikeUnitEnum::Three.is_prime());
    assert!(!TestClikeUnitEnum::Unknown.is_prime());
}
