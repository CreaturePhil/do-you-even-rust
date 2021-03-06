mod hello;
mod primitives;
mod types;
mod variable;
mod expressions;
mod flowcontrol;
mod function;
mod macros;
mod mods;
mod generics;
mod scope;
mod standard;

fn main() {
    hello::world();
    primitives::types();
    primitives::literals();
    primitives::tuples();
    primitives::arrays();
    types::structs();
    types::visibility();
    types::enums();
    types::constants();
    variable::bindings();
    variable::mutability();
    variable::scope();
    variable::declare();
    expressions::main();
    flowcontrol::if_else();
    flowcontrol::loops();
    flowcontrol::nested();
    flowcontrol::whiles();
    flowcontrol::fors();
    flowcontrol::matches();
    function::main();
    function::methods();
    function::closures();
    function::hof();
    macros::hello();
    macros::funk();
    macros::overload();
    mods::main();
    generics::main();
    scope::moves();
    scope::mutability();
    scope::borrow();
    scope::borrow_mutability();
    scope::alias();
    scope::lifetime();
    scope::static_lifetime();
    standard::vector();
    standard::strings();
}
