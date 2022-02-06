package token

type TokenType string

// トークン
type Token struct {
	Type    TokenType
	Literal string
}

const (
	ILLEGAL = "ILLIGAL"
	EOF     = "EOF"

	IDENT = "INDENT"
	INT   = "INT"

	ASSIGN   = "="
	PLUS     = "+"
	MINUS    = "-"
	ASTERISK = "*"
	SLASH    = "/"
	BANG     = "!"

	EQ     = "=="
	NOT_EQ = "!="

	COMMA     = ","
	SEMICOLON = ";"

	LPAREN = "("
	RPAREN = ")"
	LBRACE = "{"
	RBRACE = "}"

	LT = "<"
	GT = ">"

	FUNCTION = "FUNCTION"
	LET      = "LET"

	TRUE   = "true"
	FALSE  = "false"
	IF     = "if"
	ELSE   = "else"
	RETURN = "return"
)

func New(tokenType TokenType, ch byte) Token {
	return Token{
		Type:    tokenType,
		Literal: string(ch),
	}
}

//  識別子
var keywords = map[string]TokenType{
	"fn":     FUNCTION,
	"let":    LET,
	"if":     IF,
	"else":   ELSE,
	"return": RETURN,
	"true":   TRUE,
	"false":  FALSE,
}

func LookupIdent(ident string) TokenType {
	if tok, ok := keywords[ident]; ok {
		return tok
	} else {
		return IDENT
	}
}
