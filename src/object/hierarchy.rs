use super::logical_object::LogicalObject;

#[derive(Debug, Clone)]
pub struct Hierarchy {
    pub flat_list: Vec<LogicalObject>,
}