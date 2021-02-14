#[derive(Debug)]
pub enum Token {
    Number(i32),
    Operator(Op),
    Parenthesis(char)
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn precedence(&self) -> i32 {
        match self {
            Self::Add | Self::Sub => 0,
            Self::Mul | Self::Div => 1,
        }
    }

    fn apply(&self, arg1: i32, arg2: i32) -> i32 {
        match self {
            Self::Add => arg1 + arg2,
            Self::Sub => arg1 - arg2,
            Self::Mul => arg1 * arg2,
            Self::Div => arg1 / arg2,
        }
    }
}

#[derive(Debug)]
enum Item {
    Number(i32),
    Operator(Op),
}

#[derive(Debug)]
pub struct BinaryExpressionTree {
    root: Item,
    left: Option<Box<BinaryExpressionTree>>,
    right: Option<Box<BinaryExpressionTree>>,
}

impl BinaryExpressionTree {
    pub fn evaluate(&self) -> i32 {
        if let Item::Number(c) = &self.root {
            *c
        } else if let Item::Operator(op) = &self.root {
            op.apply(self.left.as_ref().unwrap().evaluate(), self.right.as_ref().unwrap().evaluate())
        } else {
            0
        }
    }
}


/*
 * Convert expression to Token vector
 * TODO: Add Panic for characters that are neither part of a token or whitespace
 * TODO: Use Result<Vec<Token>> instead probably
 * TODO: Make it less shit (especially the digit listing lol)
 */
pub fn tokenize(expr: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Token::Operator(Op::Add)),
            '-' => tokens.push(Token::Operator(Op::Sub)),
            '*' => tokens.push(Token::Operator(Op::Mul)),
            '/' => tokens.push(Token::Operator(Op::Div)),
            '(' | ')' => tokens.push(Token::Parenthesis(c)),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut num_str = String::from(c);
                while let Some(d) = chars.peek() {
                    if d.is_numeric() {
                        num_str.push(*d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num_str.parse::<i32>().unwrap()));
            },
            _ => (),
        }
    }
    tokens
}

/*
* Convert Token vector to binary expression tree using the shunning yard algorithm
* RANGIERBAHNHOF
 */
pub fn to_expression_tree(tokens: Vec<Token>) -> BinaryExpressionTree {
    let mut stack: Vec<Token> = Vec::new();
    let mut trees: Vec<BinaryExpressionTree> = Vec::new();
    for t in tokens {
        match t {
            Token::Number(v) => trees.push(BinaryExpressionTree{root: Item::Number(v), left: None, right: None}),
            Token::Operator(ref c) => {
                // If precedence of t is lower than the top of the stack
                // Pop stack until t has higher precedence than top
                while stack.len() > 0 {
                    if let Token::Parenthesis(_) = stack[stack.len() - 1] {
                        break;
                    }
                    if let Token::Operator(ref op) = stack[stack.len() - 1] {
                        if op.precedence() >= c.precedence() {
                            let t2 = trees.pop().unwrap();
                            let t1 = trees.pop().unwrap();
                            if let Token::Operator(top) = stack.pop().unwrap() {
                                trees.push(BinaryExpressionTree{root: Item::Operator(top), left: Some(Box::new(t1)), right: Some(Box::new(t2))});
                            }
                        } else {
                            break;
                        }
                    }
                }
                stack.push(t);
            },
            Token::Parenthesis(c) => {
                match c {
                    '(' => stack.push(t),
                    ')' => {
                        while stack.len() > 0 {
                            if let Token::Parenthesis(_) = stack[stack.len() - 1] {
                                stack.pop();
                                break;
                            }
                            let t2 = trees.pop().unwrap();
                            let t1 = trees.pop().unwrap();
                            if let Token::Operator(op) = stack.pop().unwrap() {
                                trees.push(BinaryExpressionTree{root: Item::Operator(op), left: Some(Box::new(t1)), right: Some(Box::new(t2))});
                            }
                        }
                    },
                    _ => (),
                }
            },
        }
    }
    // No more Tokens in input -> process the remaining operators on the stack
    while stack.len() > 0 {
        println!("{:?}", trees);
        if let Token::Parenthesis(_) = stack[stack.len() - 1] {
            panic!("Parenthesis in stack after traversing all tokens");
        }
        if let Token::Operator(op) = stack.pop().unwrap() {
                let t2 = trees.pop().unwrap();
                let t1 = trees.pop().unwrap();
                trees.push(BinaryExpressionTree{root: Item::Operator(op), left: Some(Box::new(t1)), right: Some(Box::new(t2))});
        }
    }
    trees.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        // Just a bunch of expressions I hope it covers enough cases
        let expressions = vec![
            ("3", 3),
            ("3 + 4", 7),
            ("3 - 4", -1),
            ("3 * 4", 12),
            ("3 / 3", 1),
            ("6 / 3", 2),
            ("3 * (4 + 2)", 18),
            ("3 + 4 * (4 + 2)", 27),
            ("(3) * (4 + 2)", 18),
            ("(((3)))", 3),
        ];
        for (expr, res) in expressions {
            assert_eq!(to_expression_tree(tokenize(String::from(expr))).evaluate(), res);
        }
    }
}
