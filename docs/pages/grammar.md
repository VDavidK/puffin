# Puffin's grammar

```ebnf
component ::= {declaration}

declaration ::= var_decl |
                signal_decl |
                decorated_method_decl |
                method_decl |
                component_decl |
                constructor_decl |
                layout_decl |
                require_decl |
                use_decl |
                export_decl |
                enum_decl |
                error_decl;

var_decl := var_type, identifier, ";";

signal_decl ::= "signal", identifier, parameters, ";";

decorated_method_decl ::= decorator, method_decl;

method_decl ::= "fn", identifier, parameters, block_stat;

component_decl ::= "component", identifier, parameters, "{", {declaration}, "}";

constructor_decl ::= "new", parameters, block_stat;

layout_decl ::= "layout", {identifier}, parameters, "{", markup, "}";

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

expr_or_assign_stat ::= expression, [
    ("=", expression, [catch_stat]) |
    catch_stat |
    "++" |
    "--" |
    ("+=" | "-=" | "*=" | "/=", expression)
];

decorator ::= "@", identifier, parameters;

parameters ::= ["(", name_list, ")"];

block_stat ::= "{", {statement}, "}";

markup ::= match_markup | if_markup | for_markup | markup_item | markup_style;

if_markup ::= "if", expression, markup_block, ["else", if_markup | markup_block];

for_markup ::= "for", identifier, "in", expression, [":", expression], markup_block;

markup_item ::= layout_render | component_render;

layout_render ::= identifier, "(", expr_list, ")", ";"

component_render ::= identifier, {markup_binding}, (string, ";") | markup_block;

markup_binding ::= markup_direct_binding | markup_lambda_binding;

markup_direct_binding ::= identifier, "=", ("[", [name_list], "]") | identifier;

markup_lambda_binding ::= identifier, "(", [name_list], ")", "{", expr_list, "}";

markup_style ::= identifier, "=", expression, ";";

markup_block ::= "{", markup, "}";

string ::= """, {- """} """;

var_type ::= "let" | "const";

expr_list ::= expression, {",", expression};

name_list ::= identifier, {",", identifier};

identifier ::= (UNICODE_LETTER | "_"), {UNICODE_LETTER | "_" | DIGIT}
```