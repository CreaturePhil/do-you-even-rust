mod hello;
mod primitives;
mod types;
mod variable;
mod expressions;

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
}
