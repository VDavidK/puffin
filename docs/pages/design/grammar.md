# Puffin's grammar

?> Puffin's grammar is an LL(2) grammar documented in Extended Backus-Naur Form.

```ebnf
component ::= {declaration}

declaration ::= var_decl |
                signal_decl |
                method_decl |
                component_decl |
                constructor_decl |
                layout_decl |
                require_decl |
                use_decl |
                export_decl |
                enum_decl |
                error_decl;

var_decl ::= var_type, identifier, ";";

signal_decl ::= "signal", identifier, parameters, ";";

method_decl ::= [decorator], "fn", identifier, parameters, block_stat;

component_decl ::= "component", identifier, parameters, "{", {declaration}, "}";

constructor_decl ::= "new", parameters, block_stat;

layout_decl ::= "layout", {identifier}, parameters, markup_block;

require_decl ::= "require", string, ";";

use_decl ::= "use", identifier, {".", identifier}, ";";

export_decl ::= "export", var_decl | method_decl | enum_decl;

enum_decl ::= "enum", identifier, "{", [name_list], "}";

error_decl ::= "error", "{", [name_list], "}";

statement ::= if_stat |
              for_stat |
              return_stat |
              break_stat |
              throw_stat |
              continue_stat |
              raise_stat |
              match_stat |
              var_stat |
              expr_or_assign_stat;

if_stat ::= "if", expression, block_stat, ["else", if_stat | block_stat];

for_stat ::= "for", identifier, "in", expression, [":", expression], block_stat;

return_stat ::= "return", [expression];

break_stat ::= "break", ";";

throw_stat ::= "throw", expression, ";";

continue_stat ::= "continue", ";";

raise_stat ::= "raise";

match_stat ::= "match", expression, "{", {expression, "=>", block_stat | expr_stat}, ["default", [identifier], "=>", statement], "}";

var_stat ::= var_type, identifier, "=", expression, [catch_stat], ";";

expr_or_assign_stat ::= expression,
    ("=", expression, [catch_stat])
    catch_stat |
    "++" |
    "--" |
    ("+=" | "-=" | "*=" | "/="), expression
;

primary_expr ::= (literal_expr | paren_expr | list_expr | dictionary_expr), {accessor_expr | call_expr | index_expr};

accessor_expr ::= ".", identifier;

call_expr ::= "(", {expr_list}, ")";

index_expr ::= "[", expression, "]";
                
literal_expr ::= string | integer | float | boolean | identifier | null;

paren_expr ::= "(", expression, ")";

list_expr ::= "[", {expr_list}, "]"

dictionary_expr ::= "{", [dictionary_entry, {",", dictionary_entry}], "}";

dictionary_entry ::= identifier, ":", expression;

decorator ::= "@", identifier, parameters;

parameters ::= ["(", name_list, ")"];

block_stat ::= "{", {statement}, "}";

markup ::= {match_markup | if_markup | for_markup | markup_item | markup_style};

if_markup ::= "if", expression, markup_block, ["else", if_markup | markup_block];

for_markup ::= "for", identifier, "in", expression, [":", expression], markup_block;

markup_item ::= identifier, {prop}, [markup_block | expression, ";"] ;

markup_style ::= identifier, "=", expression, ";";

markup_block ::= "{", markup, "}";

prop ::= identifier, "=", primary_expr;

string ::= """, {- """} """;

integer ::= DIGIT, {DIGIT};

float ::= DIGIT, {DIGIT}, ".", DIGIT, {DIGIT};

boolean ::= "true" | "false";

identifier ::= (UNICODE_LETTER | "_"), {UNICODE_LETTER | "_" | DIGIT}

var_type ::= "let" | "const";

expr_list ::= expression, {",", expression};

name_list ::= identifier, {",", identifier};
```