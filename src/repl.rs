pub enum State {
    On,
    Off
}

pub struct Repl {
    state: State
}

pub fn new(state:State) -> Repl {
    Repl {
        state: state,
    }
}
