use crate::tokenizer::Token;

struct Root {
    children: Vec<Node>,
}

enum ReservedFunction {
    Input(String),
    Sleep(i32),
}

enum ValueType {
    Str(String),
    Num(i32),
}

enum Node {
    FnCall { kind: ReservedFunction },
    Loop { children: Vec<Node> },
    Increment { who: String, amount: i32 },
    DefineVariable { name: String, value: ValueType },
}

pub fn parse(tokens: Vec<Token>) -> Root {}
