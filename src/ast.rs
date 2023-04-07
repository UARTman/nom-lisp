#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Node {
    Identifier(String),
    List(Vec<Node>),
    StringLiteral(String),
    IntegerLiteral(i32),
    Quote(Box<Node>),
}

// impl Node {
//     pub fn is_identifier(&self) -> bool {
//         matches!(self, Self::Identifier(_))
//     }

//     pub fn is_list(&self) -> bool {
//         matches!(self, Self::List(_))
//     }

//     pub fn is_stringl(&self) -> bool {
//         matches!(self, Self::StringLiteral(_))
//     }

//     pub fn is_intl(&self) -> bool {
//         matches!(self, Self::IntegerLiteral(_))
//     }
// }
