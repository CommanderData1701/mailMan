pub enum Action {
    MoveTo(String),
    Forward(String, Box<Action>),
    Empty,
    Delete,
}
