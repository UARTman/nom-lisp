use std::{collections::HashMap, fmt::Debug};

use crate::ast::Node;

mod error;
mod intrinsic;

use error::Result;

use self::error::LispError;

pub type Namespace = HashMap<String, Data>;
pub type IntrinsicRef = &'static dyn Fn(&mut NSStack, &[Node]) -> Result<Data>;
pub struct NSStack {
    spaces: Vec<Namespace>,
}

impl NSStack {
    pub fn new() -> Self {
        NSStack {
            spaces: vec![Namespace::new()],
        }
    }

    pub fn lookup(&self, name: &str) -> Result<&Data> {
        for space in self.spaces.iter().rev() {
            if let Some(d) = space.get(name) {
                return Ok(d);
            }
        }
        Err(LispError::VariableNotFound(name.into()))
    }

    pub fn lookup_mut(&mut self, name: &str) -> Result<&mut Data> {
        for space in self.spaces.iter_mut().rev() {
            if let Some(d) = space.get_mut(name) {
                return Ok(d);
            }
        }
        Err(LispError::VariableNotFound(name.into()))
    }

    pub fn enter_scope(&mut self) {
        self.spaces.push(HashMap::new())
    }

    pub fn exit_scope(&mut self) {
        self.spaces.pop();
    }

    pub fn top(&mut self) -> Result<&mut Namespace> {
        self.spaces.last_mut().ok_or(LispError::StackEmpty)
    }

    pub fn register_intrinsic(&mut self, name: &str, f: IntrinsicRef) -> Result<()> {
        let r = Data::Intrinsic(name.into(), f);
        self.spaces
            .get_mut(0)
            .ok_or(LispError::StackEmpty)?
            .insert(name.into(), r);
        Ok(())
    }
}

pub struct Runtime {
    stack: NSStack,
}

impl Runtime {
    pub fn try_new() -> Result<Self> {
        let mut stack = NSStack::new();
        stack.register_intrinsic("let", &intrinsic::f_let)?;
        stack.register_intrinsic("quote", &intrinsic::quote)?;
        stack.register_intrinsic("unquote", &intrinsic::unquote)?;
        stack.register_intrinsic("do", &intrinsic::f_do)?;
        stack.register_intrinsic("if", &intrinsic::f_if)?;
        stack.register_intrinsic("fn", &intrinsic::f_fn)?;
        stack.register_intrinsic("debug", &intrinsic::debug)?;
        Ok(Self { stack })
    }

    pub fn eval(&mut self, node: Node) -> Result<Data> {
        node.eval(&mut self.stack)
    }
}

#[derive(Clone)]
pub enum Data {
    Quote(Node),
    Int(i32),
    Str(String),
    // String()
    Intrinsic(String, IntrinsicRef),
    Function(Vec<String>, Node),
    Empty,
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Quote(l0), Self::Quote(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Str(l0), Self::Str(r0)) => l0 == r0,
            (Self::Intrinsic(l0, _), Self::Intrinsic(r0, _)) => l0 == r0,
            (Self::Function(l0, l1), Self::Function(r0, r1)) => l0 == r0 && l1 == r1,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quote(arg0) => f.debug_tuple("Quote").field(arg0).finish(),
            Self::Int(arg0) => f.debug_tuple("Int").field(arg0).finish(),
            Self::Str(arg0) => f.debug_tuple("Str").field(arg0).finish(),
            Self::Intrinsic(arg0, _) => f.debug_tuple("Intrinsic").field(arg0).finish(),
            Self::Function(arg0, arg1) => {
                f.debug_tuple("Function").field(arg0).field(arg1).finish()
            }
            Self::Empty => write!(f, "Empty"),
        }
    }
}

impl Data {
    fn exec(&self, stack: &mut NSStack, params: &[Node]) -> Result<Data> {
        match self {
            Data::Intrinsic(_, f) => f(stack, params),
            Data::Function(argnames, body) => {
                if params.len() != argnames.len() {
                    return Err(LispError::SyntaxError(
                        "Wrong function argument count.".into(),
                    ));
                }
                stack.enter_scope();
                for (i, param) in params.iter().enumerate() {
                    let param_data = param.eval(stack)?;
                    stack.top()?.insert(argnames[i].clone(), param_data.clone());
                }
                let r = body.eval(stack);
                stack.exit_scope();
                r
            }
            _ => Err(LispError::TypeError(format!("{:?} is not callable.", self))),
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Data::Quote(q) => *q == Node::Identifier("true".into()),
            Data::Int(i) => *i != 0,
            Data::Str(s) => !s.is_empty(),
            Data::Intrinsic(_, _) => false,
            Data::Function(_, _) => false,
            Data::Empty => false,
        }
    }
}

impl Node {
    pub fn eval(&self, stack: &mut NSStack) -> Result<Data> {
        Ok(match self {
            Node::Identifier(x) => stack.lookup(x)?.clone(),
            Node::List(ops) => {
                let fun = ops
                    .get(0)
                    .ok_or(LispError::SyntaxError(
                        "List expression with zero arguments.".into(),
                    ))?
                    .eval(stack)?;
                fun.exec(stack, &ops[1..])?
            }
            Node::StringLiteral(s) => Data::Str(s.clone()),
            Node::IntegerLiteral(i) => Data::Int(*i),
            Node::Quote(boxed) => Data::Quote(*boxed.clone()),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{ast::Node, runtime::Data};

    use super::{error::Result, Runtime};

    #[test]
    fn test_quote_unquote() -> Result<()> {
        let mut runtime = Runtime::try_new()?;
        let (_, node1) = crate::parser::node(b"(let quoted (quote (do 2 3)))").unwrap();
        runtime.eval(node1).unwrap();
        assert_eq!(
            &Data::Quote(Node::List(vec![
                Node::Identifier("do".into()),
                Node::IntegerLiteral(2),
                Node::IntegerLiteral(3)
            ])),
            runtime.stack.lookup("quoted")?
        );
        let (_, node2) = crate::parser::node(b"(let unquoted (unquote quoted))").unwrap();
        runtime.eval(node2).unwrap();
        assert_eq!(&Data::Int(3), runtime.stack.lookup("unquoted")?);
        Ok(())
    }
}
