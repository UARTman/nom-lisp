use crate::ast::Node;

use super::{
    error::{LispError, Result},
    Data, NSStack,
};

pub fn f_let(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    if args.len() % 2 != 0 {
        return Err(LispError::SyntaxError(
            "Variable declaration mismatch.".into(),
        ));
    }
    for i in args.chunks(2) {
        match &i[0] {
            Node::Identifier(id) => {
                let param_value = i[1].eval(stack).unwrap();
                stack.top().unwrap().insert(id.clone(), param_value);
            }
            _ => {
                return Err(LispError::TypeError(format!(
                    "{:?} is not an identifier.",
                    &i[0]
                )))
            }
        }
    }
    Ok(Data::Empty)
}

pub fn f_do(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    let mut ret = Err(LispError::SyntaxError("Empty do block".into()));
    for node in args {
        ret = node.eval(stack);
        ret.as_ref()?;
    }
    ret
}

pub fn f_if(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    if args.len() != 3 {
        Err(LispError::SyntaxError(
            "If statement should have 3 arguments.".into(),
        ))
    } else if args[0].eval(stack)?.is_truthy() {
        args[1].eval(stack)
    } else {
        args[2].eval(stack)
    }
}

pub fn f_fn(_stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    let arg = args.get(0).ok_or(LispError::SyntaxError(
        "Function declaration should get a list of arguments and a body!".into(),
    ))?;
    match arg {
        Node::List(ns) => {
            let mut arglist = Vec::new();
            for i in ns {
                match i {
                    Node::Identifier(id) => arglist.push(id.clone()),
                    _ => {
                        return Err(LispError::SyntaxError(
                            "When declaring function, all arguments should be identifiers.".into(),
                        ))
                    }
                }
            }
            let body = args.get(1).ok_or(LispError::SyntaxError(
                "Function declaration doesn't have a body!".into(),
            ))?;
            Ok(Data::Function(arglist, body.clone()))
        }
        _ => Err(LispError::SyntaxError(
            "Function arguments should be given in a list.".into(),
        )),
    }
}

pub fn quote(_stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    let node = args.get(0).ok_or(LispError::SyntaxError(
        "Quote received zero arguments.".into(),
    ))?;
    Ok(Data::Quote(node.clone()))
}

pub fn unquote(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    let node = args.get(0).ok_or(LispError::SyntaxError(
        "Quote received zero arguments.".into(),
    ))?;
    let data = node.eval(stack)?;
    match data {
        Data::Quote(n) => n.eval(stack),
        _ => Err(LispError::TypeError(format!("{:?} is not a quote.", &data))),
    }
}

pub fn debug(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    for node in args {
        let r = node.eval(stack)?;
        println!("{:?}", r);
    }
    Ok(Data::Empty)
}

pub fn eq(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    if args.len() != 2 {
        Err(LispError::SyntaxError("= only takes 2 arguments".into()))
    } else {
        let left = args[0].eval(stack)?;
        let right = args[1].eval(stack)?;
        Ok(Data::Int(if left == right { 1 } else { 0 }))
    }
}

pub fn add(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    if args.len() != 2 {
        Err(LispError::SyntaxError("+ only takes 2 arguments".into()))
    } else {
        let left = args[0].eval(stack)?;
        let right = args[1].eval(stack)?;
        match (left, right) {
            (Data::Int(a), Data::Int(b)) => Ok(Data::Int(a + b)),
            _ => Err(LispError::TypeError("You can only add integers.".into())),
        }
    }
}

pub fn sub(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    if args.len() != 2 {
        Err(LispError::SyntaxError("- only takes 2 arguments".into()))
    } else {
        let left = args[0].eval(stack)?;
        let right = args[1].eval(stack)?;
        match (left, right) {
            (Data::Int(a), Data::Int(b)) => Ok(Data::Int(a - b)),
            _ => Err(LispError::TypeError("You can only add integers.".into())),
        }
    }
}

pub fn mul(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    if args.len() != 2 {
        Err(LispError::SyntaxError("* only takes 2 arguments".into()))
    } else {
        let left = args[0].eval(stack)?;
        let right = args[1].eval(stack)?;
        match (left, right) {
            (Data::Int(a), Data::Int(b)) => Ok(Data::Int(a * b)),
            _ => Err(LispError::TypeError("You can only add integers.".into())),
        }
    }
}

pub fn div(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    if args.len() != 2 {
        Err(LispError::SyntaxError("/ only takes 2 arguments".into()))
    } else {
        let left = args[0].eval(stack)?;
        let right = args[1].eval(stack)?;
        match (left, right) {
            (Data::Int(a), Data::Int(b)) => Ok(Data::Int(a / b)),
            _ => Err(LispError::TypeError("You can only add integers.".into())),
        }
    }
}

pub fn modul(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    if args.len() != 2 {
        Err(LispError::SyntaxError("mod only takes 2 arguments".into()))
    } else {
        let left = args[0].eval(stack)?;
        let right = args[1].eval(stack)?;
        match (left, right) {
            (Data::Int(a), Data::Int(b)) => Ok(Data::Int(a % b)),
            _ => Err(LispError::TypeError("You can only add integers.".into())),
        }
    }
}

pub fn ne(stack: &mut NSStack, args: &[Node]) -> Result<Data> {
    if args.len() != 2 {
        Err(LispError::SyntaxError("= only takes 2 arguments".into()))
    } else {
        let left = args[0].eval(stack)?;
        let right = args[1].eval(stack)?;
        Ok(Data::Int(if left == right { 0 } else { 1 }))
    }
}
