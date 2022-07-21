// @author:    olinex
// @time:      2022/07/19

// self mods

// use other mods
use enum_group::EnumGroup;

// use self mods


#[derive(EnumGroup)]
enum TestNamedEnum {

    #[allow(dead_code)]
    #[groups(plant)]
    Sunflower{color: String, have_thorns: bool},

    #[allow(dead_code)]
    #[groups(plant)]
    Rose{color: String, have_thorns: bool},

    #[allow(dead_code)]
    #[groups(animal, reptile)]
    Tortoise{feet: i8, fur: bool, feather: bool},

    #[allow(dead_code)]
    #[groups(animal, bird)]
    Crow{feet: i8, fur: bool, feather: bool},

    #[allow(dead_code)]
    #[groups(animal, mammalian)]
    Cat{feet: i8, fur: bool, feather: bool},

    Unknown,
}

#[test]
fn test_is_plant() {
    assert!(TestNamedEnum::Sunflower { color: "yellow".into(), have_thorns: false }.is_plant());
    assert!(TestNamedEnum::Rose { color: "red".into(), have_thorns: true }.is_plant());
    assert!(!TestNamedEnum::Tortoise { feet: 4, fur: false, feather: false }.is_plant());
    assert!(!TestNamedEnum::Crow { feet: 2, fur: false, feather: true}.is_plant());
    assert!(!TestNamedEnum::Cat { feet: 4, fur: true, feather: false}.is_plant());
    assert!(!TestNamedEnum::Unknown.is_plant());
}

#[test]
fn test_is_animal() {
    assert!(!TestNamedEnum::Sunflower { color: "yellow".into(), have_thorns: false }.is_animal());
    assert!(!TestNamedEnum::Rose { color: "red".into(), have_thorns: true }.is_animal());
    assert!(TestNamedEnum::Tortoise { feet: 4, fur: false, feather: false }.is_animal());
    assert!(TestNamedEnum::Crow { feet: 2, fur: false, feather: true}.is_animal());
    assert!(TestNamedEnum::Cat { feet: 4, fur: true, feather: false}.is_animal());
    assert!(!TestNamedEnum::Unknown.is_animal());
}

#[test]
fn test_is_reptile() {
    assert!(!TestNamedEnum::Sunflower { color: "yellow".into(), have_thorns: false }.is_reptile());
    assert!(!TestNamedEnum::Rose { color: "red".into(), have_thorns: true }.is_reptile());
    assert!(TestNamedEnum::Tortoise { feet: 4, fur: false, feather: false }.is_reptile());
    assert!(!TestNamedEnum::Crow { feet: 2, fur: false, feather: true}.is_reptile());
    assert!(!TestNamedEnum::Cat { feet: 4, fur: true, feather: false}.is_reptile());
    assert!(!TestNamedEnum::Unknown.is_reptile());
}

#[test]
fn test_is_bird() {
    assert!(!TestNamedEnum::Sunflower { color: "yellow".into(), have_thorns: false }.is_bird());
    assert!(!TestNamedEnum::Rose { color: "red".into(), have_thorns: true }.is_bird());
    assert!(!TestNamedEnum::Tortoise { feet: 4, fur: false, feather: false }.is_bird());
    assert!(TestNamedEnum::Crow { feet: 2, fur: false, feather: true}.is_bird());
    assert!(!TestNamedEnum::Cat { feet: 4, fur: true, feather: false}.is_bird());
    assert!(!TestNamedEnum::Unknown.is_bird());
}

#[test]
fn test_is_mammalian() {
    assert!(!TestNamedEnum::Sunflower { color: "yellow".into(), have_thorns: false }.is_mammalian());
    assert!(!TestNamedEnum::Rose { color: "red".into(), have_thorns: true }.is_mammalian());
    assert!(!TestNamedEnum::Tortoise { feet: 4, fur: false, feather: false }.is_mammalian());
    assert!(!TestNamedEnum::Crow { feet: 2, fur: false, feather: true}.is_mammalian());
    assert!(TestNamedEnum::Cat { feet: 4, fur: true, feather: false}.is_mammalian());
    assert!(!TestNamedEnum::Unknown.is_mammalian());
}