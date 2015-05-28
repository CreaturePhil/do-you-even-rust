mod hello;
mod primitives;
mod types;
mod variable;
mod expressions;
mod flowcontrol;
mod function;
mod macros;

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
}
