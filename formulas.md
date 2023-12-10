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
