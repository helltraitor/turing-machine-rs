use turing_machine_rs::instruction::State;

// Very small test to see if the state source code is broken
#[test]
fn addition() {
    assert_eq!(State(10), State(3) + State(7));
}

#[test]
fn assign_addition() {
    let mut state = State(3);
    state += State(7);
    assert_eq!(State(10), state);
}
