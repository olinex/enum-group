# Enum Group
`enum-group` is a simple derive macro crate that helps enum types to group their variants.
Annotate an enum with `#[derive(EnumGroup)]`,
and mark variants some group label names with `#[groups(label1, label2)]`
will auto generate function `is_label1` and `is_label2`.
These functions will tell you if a variant of the enum belongs to this grouping.

## Example
```rust
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

// Will auto generate function `fn is_odd(&self) -> bool`
assert!(Number::One.is_odd());
assert!(!Number::Two.is_odd());
assert!(Number::Three.is_odd());
assert!(!Number::Unknown(0).is_odd());

// Will auto generate function `fn is_even(&self) -> bool`
assert!(!Number::One.is_even());
assert!(Number::Two.is_even());
assert!(!Number::Three.is_even());
assert!(!Number::Unknown(0).is_even());

// Will auto generate function `fn is_prime(&self) -> bool`
assert!(!Number::One.is_prime());
assert!(Number::Two.is_prime());
assert!(Number::Three.is_prime());
assert!(!Number::Unknown(0).is_prime());
```

Sometimes, you may have a large number of group label names with the same prefix, 
and you can use this nested grouping to reduce code duplication.
It support multi-level nesting.
```rust
use enum_group::EnumGroup;

#[derive(EnumGroup)]
enum Typing {
    
    #[groups(accept(eq, ne, gt, gte, lt, lte), number)]
    I8,

    #[groups(accept(not, eq, ne))]
    Bool,
}

assert!(!Typing::I8.is_accept_not());
assert!(Typing::I8.is_accept_eq());
assert!(Typing::I8.is_accept_ne());
assert!(Typing::I8.is_accept_gt());
assert!(Typing::I8.is_accept_gte());
assert!(Typing::I8.is_accept_lt());
assert!(Typing::I8.is_accept_lte());
assert!(Typing::I8.is_number());

assert!(Typing::Bool.is_accept_not());
assert!(Typing::Bool.is_accept_eq());
assert!(Typing::Bool.is_accept_ne());
assert!(!Typing::Bool.is_accept_gt());
assert!(!Typing::Bool.is_accept_gte());
assert!(!Typing::Bool.is_accept_lt());
assert!(!Typing::Bool.is_accept_lte());
assert!(!Typing::Bool.is_number());
```

## Usage Restrictions
Each character of the group label name for each variant must be alphanumeric or `_`.

## Panic
```rust
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