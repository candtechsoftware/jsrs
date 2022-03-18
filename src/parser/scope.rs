pub struct Scope {
    outer: Box<Scope>,
    allow_in: bool,
    allow_let: bool,
    in_iteration: bool,
    in_switch: bool,
    declaration_list: Vec<()>,
    labels: Vec<String>,
}
