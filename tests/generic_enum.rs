// @author:    olinex
// @time:      2022/07/20

// self mods

// use other mods
use enum_group::EnumGroup;

// use self mods


#[derive(EnumGroup)]
enum TestGenericEnum<'a> {
    #[groups(freeze)]
    Title(&'a str),

    #[groups(freeze)]
    Author(&'a str),

    #[groups(multi_line)]
    Summary(&'a str),

    #[groups(multi_line)]
    Content(String),
}

#[test]
fn test_is_freeze() {
    assert!(TestGenericEnum::Title("hello world").is_freeze());
    assert!(TestGenericEnum::Author("beyond").is_freeze());
    assert!(!TestGenericEnum::Summary("say hello to everyone").is_freeze());
    assert!(!TestGenericEnum::Content("Hello everyone".to_string()).is_freeze());
}

#[test]
fn test_is_multi_line() {
    assert!(!TestGenericEnum::Title("hello world").is_multi_line());
    assert!(!TestGenericEnum::Author("beyond").is_multi_line());
    assert!(TestGenericEnum::Summary("say hello to everyone").is_multi_line());
    assert!(TestGenericEnum::Content("Hello everyone".to_string()).is_multi_line());
}

