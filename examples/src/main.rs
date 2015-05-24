mod hello;
mod primitives;
mod types;

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
}
