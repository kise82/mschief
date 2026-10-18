# Grammar Specification

## Expressions

```ebnf
program
    = expression ;

expression
    = logicalOr ;

logicalOr
    = logicalAnd , { "||" , logicalAnd } ;

logicalAnd
    = equality , { "&&" , equality } ;

equality
    = relational , { equalityOp , relational } ;


relational
    = term , { relationalOp , term } ;

term
    = factor , { termOp , factor } ;

factor
    = unary , { factorOp , unary } ;

unary
    = { prefixOp } , primary ;

primary
    = "(", expression, ")"
    | literal
    ;

literal
    = Integer
    | Float
    | String
    | Boolean
    | Variable
    ;
```

## Operators

```ebnf
logicalOp
    = "&&"
    | "||"
    ;

equalityOp
    = "=="
    | "!="
    ;

relationalOp
    = "<"
    | "<="
    | ">"
    | ">="
    ;

termOp
    = "+"
    | "-"
    ;

factorOp
    = "*"
    | "/"
    ;

prefixOp
    = "+"
    | "-"
    ;
````
