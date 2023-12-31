# Formulas
Evaluating formulas requires a flexible solution for parsing out operands and operators, handling precedences, and evaluating down to a single value from formulas with potentially many terms. One approach to this is to define syntax rules and associated tokens then parse the formulas and construct a tree structure for representing these tokens and ultimately evaluate the expression by walking the tree. Start small first, just focus on literals (numeric and cell locations that contain numerics) and binary expressions (+/-) and only handle numeric values (always return `Real(...)` if the expression is valid)

## Syntax
| rule | definition | description |
|-|-|-|
| `num` | | numeric literal|
| `loc` | | cell location (`CellLoc`) |
| `lit` | `<num>` \| `<loc>` | literal value |
| `binop` | `+` \| `-` | binary operator |
| `binexpr` | `<expr><binop><expr>` | binary expression | 
| `expr` | `<lit>` \| `<binexpr>` | basic expression, can be a literal or binary expression |

## Parsing
Examples for how some expressions should be parsed:
| expression | tokens |
|-|-|
| `=1` | `<lit>` |
| | `<expr>` |
| `=1+2` | `<lit><binop><lit>` |
| | `<expr><binop><expr>` |
| | `<binexpr>` |
| | `<expr>` |
| `=1+2+3` | `<lit><binop><lit><binop><lit>` |
| | `<expr><binop><expr><binop><expr>` |
| | `<binexpr><binop><expr>` |
| | `<binexpr>` |
| | `<expr>` |

Parse tree for expression `=1+2+3`
```
            <binop>
              "+"   
             /   \
            /     \
        <binop>   <lit>
          "+"      "3"
         /   \
        /     \
     <lit>   <lit>
      "1"     "2"
```

### Parse Example in Code
The formula: `=1+A1+2+B2+3`

Gets parsed into a list of tokens:
```rust
[
    Num(1.0), 
    BinOp("+"),
    Loc(CellLoc { col: "A", row: 1 }), 
    BinOp("+"), 
    Num(2.0), 
    BinOp("+"), 
    Loc(CellLoc { col: "B", row: 2 }), 
    BinOp("+"), 
    Num(3.0)
]
```

Which makes a parse tree:
```rust
TknTree { 
    root: Some(TknNode { 
        token: BinOp("+"), 
        left: Some(TknNode { 
            token: BinOp("+"), 
            left: Some(TknNode { 
                token: BinOp("+"), 
                left: Some(TknNode { 
                    token: BinOp("+"), 
                    left: Some(TknNode { 
                        token: Num(1.0), 
                        left: None, 
                        right: None 
                    }), 
                    right: Some(TknNode { 
                        token: Loc(CellLoc { col: "A", row: 1 }), 
                        left: None, 
                        right: None 
                    }) 
                }), 
                right: Some(TknNode {
                    token: Num(2.0), 
                    left: None, 
                    right: None 
                }) 
            }), 
            right: Some(TknNode { 
                token: Loc(CellLoc { col: "B", row: 2 }), 
                left: None, 
                right: None 
            }) 
        }), 
        right: Some(TknNode { 
            token: Num(3.0), 
            left: None, 
            right: None 
        }) 
    }) 
}
```


## Evaluation
* for a `<lit>` eval returns the value
* for a `<binexpr>` eval returns the result of its operator applied to the values from its two operands

## Implementation Strategy
- basic datatypes for tokens parsed from an expression
- parsing function that parses an expression and creates tokens
- take a vector of tokens and convert into a tree structure
- compute the value of the expression by evaluating the tree from the bottom up



