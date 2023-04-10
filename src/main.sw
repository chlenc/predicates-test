predicate;
use std::inputs::input_owner;

fn main(address: Address) -> bool {
    input_owner(0).unwrap() == address || input_owner(1).unwrap() == address
}
