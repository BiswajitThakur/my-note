use std::{
    collections::VecDeque,
    io::{self, ErrorKind},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Minus,
    Plus,
    Percentage,
    Slash,
    Star,
    Caret,
}

impl Precedence {
    fn get_priority(&self) -> u8 {
        match self {
            Self::Minus | Self::Plus => 1,
            Self::Percentage => 2,
            Self::Star | Self::Slash => 3,
            Self::Caret => 4,
        }
    }
}

impl TryFrom<char> for Precedence {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '-' => Ok(Self::Minus),
            '+' => Ok(Self::Plus),
            '%' => Ok(Self::Percentage),
            '/' => Ok(Self::Slash),
            '*' => Ok(Self::Star),
            '^' => Ok(Self::Caret),
            _ => Err(()),
        }
    }
}

fn infix_to_postfix<T: AsRef<str>>(value: T) -> io::Result<String> {
    let mut stack = Vec::new();
    let mut exp = Vec::new();
    let mut chars = value.as_ref().chars();
    while let Some(item) = chars.next() {
        match item {
            '(' => stack.push(item),
            ')' => {
                while let Some(v) = stack.pop() {
                    match v {
                        '(' => {
                            stack.push(v);
                            break;
                        }
                        _ => exp.push(v),
                    }
                }
                if stack.is_empty() {
                    return Err(io::Error::new(
                        ErrorKind::InvalidData,
                        "Incorrect Expression",
                    ));
                }
                stack.pop();
            }
            v if is_operator(v) => {
                let op = Precedence::try_from(v).unwrap();
                while let Some(v) = stack.pop() {
                    if !is_operator(v) {
                        stack.push(v);
                        break;
                    }
                    let top_op_stack = Precedence::try_from(v).unwrap();
                    if top_op_stack.get_priority() < op.get_priority() {
                        stack.push(v);
                        break;
                    }
                    exp.push(v);
                }
                stack.push(v);
            }
            v => {
                exp.push(v);
            }
        }
    }
    while let Some(item) = stack.pop() {
        match item {
            '(' => {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    "Incorrect Expression",
                ))
            }
            v => exp.push(v),
        }
    }
    Ok(exp.into_iter().collect())
}

fn infix_to_prefix<T: AsRef<str>>(value: T) -> io::Result<String> {
    let mut chars = value.as_ref().chars();
    let mut stack = Vec::new();
    let mut expr = Vec::new();
    while let Some(item) = chars.next() {
        match item {
            v @ '(' => {
                stack.push(v);
            }
            ')' => {
                while let Some(item) = stack.pop() {
                    match item {
                        v @ '(' => {
                            stack.push(v);
                            break;
                        }
                        v => {
                            let item1 = expr.pop();
                            if item1.is_none() {
                                return Err(io::Error::new(
                                    ErrorKind::InvalidData,
                                    "Incorrect Expression",
                                ));
                            }
                            let item2 = expr.pop();
                            if item2.is_none() {
                                return Err(io::Error::new(
                                    ErrorKind::InvalidData,
                                    "Incorrect Expression",
                                ));
                            }
                            expr.push(v);
                            expr.push(item1.unwrap());
                            expr.push(item2.unwrap());
                            if stack.pop().is_none() {
                                return Err(io::Error::new(
                                    ErrorKind::InvalidData,
                                    "Incorrect Expression",
                                ));
                            }
                        }
                    }
                }
            }
            v if is_operator(v) => {
                let op = Precedence::try_from(v).unwrap();
                while let Some(v) = stack.pop() {
                    if !is_operator(v) {
                        stack.push(v);
                        break;
                    }
                    let top_op_stack = Precedence::try_from(v).unwrap();
                    if top_op_stack.get_priority() < op.get_priority() {
                        stack.push(v);
                        break;
                    }
                    expr.push(v);
                }
                stack.push(v);
            }
            v => {
                expr.push(v);
            }
        }
    }
    while let Some(item) = stack.pop() {
        match item {
            '(' => {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    "Incorrect Expression",
                ))
            }
            v => expr.push(v),
        }
    }
    Ok(expr.into_iter().collect())
}

fn is_operator(ch: char) -> bool {
    matches!(ch, '+' | '-' | '*' | '/' | '%' | '^')
}

fn precedence(op: char) -> usize {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn infix_to_prefix_2<T: AsRef<str>>(expr: T) -> io::Result<String> {
    let mut operators: Vec<char> = Vec::new();
    let mut result: VecDeque<char> = VecDeque::new();
    let mut rev_expr = expr.as_ref().chars().rev();

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::notation::infix_to_prefix;

    use super::infix_to_postfix;

    fn test<T: AsRef<str>>(args: Vec<(T, T)>) {
        for (index, (left, right)) in args.into_iter().enumerate() {
            let left = left.as_ref();
            let right = right.as_ref();
            let got = infix_to_postfix(left).unwrap();
            if got.as_str() != right {
                eprintln!("Index: {}", index);
            }
            assert_eq!(got.as_str(), right);
        }
    }

    #[test]
    fn test_infix_to_postfix() {
        let input = vec![
            ("a+b", "ab+"),
            ("(a-b)*(c/d)", "ab-cd/*"),
            ("a+b-c*d", "ab+cd*-"),
            ("a+(b-(c+d))/e", "abcd+-e/+"),
        ];
        test(input);
    }

    #[test]
    fn test_infix_to_prefix() {
        let value = "a+b";
        assert_eq!(infix_to_prefix(value).unwrap(), "".to_owned());
    }
}
