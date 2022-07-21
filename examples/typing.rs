// @author:    olinex
// @time:      2022/07/18

// self mods

// use other mods
use enum_group::EnumGroup;

// use self mods

#[allow(dead_code)]
#[derive(Debug, EnumGroup)]
pub enum Typing {
    #[groups(integer, comparable, ordering)]
    I8 = 1,
    #[groups(integer, comparable, ordering)]
    I16 = 2,
    #[groups(comparable)]
    Bool = 3,
    #[groups(comparable, ordering)]
    Str = 4,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprint!("{:#?}", Typing::I8.is_integer());
    Ok(())
}


