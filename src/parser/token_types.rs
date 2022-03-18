pub enum Token {
    ILLEGAL,
    EOF,
    COMMENT,

    STRING,
    NUMBER,
    PLUS,      // +
    MINUS,     // -
    MULTIPLY,  // *
    SLASH,     // /
    REMAINDER, // %

    AND,                  // &
    OR,                   // |
    EXCLUSIVE_OR,         // ^
    SHIFT_LEFT,           // <<
    SHIFT_RIGHT,          // >>
    UNSIGNED_SHIFT_RIGHT, // >>>

    ADD_ASSIGN,       // +=
    SUBTRACT_ASSIGN,  // -=
    MULTIPLY_ASSIGN,  // *=
    QUOTIENT_ASSIGN,  // /=
    REMAINDER_ASSIGN, // %=

    AND_ASSIGN,                  // &=
    OR_ASSIGN,                   // |=
    EXCLUSIVE_OR_ASSIGN,         // ^=
    SHIFT_LEFT_ASSIGN,           // <<=
    SHIFT_RIGHT_ASSIGN,          // >>=
    UNSIGNED_SHIFT_RIGHT_ASSIGN, // >>>=

    LOGICAL_AND, // &&
    LOGICAL_OR,  // ||
    INCREMENT,   // ++
    DECREMENT,   // --

    EQUAL,        // ==
    STRICT_EQUAL, // ===
    LESS,         // <
    GREATER,      // >
    ASSIGN,       // =
    NOT,          // !

    BITWISE_NOT, // ~

    NOT_EQUAL,        // !=
    STRICT_NOT_EQUAL, // !==
    LESS_OR_EQUAL,    // <=
    GREATER_OR_EQUAL, // >=

    LEFT_PARENTHESIS, // (
    LEFT_BRACKET,     // [
    LEFT_BRACE,       // {
    COMMA,            // ,
    PERIOD,           // .

    RIGHT_PARENTHESIS, // )
    RIGHT_BRACKET,     // ]
    RIGHT_BRACE,       // }
    SEMICOLON,         // ;
    COLON,             // :
    QUESTION_MARK,     // ?
    QUESTION_DOT,      // ?.
    ARROW,             // =>
    ELLIPSIS,          // ...
    BACKTICK,          // `

    // tokens below (and only them) are syntactically valid identifiers
    IDENTIFIER,
    KEYWORD,
    BOOLEAN,
    NULL,

    IF,
    IN,
    OF,
    DO,
    VAR,
    LET,
    FOR,
    NEW,
    TRY,

    THIS,
    ELSE,
    CASE,
    VOID,
    WITH,

    CONST,
    WHILE,
    BREAK,
    CATCH,
    THROW,
    RETURN,
    TYPEOF,
    DELETE,
    SWITCH,

    DEFAULT,
    FINALLY,

    FUNCTION,
    CONTINUE,
    DEBUGGER,

    INSTANCEOF,
}
