#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Variable {
    pub name: String,
    pub content: String,
    pub content_quoted: String,
    pub can_be_type: CanBeType
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CanBeType {
    pub can_be_int: bool,
    pub can_be_uint: bool,
    pub can_be_float: bool,
}