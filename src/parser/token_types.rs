pub enum Token {
    Illegal,
    EOF,
    Comment,

    String,
    Number,
    Plus,      // +
    Minus,     // -
    Multiply,  // *
    Slash,     // /
    Remainder, // %

    And,                // &
    Or,                 // |
    ExclusiveOr,        // ^
    ShiftLeft,          // <<
    ShiftRight,         // >>
    UnsignedShiftRight, // >>>

    AddAssign,       // +=
    SubtractAssign,  // -=
    MultiplyAssign,  // *=
    QuotientAssign,  // /=
    RemainderAssign, // %=

    AndAssign,                // &=
    OrAssign,                 // |=
    ExclusiveOrAssign,        // ^=
    ShiftLeftAssign,          // <<=
    ShiftRightAssign,         // >>=
    UnsignedShiftRightAssign, // >>>=

    LogicalAnd, // &&
    LogicalOr,  // ||
    Increment,  // ++
    Decrement,  // --

    Equal,       // ==
    StrictEqual, // ===
    Less,        // <
    Greater,     // >
    Assign,      // =
    Not,         // !

    BitwiseNot, // ~

    NotEqual,       // !=
    StrictNotEqual, // !==
    Lessorequal,    // <=
    Greaterorequal, // >=

    Leftparenthesis, // (
    Leftbracket,     // [
    Leftbrace,       // {
    Comma,           // ,
    Period,          // .

    RightParenthesis, // )
    RightBracket,     // ]
    RightBrace,       // }
    Semicolon,        // ;
    Colon,            // :
    QuestionMark,     // ?
    QuestionDot,      // ?.
    Arrow,            // =>
    Ellipsis,         // ...
    Backtick,         // `

    // Tokens Below (and Only Them) Are Syntactically Valid Identifiers
    Identifier,
    Keyword,
    Boolean,
    Null,

    If,
    In,
    Of,
    Do,
    Var,
    Let,
    For,
    New,
    Try,

    This,
    Else,
    Case,
    Void,
    With,

    Const,
    While,
    Break,
    Catch,
    Throw,
    Return,
    Typeof,
    Delete,
    Switch,

    DefaultToken,
    Finally,

    Function,
    Continue,
    Debugger,

    Instanceof,
}

impl Default for Token {
    fn default() -> Self {
        Token::Illegal
    }
}

impl PartialEq<usize> for Token {
    fn eq(&self, other: &usize) -> bool {
        (*self as usize).eq(other)
    }
}

impl PartialOrd<usize> for Token {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        (*self as usize).partial_cmp(other)
    }
}
