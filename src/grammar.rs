pub mod expr;

// expression     → literal
//                | unary
//                | binary
//                | grouping ;

// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;

// Name	    Operators     Associates
// Equality	  == !=         Left
// Comparison	  > >= < <=     Left
// Term	       - +          Left
// Factor	       / *          Left
// Unary	        ! -         Right
