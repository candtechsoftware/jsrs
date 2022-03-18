#[derive(Default)]
pub struct Scope {
    pub outer: Box<Scope>,
    pub allow_in: bool,
    pub allow_let: bool,
    pub in_iteration: bool,
    pub in_switch: bool,
    pub declaration_list: Vec<()>,
    pub labels: Vec<String>,
}
