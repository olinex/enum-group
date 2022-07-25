# Enum Group

`enum-group` is a simple derive macro crate that helps enum types to group their variants.
Annotate an enum with `#[derive(EnumGroup)]`,
and mark variants some group label names with `#[groups(label1, label2)]`
will auto generate function `is_label1` and `is_label2`.
These functions will tell you if a variant of the enum belongs to this grouping.

## Example

### Group your variants

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

### Label name with prefix

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

### Variant judgment functions

If you just want to determine which variant the current variant is
and don't want to write too many cumbersome match expressions,
You can use the following functions to get help quickly.
Each variant provides a lowercase judgment function.

```rust
use enum_group::EnumGroup;

#[derive(EnumGroup)]
enum Pet {
    
    Cat,

    Dog,
}

assert!(Pet::Cat.is_cat());
assert!(!Pet::Dog.is_cat());
assert!(!Pet::Cat.is_dog());
assert!(Pet::Dog.is_dog());
```

### Other help functions

Sometimes you may want to print each variant's name string,
you can use `variant_name()` to get `&str`.

```rust
use enum_group::EnumGroup;

#[derive(EnumGroup)]
enum FooBar {
    
    Foo = 1,

    BAR = 2,
}

assert_eq!(FooBar::Foo.variant_name(), "Foo");
assert_eq!(FooBar::BAR.variant_name(), "BAR");
```

## Usage Restrictions

Each character of the group label name for each variant must be lower case alphanumeric or `_`.

### Panic

```rust,should_panic
use enum_group::EnumGroup;

#[derive(EnumGroup)]
enum Number {
    // #[groups(one)] // conflict group label name and viriant name. group name cannot equal to variant name
    // #[groups(Odd_&)] // groups attribute ident can only contain the characters a-z0-9_
    // #[groups(_odd)] // groups attribute ident must starts wtih characters a-z
    // #[groups(odd_)] // groups attribute ident must ends wtih characters a-z
    // #[groups()] // must have group ident in groups attribute
    One,
}
```
