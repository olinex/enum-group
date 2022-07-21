# Enum Group
`enum-group` is a simple derive macro crate that helps enum types to group their variants.
Annotate an enum with `#[derive(EnumGroup)]`,
and mark variants some group label names with `#[groups(label1, label2)]`
will auto generate function `is_label1` and `is_label2`.
These functions will tell you if a variant of the enum belongs to this grouping.

## Example
```
use enum_group::EnumGroup;

#[derive(EnumGroup)]
enum Number {
    #[groups(odd)]
    One,

    #[groups(even, prime)]
    Two,

    #[groups(odd, prime)]
    Three,

    Unknown(usize)
}

// will auto generate function `fn is_odd(&self) -> bool`
assert!(Number::One.is_odd());
assert!(!Number::Two.is_odd());
assert!(Number::Three.is_odd());
assert!(!Number::Unknown(0).is_odd());

// will auto generate function `fn is_even(&self) -> bool`
assert!(!Number::One.is_even());
assert!(Number::Two.is_even());
assert!(!Number::Three.is_even());
assert!(!Number::Unknown(0).is_even());

// will auto generate function `fn is_prime(&self) -> bool`
assert!(!Number::One.is_prime());
assert!(Number::Two.is_prime());
assert!(Number::Three.is_prime());
assert!(!Number::Unknown(0).is_prime());
```

## Usage Restrictions
Each character of the group label name for each variant must be alphanumeric or `_`.

## Panic
```
use enum_group::EnumGroup;

#[derive(EnumGroup)]
enum Number {
    // #[groups(odd_&)] groups attribute ident can only contain the characters a-zA-Z0-9_
    // #[groups(_odd)] groups attribute ident must starts wtih characters a-zA-Z
    // #[groups(odd_)] groups attribute ident must ends wtih characters a-zA-Z
    // #[groups()] must have group ident in groups attribute
    One,
}
```