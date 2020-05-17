#![feature(prelude_import)]
#![allow(unused)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;

#[macro_use]
extern crate pest_derive;

#[grammar = "../grammar.pest"]
pub struct Grammar;
#[allow(non_upper_case_globals)]
const _PEST_GRAMMAR_Grammar: &'static str =
    "file                        = _{ SOI ~ item* ~ EOI }\r\n\r\nitem                        =  { struct_decl | function | const_item | impl_block | impl_trait | type_alias }\r\nconst_item                  =  { const_keyword ~ ident ~ typespec ~ assign ~ expr ~ semicolon }\r\n\r\nident                       = @{ (XID_START | \"_\" | \"$\") ~ XID_CONTINUE* }\r\n\r\n/// Whitespace and Comments\r\nWHITESPACE                  = _{ (\" \" | \"\\t\" | NEWLINE)+ }\r\nCOMMENT                     = _{ comment_block | comment_line }\r\ncomment_block               = @{ \"/*\" ~ (!\"*/\" ~ ANY)* ~ \"*/\" }\r\ncomment_line                = @{ \"//\" ~ (!NEWLINE ~ ANY)* }\r\n\r\n/// Keywords\r\nmut_keyword                 =  { \"mut\" }\r\nconst_keyword               =  { \"const\" }\r\nunsafe_keyword              =  { \"unsafe\" }\r\nself_keyword                =  { \"self\" }\r\ntrait_keyword               =  { \"trait\" }\r\n\r\nlet_leyword                 = _{ \"let\" }\r\nreturn_keyword              = _{ \"return\" }\r\nif_keyword                  = _{ \"if\" }\r\nelse_keyword                = _{ \"else\" }\r\nwhile_keyword               = _{ \"while\" }\r\nloop_keyword                = _{ \"loop\" }\r\nfor_keyword                 = _{ \"for\" }\r\nin_keyword                  = _{ \"in\" }\r\nfn_keyword                  = _{ \"fn\" }\r\nas_keyword                  = _{ \"as\" }\r\nstruct_keyword              = _{ \"struct\" }\r\nbreak_keyword               = _{ \"break\" }\r\nimpl_keyword                = _{ \"impl\" }\r\ntype_keyword                = _{ \"type\" }\r\n\r\n/// Types\r\ntypespec                    =  { colon ~ ty }\r\n\r\nty                          = _{ generic_ty | basic_ty }\r\nbasic_ty                    = _{ array_ty | tuple_ty | ptr_ty | ident }\r\ngeneric_ty                  =  { basic_ty ~ ty_args }\r\nptr_ty                      =  { \"*\" ~ (const_keyword | mut_keyword) ~ ty }\r\n\r\n/// Functions\r\n\r\n// A code block is simply a sequence of statements with an optional expression at the end\r\ncode_block                  =  { open_block ~ stmt* ~ close_block }\r\nunsafe_code_block           =  { unsafe_keyword? ~ code_block }\r\n\r\nfunction                    =  { unsafe_keyword? ~ fn_keyword ~ ident ~ function_args ~ function_return? ~ code_block }\r\nfunction_args               = _{ open_paren ~ (function_arg ~ comma)* ~ function_arg? ~ close_paren }\r\nfunction_arg                =  { binding ~ typespec }\r\nfunction_return             =  { \"->\" ~ ty }\r\nfunction_call               =  { turbofish? ~ open_paren ~ (expr ~ comma)* ~ expr? ~ close_paren }\r\nturbofish                   =  { double_colon ~ ty_args }\r\n\r\n/// Impl blocks\r\n\r\nty_param                    =  { ty }\r\nty_params                   =  { open_angle ~ (ty_param ~ comma)* ~ ty_param? ~ close_angle }\r\n\r\nty_args                     =  { open_angle ~ (ty ~ comma)* ~ ty? ~ close_angle }\r\n\r\nimpl_block                  =  { impl_keyword ~ ty_params? ~ ty ~ open_block ~ impl_item* ~ close_block }\r\nimpl_trait                  =  { impl_keyword ~ ty_params? ~ ty ~ for_keyword ~ ty ~ open_block ~ impl_item* ~ close_block }\r\n\r\nimpl_item                   = _{ function | const_item | type_alias }\r\n\r\n/// Statements\r\nstmt                        = _{ item | local | (expr ~ semicolon?) }\r\n\r\nlocal                       =  { let_leyword ~ binding ~ typespec? ~ (assign ~ expr)? ~ semicolon }\r\n\r\ntype_alias                  =  { type_keyword ~ ident ~ ty_params? ~ assign ~ ty }\r\n\r\n/// Bindings and accesses\r\nbinding                     =  { destructure | (mut_keyword? ~ ident) }\r\n\r\ndestructure                 =  { array_destructure | tuple_destructure | struct_destructure }\r\n\r\n\r\n/// Expressions\r\nexpr                        =  { cast_expr | assign_expr | return_expr | break_expr | while_loop | for_loop | basic_expr }\r\n\r\nbasic_expr                  = _{ infix | inner_expr }\r\n\r\ninner_expr                  = _{ deref_expr | dereferenceable_expr }\r\n// expressions which can be implicitely dereferenced via field access, array access or function call\r\ndereferenceable_expr        = _{\r\n                                  braced_expr\r\n                                | unsafe_code_block\r\n                                | static_access\r\n                                | if_expr\r\n                                | inf_loop\r\n                                | struct_construct_expr\r\n                                | array\r\n                                | tuple\r\n                                | reference_expr\r\n                                | prefix\r\n                                | literal\r\n                                | ident\r\n                            }\r\n\r\ncast_expr                   =  { basic_expr ~ (as_keyword ~ ty)+ }\r\nprefix                      =  { unary_operator ~ expr }\r\ninfix                       =  { inner_expr ~ (binary_operator ~ inner_expr)+ }\r\n\r\nbraced_expr                 = _{ open_paren ~ expr ~ close_paren }\r\n\r\nstatic_access               =  { ty ~ (double_colon ~ ident)+ }\r\n\r\nfield_access                =  { \".\" ~ (ident | decinteger) }\r\narray_access                =  { open_square ~ expr ~ close_square }\r\n\r\nreference_expr              =  { \"&\" ~ mut_keyword? ~ expr }\r\ndereference                 =  { \"*\" } // not silent to differentiate implicit/explicit deref\r\n\r\nimplicit_deref_expr         = _{ dereferenceable_expr ~ (field_access | array_access | function_call)+} // x.0.h().y[1].z[4].w(x).etc\r\nexplicit_deref_expr         = _{ dereference ~ inner_expr }\r\n\r\nderef_expr                  =  { implicit_deref_expr | explicit_deref_expr }\r\n\r\nreturn_expr                 =  { return_keyword ~ expr }\r\nbreak_expr                  =  { break_keyword ~ (\"\'\" ~ ident)? ~ expr? }\r\n\r\nassign_expr                 =  { (deref_expr | ident) ~ assign_operator ~ expr }\r\n\r\n/// Conditionals\r\nif_expr                     =  { if_keyword ~ expr ~ code_block ~ if_else? }\r\nif_else                     = _{ else_keyword ~ (if_expr | code_block)}\r\n\r\n/// Loops\r\nloop_label                  = _{ \"\'\" ~ ident ~ \":\" }\r\nwhile_loop                  =  { loop_label? ~ while_keyword                      ~ expr ~ code_block }\r\nfor_loop                    =  { loop_label? ~ for_keyword ~ binding ~ in_keyword ~ expr ~ code_block }\r\ninf_loop                    =  { loop_label? ~ loop_keyword                              ~ code_block }\r\n\r\n/// Arrays\r\narray                       = _{ array_lit | array_splat }\r\narray_lit                   =  { open_square ~ (expr ~ comma)*  ~ expr?      ~ close_square }\r\narray_splat                 =  { open_square ~ expr ~ semicolon ~ expr       ~ close_square }\r\narray_ty                    =  { open_square ~ ty   ~ semicolon ~ expr       ~ close_square }\r\narray_destructure           =  { open_square ~ (binding ~ comma)+ ~ binding? ~ close_square }\r\n\r\n/// Tuples\r\ntuple                       =  { open_paren ~ (expr    ~ comma)+ ~ expr?    ~ close_paren }\r\ntuple_ty                    =  { open_paren ~ (ty      ~ comma)+ ~ ty?      ~ close_paren }\r\ntuple_destructure           =  { open_paren ~ (binding ~ comma)+ ~ binding? ~ close_paren }\r\n\r\n/// Structs\r\nstruct_field                =  { ident ~ typespec }\r\nstruct_fields               = _{ (struct_field ~ comma)* ~ struct_field? }\r\nstruct_decl                 =  { struct_keyword ~ ident ~ open_block ~ struct_fields ~ close_block }\r\n\r\nstruct_construct_field      =  { ident ~ (colon ~ expr)? }\r\nstruct_construct_fields     = _{ (struct_construct_field ~ comma)* ~ struct_construct_field? }\r\nstruct_construct_expr       =  { ident ~ open_block ~ struct_construct_fields ~ close_block }\r\n\r\nstruct_destructure_field    =  { (mut_keyword ~ ident) | (ident ~ (colon ~ binding)?) }\r\nstruct_destructure_fields   = _{ (struct_destructure_field ~ comma)* ~ struct_destructure_field? }\r\nstruct_destructure          =  { ident ~ open_block ~ struct_destructure_fields ~ close_block }\r\n\r\n/// Basic symbols\r\ncolon                       = _{ \":\" }\r\nsemicolon                   = _{ \";\" }\r\ncomma                       = _{ \",\" }\r\nopen_block                  = _{ \"{\" }\r\nclose_block                 = _{ \"}\" }\r\nopen_square                 = _{ \"[\" }\r\nclose_square                = _{ \"]\" }\r\nopen_angle                  = _{ \"<\" }\r\nclose_angle                 = _{ \">\" }\r\nopen_paren                  = _{ \"(\" }\r\nclose_paren                 = _{ \")\" }\r\nlow_line                    = _{ \"_\" }\r\ndouble_colon                = _{ \"::\" }\r\n\r\n/// Literal values\r\nliteral                     =  { float | signed_integer | boolean | string }\r\n\r\nstring                      = _{ string_single_line | raw_string }\r\nstring_single_line          = _{ string_delimiter ~ string_content ~ string_delimiter }\r\nstring_content              = ${ (string_escape | !(string_delimiter | \"\\\\\") ~ ANY)* }\r\nstring_delimiter            = _{ \"\\\"\" }\r\nstring_unicode              = _{ \"u\" ~ ASCII_HEX_DIGIT{4} }\r\nstring_escape               = _{ \"\\\\\" ~ (\"\\\"\" | \"\\\\\" | \"a\" | \"b\" | \"f\" | \"n\" | \"r\" | \"t\" | \"v\" | string_unicode) }\r\n\r\nraw_string                  = ${\r\n                                \"r\" ~ PUSH(\"#\"*) ~ \"\\\"\" // push the number signs onto the stack\r\n                                ~ raw_string_interior\r\n                                ~ \"\\\"\" ~ POP            // match a quotation mark and the number signs\r\n                            }\r\n// unless the next character is a quotation mark\r\n// followed by the correct amount of number signs,\r\n// consume one character\r\nraw_string_interior         = @{ (!(\"\\\"\" ~ PEEK) ~ ANY)* }\r\n\r\nfloat                       = @{ signed_integer+ ~ \".\" ~ integer+ ~ (^\"e\" ~ signed_integer+)? }\r\n\r\nsigned_integer              = _{ (plus | minus)? ~ integer }\r\ninteger                     = _{ hexinteger | bininteger | octinteger | decinteger }\r\n\r\ndecinteger                  = @{         (ASCII_DIGIT     | low_line)+ }\r\nbininteger                  = @{ ^\"0b\" ~ (ASCII_BIN_DIGIT | low_line)+ }\r\nhexinteger                  = @{ ^\"0x\" ~ (ASCII_HEX_DIGIT | low_line)+ }\r\noctinteger                  = @{ ^\"0o\" ~ (ASCII_OCT_DIGIT | low_line)+ }\r\n\r\nboolean                     = _{ boolean_true | boolean_false }\r\nboolean_true                =  { \"true\" }\r\nboolean_false               =  { \"false\" }\r\n\r\n/// Operators\r\nmultiply                    =  { \"*\" }\r\ndivide                      =  { \"/\" }\r\nmodulus                     =  { \"%\" }\r\nplus                        =  { \"+\" }\r\nminus                       =  { \"-\" }\r\nshift_left                  =  { \"<<\" }\r\nshift_right                 =  { \">>\" }\r\nless_than                   =  { \"<\" }\r\nless_than_or_equal          =  { \"<=\" }\r\ngreater_than                =  { \">\" }\r\ngreater_than_or_equal       =  { \">=\" }\r\nnot_equal                   =  { \"!=\" }\r\nequal                       =  { \"==\" }\r\nlogical_and                 =  { \"&&\" }\r\nlogical_or                  =  { \"||\" }\r\nlogical_not                 =  { \"!\" }\r\nbitwise_and                 =  { \"&\" }\r\nbitwise_or                  =  { \"|\" }\r\nbitwise_xor                 =  { \"^\" }\r\nassign                      =  { \"=\" }\r\nrange                       =  { \"..\" }\r\n\r\nunary_operator              = _{ plus | minus | logical_not }\r\nbinary_operator             = _{\r\n                                multiply | divide | modulus |\r\n                                plus | minus |\r\n                                shift_left | shift_right |\r\n                                less_than_or_equal | less_than |\r\n                                greater_than_or_equal | greater_than |\r\n                                not_equal | equal |\r\n                                logical_and | logical_or |\r\n                                bitwise_and | bitwise_or | bitwise_xor |\r\n                                range\r\n                            }\r\n\r\nassign_operator             =  {(\r\n                                multiply | divide | modulus |\r\n                                plus | minus |\r\n                                shift_left | shift_right |\r\n                                bitwise_and | bitwise_or | bitwise_xor\r\n                            )? ~ assign}\r\n\r\n";
#[allow(dead_code, non_camel_case_types)]
pub enum Rule {
    EOI,
    file,
    item,
    const_item,
    ident,
    WHITESPACE,
    COMMENT,
    comment_block,
    comment_line,
    mut_keyword,
    const_keyword,
    unsafe_keyword,
    self_keyword,
    trait_keyword,
    let_leyword,
    return_keyword,
    if_keyword,
    else_keyword,
    while_keyword,
    loop_keyword,
    for_keyword,
    in_keyword,
    fn_keyword,
    as_keyword,
    struct_keyword,
    break_keyword,
    impl_keyword,
    type_keyword,
    typespec,
    ty,
    basic_ty,
    generic_ty,
    ptr_ty,
    code_block,
    unsafe_code_block,
    function,
    function_args,
    function_arg,
    function_return,
    function_call,
    turbofish,
    ty_param,
    ty_params,
    ty_args,
    impl_block,
    impl_trait,
    impl_item,
    stmt,
    local,
    type_alias,
    binding,
    destructure,
    expr,
    basic_expr,
    inner_expr,
    dereferenceable_expr,
    cast_expr,
    prefix,
    infix,
    braced_expr,
    static_access,
    field_access,
    array_access,
    reference_expr,
    dereference,
    implicit_deref_expr,
    explicit_deref_expr,
    deref_expr,
    return_expr,
    break_expr,
    assign_expr,
    if_expr,
    if_else,
    loop_label,
    while_loop,
    for_loop,
    inf_loop,
    array,
    array_lit,
    array_splat,
    array_ty,
    array_destructure,
    tuple,
    tuple_ty,
    tuple_destructure,
    struct_field,
    struct_fields,
    struct_decl,
    struct_construct_field,
    struct_construct_fields,
    struct_construct_expr,
    struct_destructure_field,
    struct_destructure_fields,
    struct_destructure,
    colon,
    semicolon,
    comma,
    open_block,
    close_block,
    open_square,
    close_square,
    open_angle,
    close_angle,
    open_paren,
    close_paren,
    low_line,
    double_colon,
    literal,
    string,
    string_single_line,
    string_content,
    string_delimiter,
    string_unicode,
    string_escape,
    raw_string,
    raw_string_interior,
    float,
    signed_integer,
    integer,
    decinteger,
    bininteger,
    hexinteger,
    octinteger,
    boolean,
    boolean_true,
    boolean_false,
    multiply,
    divide,
    modulus,
    plus,
    minus,
    shift_left,
    shift_right,
    less_than,
    less_than_or_equal,
    greater_than,
    greater_than_or_equal,
    not_equal,
    equal,
    logical_and,
    logical_or,
    logical_not,
    bitwise_and,
    bitwise_or,
    bitwise_xor,
    assign,
    range,
    unary_operator,
    binary_operator,
    assign_operator,
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::core::clone::Clone for Rule {
    #[inline]
    fn clone(&self) -> Rule { { *self } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::core::marker::Copy for Rule { }
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::core::fmt::Debug for Rule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Rule::EOI,) => {
                let mut debug_trait_builder = f.debug_tuple("EOI");
                debug_trait_builder.finish()
            }
            (&Rule::file,) => {
                let mut debug_trait_builder = f.debug_tuple("file");
                debug_trait_builder.finish()
            }
            (&Rule::item,) => {
                let mut debug_trait_builder = f.debug_tuple("item");
                debug_trait_builder.finish()
            }
            (&Rule::const_item,) => {
                let mut debug_trait_builder = f.debug_tuple("const_item");
                debug_trait_builder.finish()
            }
            (&Rule::ident,) => {
                let mut debug_trait_builder = f.debug_tuple("ident");
                debug_trait_builder.finish()
            }
            (&Rule::WHITESPACE,) => {
                let mut debug_trait_builder = f.debug_tuple("WHITESPACE");
                debug_trait_builder.finish()
            }
            (&Rule::COMMENT,) => {
                let mut debug_trait_builder = f.debug_tuple("COMMENT");
                debug_trait_builder.finish()
            }
            (&Rule::comment_block,) => {
                let mut debug_trait_builder = f.debug_tuple("comment_block");
                debug_trait_builder.finish()
            }
            (&Rule::comment_line,) => {
                let mut debug_trait_builder = f.debug_tuple("comment_line");
                debug_trait_builder.finish()
            }
            (&Rule::mut_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("mut_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::const_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("const_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::unsafe_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("unsafe_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::self_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("self_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::trait_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("trait_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::let_leyword,) => {
                let mut debug_trait_builder = f.debug_tuple("let_leyword");
                debug_trait_builder.finish()
            }
            (&Rule::return_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("return_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::if_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("if_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::else_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("else_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::while_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("while_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::loop_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("loop_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::for_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("for_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::in_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("in_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::fn_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("fn_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::as_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("as_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::struct_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("struct_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::break_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("break_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::impl_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("impl_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::type_keyword,) => {
                let mut debug_trait_builder = f.debug_tuple("type_keyword");
                debug_trait_builder.finish()
            }
            (&Rule::typespec,) => {
                let mut debug_trait_builder = f.debug_tuple("typespec");
                debug_trait_builder.finish()
            }
            (&Rule::ty,) => {
                let mut debug_trait_builder = f.debug_tuple("ty");
                debug_trait_builder.finish()
            }
            (&Rule::basic_ty,) => {
                let mut debug_trait_builder = f.debug_tuple("basic_ty");
                debug_trait_builder.finish()
            }
            (&Rule::generic_ty,) => {
                let mut debug_trait_builder = f.debug_tuple("generic_ty");
                debug_trait_builder.finish()
            }
            (&Rule::ptr_ty,) => {
                let mut debug_trait_builder = f.debug_tuple("ptr_ty");
                debug_trait_builder.finish()
            }
            (&Rule::code_block,) => {
                let mut debug_trait_builder = f.debug_tuple("code_block");
                debug_trait_builder.finish()
            }
            (&Rule::unsafe_code_block,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("unsafe_code_block");
                debug_trait_builder.finish()
            }
            (&Rule::function,) => {
                let mut debug_trait_builder = f.debug_tuple("function");
                debug_trait_builder.finish()
            }
            (&Rule::function_args,) => {
                let mut debug_trait_builder = f.debug_tuple("function_args");
                debug_trait_builder.finish()
            }
            (&Rule::function_arg,) => {
                let mut debug_trait_builder = f.debug_tuple("function_arg");
                debug_trait_builder.finish()
            }
            (&Rule::function_return,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("function_return");
                debug_trait_builder.finish()
            }
            (&Rule::function_call,) => {
                let mut debug_trait_builder = f.debug_tuple("function_call");
                debug_trait_builder.finish()
            }
            (&Rule::turbofish,) => {
                let mut debug_trait_builder = f.debug_tuple("turbofish");
                debug_trait_builder.finish()
            }
            (&Rule::ty_param,) => {
                let mut debug_trait_builder = f.debug_tuple("ty_param");
                debug_trait_builder.finish()
            }
            (&Rule::ty_params,) => {
                let mut debug_trait_builder = f.debug_tuple("ty_params");
                debug_trait_builder.finish()
            }
            (&Rule::ty_args,) => {
                let mut debug_trait_builder = f.debug_tuple("ty_args");
                debug_trait_builder.finish()
            }
            (&Rule::impl_block,) => {
                let mut debug_trait_builder = f.debug_tuple("impl_block");
                debug_trait_builder.finish()
            }
            (&Rule::impl_trait,) => {
                let mut debug_trait_builder = f.debug_tuple("impl_trait");
                debug_trait_builder.finish()
            }
            (&Rule::impl_item,) => {
                let mut debug_trait_builder = f.debug_tuple("impl_item");
                debug_trait_builder.finish()
            }
            (&Rule::stmt,) => {
                let mut debug_trait_builder = f.debug_tuple("stmt");
                debug_trait_builder.finish()
            }
            (&Rule::local,) => {
                let mut debug_trait_builder = f.debug_tuple("local");
                debug_trait_builder.finish()
            }
            (&Rule::type_alias,) => {
                let mut debug_trait_builder = f.debug_tuple("type_alias");
                debug_trait_builder.finish()
            }
            (&Rule::binding,) => {
                let mut debug_trait_builder = f.debug_tuple("binding");
                debug_trait_builder.finish()
            }
            (&Rule::destructure,) => {
                let mut debug_trait_builder = f.debug_tuple("destructure");
                debug_trait_builder.finish()
            }
            (&Rule::expr,) => {
                let mut debug_trait_builder = f.debug_tuple("expr");
                debug_trait_builder.finish()
            }
            (&Rule::basic_expr,) => {
                let mut debug_trait_builder = f.debug_tuple("basic_expr");
                debug_trait_builder.finish()
            }
            (&Rule::inner_expr,) => {
                let mut debug_trait_builder = f.debug_tuple("inner_expr");
                debug_trait_builder.finish()
            }
            (&Rule::dereferenceable_expr,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("dereferenceable_expr");
                debug_trait_builder.finish()
            }
            (&Rule::cast_expr,) => {
                let mut debug_trait_builder = f.debug_tuple("cast_expr");
                debug_trait_builder.finish()
            }
            (&Rule::prefix,) => {
                let mut debug_trait_builder = f.debug_tuple("prefix");
                debug_trait_builder.finish()
            }
            (&Rule::infix,) => {
                let mut debug_trait_builder = f.debug_tuple("infix");
                debug_trait_builder.finish()
            }
            (&Rule::braced_expr,) => {
                let mut debug_trait_builder = f.debug_tuple("braced_expr");
                debug_trait_builder.finish()
            }
            (&Rule::static_access,) => {
                let mut debug_trait_builder = f.debug_tuple("static_access");
                debug_trait_builder.finish()
            }
            (&Rule::field_access,) => {
                let mut debug_trait_builder = f.debug_tuple("field_access");
                debug_trait_builder.finish()
            }
            (&Rule::array_access,) => {
                let mut debug_trait_builder = f.debug_tuple("array_access");
                debug_trait_builder.finish()
            }
            (&Rule::reference_expr,) => {
                let mut debug_trait_builder = f.debug_tuple("reference_expr");
                debug_trait_builder.finish()
            }
            (&Rule::dereference,) => {
                let mut debug_trait_builder = f.debug_tuple("dereference");
                debug_trait_builder.finish()
            }
            (&Rule::implicit_deref_expr,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("implicit_deref_expr");
                debug_trait_builder.finish()
            }
            (&Rule::explicit_deref_expr,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("explicit_deref_expr");
                debug_trait_builder.finish()
            }
            (&Rule::deref_expr,) => {
                let mut debug_trait_builder = f.debug_tuple("deref_expr");
                debug_trait_builder.finish()
            }
            (&Rule::return_expr,) => {
                let mut debug_trait_builder = f.debug_tuple("return_expr");
                debug_trait_builder.finish()
            }
            (&Rule::break_expr,) => {
                let mut debug_trait_builder = f.debug_tuple("break_expr");
                debug_trait_builder.finish()
            }
            (&Rule::assign_expr,) => {
                let mut debug_trait_builder = f.debug_tuple("assign_expr");
                debug_trait_builder.finish()
            }
            (&Rule::if_expr,) => {
                let mut debug_trait_builder = f.debug_tuple("if_expr");
                debug_trait_builder.finish()
            }
            (&Rule::if_else,) => {
                let mut debug_trait_builder = f.debug_tuple("if_else");
                debug_trait_builder.finish()
            }
            (&Rule::loop_label,) => {
                let mut debug_trait_builder = f.debug_tuple("loop_label");
                debug_trait_builder.finish()
            }
            (&Rule::while_loop,) => {
                let mut debug_trait_builder = f.debug_tuple("while_loop");
                debug_trait_builder.finish()
            }
            (&Rule::for_loop,) => {
                let mut debug_trait_builder = f.debug_tuple("for_loop");
                debug_trait_builder.finish()
            }
            (&Rule::inf_loop,) => {
                let mut debug_trait_builder = f.debug_tuple("inf_loop");
                debug_trait_builder.finish()
            }
            (&Rule::array,) => {
                let mut debug_trait_builder = f.debug_tuple("array");
                debug_trait_builder.finish()
            }
            (&Rule::array_lit,) => {
                let mut debug_trait_builder = f.debug_tuple("array_lit");
                debug_trait_builder.finish()
            }
            (&Rule::array_splat,) => {
                let mut debug_trait_builder = f.debug_tuple("array_splat");
                debug_trait_builder.finish()
            }
            (&Rule::array_ty,) => {
                let mut debug_trait_builder = f.debug_tuple("array_ty");
                debug_trait_builder.finish()
            }
            (&Rule::array_destructure,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("array_destructure");
                debug_trait_builder.finish()
            }
            (&Rule::tuple,) => {
                let mut debug_trait_builder = f.debug_tuple("tuple");
                debug_trait_builder.finish()
            }
            (&Rule::tuple_ty,) => {
                let mut debug_trait_builder = f.debug_tuple("tuple_ty");
                debug_trait_builder.finish()
            }
            (&Rule::tuple_destructure,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("tuple_destructure");
                debug_trait_builder.finish()
            }
            (&Rule::struct_field,) => {
                let mut debug_trait_builder = f.debug_tuple("struct_field");
                debug_trait_builder.finish()
            }
            (&Rule::struct_fields,) => {
                let mut debug_trait_builder = f.debug_tuple("struct_fields");
                debug_trait_builder.finish()
            }
            (&Rule::struct_decl,) => {
                let mut debug_trait_builder = f.debug_tuple("struct_decl");
                debug_trait_builder.finish()
            }
            (&Rule::struct_construct_field,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("struct_construct_field");
                debug_trait_builder.finish()
            }
            (&Rule::struct_construct_fields,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("struct_construct_fields");
                debug_trait_builder.finish()
            }
            (&Rule::struct_construct_expr,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("struct_construct_expr");
                debug_trait_builder.finish()
            }
            (&Rule::struct_destructure_field,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("struct_destructure_field");
                debug_trait_builder.finish()
            }
            (&Rule::struct_destructure_fields,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("struct_destructure_fields");
                debug_trait_builder.finish()
            }
            (&Rule::struct_destructure,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("struct_destructure");
                debug_trait_builder.finish()
            }
            (&Rule::colon,) => {
                let mut debug_trait_builder = f.debug_tuple("colon");
                debug_trait_builder.finish()
            }
            (&Rule::semicolon,) => {
                let mut debug_trait_builder = f.debug_tuple("semicolon");
                debug_trait_builder.finish()
            }
            (&Rule::comma,) => {
                let mut debug_trait_builder = f.debug_tuple("comma");
                debug_trait_builder.finish()
            }
            (&Rule::open_block,) => {
                let mut debug_trait_builder = f.debug_tuple("open_block");
                debug_trait_builder.finish()
            }
            (&Rule::close_block,) => {
                let mut debug_trait_builder = f.debug_tuple("close_block");
                debug_trait_builder.finish()
            }
            (&Rule::open_square,) => {
                let mut debug_trait_builder = f.debug_tuple("open_square");
                debug_trait_builder.finish()
            }
            (&Rule::close_square,) => {
                let mut debug_trait_builder = f.debug_tuple("close_square");
                debug_trait_builder.finish()
            }
            (&Rule::open_angle,) => {
                let mut debug_trait_builder = f.debug_tuple("open_angle");
                debug_trait_builder.finish()
            }
            (&Rule::close_angle,) => {
                let mut debug_trait_builder = f.debug_tuple("close_angle");
                debug_trait_builder.finish()
            }
            (&Rule::open_paren,) => {
                let mut debug_trait_builder = f.debug_tuple("open_paren");
                debug_trait_builder.finish()
            }
            (&Rule::close_paren,) => {
                let mut debug_trait_builder = f.debug_tuple("close_paren");
                debug_trait_builder.finish()
            }
            (&Rule::low_line,) => {
                let mut debug_trait_builder = f.debug_tuple("low_line");
                debug_trait_builder.finish()
            }
            (&Rule::double_colon,) => {
                let mut debug_trait_builder = f.debug_tuple("double_colon");
                debug_trait_builder.finish()
            }
            (&Rule::literal,) => {
                let mut debug_trait_builder = f.debug_tuple("literal");
                debug_trait_builder.finish()
            }
            (&Rule::string,) => {
                let mut debug_trait_builder = f.debug_tuple("string");
                debug_trait_builder.finish()
            }
            (&Rule::string_single_line,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("string_single_line");
                debug_trait_builder.finish()
            }
            (&Rule::string_content,) => {
                let mut debug_trait_builder = f.debug_tuple("string_content");
                debug_trait_builder.finish()
            }
            (&Rule::string_delimiter,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("string_delimiter");
                debug_trait_builder.finish()
            }
            (&Rule::string_unicode,) => {
                let mut debug_trait_builder = f.debug_tuple("string_unicode");
                debug_trait_builder.finish()
            }
            (&Rule::string_escape,) => {
                let mut debug_trait_builder = f.debug_tuple("string_escape");
                debug_trait_builder.finish()
            }
            (&Rule::raw_string,) => {
                let mut debug_trait_builder = f.debug_tuple("raw_string");
                debug_trait_builder.finish()
            }
            (&Rule::raw_string_interior,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("raw_string_interior");
                debug_trait_builder.finish()
            }
            (&Rule::float,) => {
                let mut debug_trait_builder = f.debug_tuple("float");
                debug_trait_builder.finish()
            }
            (&Rule::signed_integer,) => {
                let mut debug_trait_builder = f.debug_tuple("signed_integer");
                debug_trait_builder.finish()
            }
            (&Rule::integer,) => {
                let mut debug_trait_builder = f.debug_tuple("integer");
                debug_trait_builder.finish()
            }
            (&Rule::decinteger,) => {
                let mut debug_trait_builder = f.debug_tuple("decinteger");
                debug_trait_builder.finish()
            }
            (&Rule::bininteger,) => {
                let mut debug_trait_builder = f.debug_tuple("bininteger");
                debug_trait_builder.finish()
            }
            (&Rule::hexinteger,) => {
                let mut debug_trait_builder = f.debug_tuple("hexinteger");
                debug_trait_builder.finish()
            }
            (&Rule::octinteger,) => {
                let mut debug_trait_builder = f.debug_tuple("octinteger");
                debug_trait_builder.finish()
            }
            (&Rule::boolean,) => {
                let mut debug_trait_builder = f.debug_tuple("boolean");
                debug_trait_builder.finish()
            }
            (&Rule::boolean_true,) => {
                let mut debug_trait_builder = f.debug_tuple("boolean_true");
                debug_trait_builder.finish()
            }
            (&Rule::boolean_false,) => {
                let mut debug_trait_builder = f.debug_tuple("boolean_false");
                debug_trait_builder.finish()
            }
            (&Rule::multiply,) => {
                let mut debug_trait_builder = f.debug_tuple("multiply");
                debug_trait_builder.finish()
            }
            (&Rule::divide,) => {
                let mut debug_trait_builder = f.debug_tuple("divide");
                debug_trait_builder.finish()
            }
            (&Rule::modulus,) => {
                let mut debug_trait_builder = f.debug_tuple("modulus");
                debug_trait_builder.finish()
            }
            (&Rule::plus,) => {
                let mut debug_trait_builder = f.debug_tuple("plus");
                debug_trait_builder.finish()
            }
            (&Rule::minus,) => {
                let mut debug_trait_builder = f.debug_tuple("minus");
                debug_trait_builder.finish()
            }
            (&Rule::shift_left,) => {
                let mut debug_trait_builder = f.debug_tuple("shift_left");
                debug_trait_builder.finish()
            }
            (&Rule::shift_right,) => {
                let mut debug_trait_builder = f.debug_tuple("shift_right");
                debug_trait_builder.finish()
            }
            (&Rule::less_than,) => {
                let mut debug_trait_builder = f.debug_tuple("less_than");
                debug_trait_builder.finish()
            }
            (&Rule::less_than_or_equal,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("less_than_or_equal");
                debug_trait_builder.finish()
            }
            (&Rule::greater_than,) => {
                let mut debug_trait_builder = f.debug_tuple("greater_than");
                debug_trait_builder.finish()
            }
            (&Rule::greater_than_or_equal,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("greater_than_or_equal");
                debug_trait_builder.finish()
            }
            (&Rule::not_equal,) => {
                let mut debug_trait_builder = f.debug_tuple("not_equal");
                debug_trait_builder.finish()
            }
            (&Rule::equal,) => {
                let mut debug_trait_builder = f.debug_tuple("equal");
                debug_trait_builder.finish()
            }
            (&Rule::logical_and,) => {
                let mut debug_trait_builder = f.debug_tuple("logical_and");
                debug_trait_builder.finish()
            }
            (&Rule::logical_or,) => {
                let mut debug_trait_builder = f.debug_tuple("logical_or");
                debug_trait_builder.finish()
            }
            (&Rule::logical_not,) => {
                let mut debug_trait_builder = f.debug_tuple("logical_not");
                debug_trait_builder.finish()
            }
            (&Rule::bitwise_and,) => {
                let mut debug_trait_builder = f.debug_tuple("bitwise_and");
                debug_trait_builder.finish()
            }
            (&Rule::bitwise_or,) => {
                let mut debug_trait_builder = f.debug_tuple("bitwise_or");
                debug_trait_builder.finish()
            }
            (&Rule::bitwise_xor,) => {
                let mut debug_trait_builder = f.debug_tuple("bitwise_xor");
                debug_trait_builder.finish()
            }
            (&Rule::assign,) => {
                let mut debug_trait_builder = f.debug_tuple("assign");
                debug_trait_builder.finish()
            }
            (&Rule::range,) => {
                let mut debug_trait_builder = f.debug_tuple("range");
                debug_trait_builder.finish()
            }
            (&Rule::unary_operator,) => {
                let mut debug_trait_builder = f.debug_tuple("unary_operator");
                debug_trait_builder.finish()
            }
            (&Rule::binary_operator,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("binary_operator");
                debug_trait_builder.finish()
            }
            (&Rule::assign_operator,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("assign_operator");
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(dead_code, non_camel_case_types)]
impl ::core::marker::StructuralEq for Rule { }
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::core::cmp::Eq for Rule {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::core::hash::Hash for Rule {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match (&*self,) {
            _ => {
                ::core::hash::Hash::hash(&unsafe {
                                              ::core::intrinsics::discriminant_value(self)
                                          }, state)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::core::cmp::Ord for Rule {
    #[inline]
    fn cmp(&self, other: &Rule) -> ::core::cmp::Ordering {
        {
            let __self_vi =
                unsafe { ::core::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::core::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) { _ => ::core::cmp::Ordering::Equal, }
            } else { __self_vi.cmp(&__arg_1_vi) }
        }
    }
}
#[allow(dead_code, non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for Rule { }
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::core::cmp::PartialEq for Rule {
    #[inline]
    fn eq(&self, other: &Rule) -> bool {
        {
            let __self_vi =
                unsafe { ::core::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::core::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) { _ => true, }
            } else { false }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::core::cmp::PartialOrd for Rule {
    #[inline]
    fn partial_cmp(&self, other: &Rule)
     -> ::core::option::Option<::core::cmp::Ordering> {
        {
            let __self_vi =
                unsafe { ::core::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::core::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ =>
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                }
            } else { __self_vi.partial_cmp(&__arg_1_vi) }
        }
    }
}
#[allow(clippy :: all)]
impl ::pest::Parser<Rule> for Grammar {
    fn parse<'i>(rule: Rule, input: &'i str)
     ->
         ::std::result::Result<::pest::iterators::Pairs<'i, Rule>,
                               ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic {
                        state.sequence(|state|
                                           {
                                               state.repeat(|state|
                                                                super::visible::WHITESPACE(state)).and_then(|state|
                                                                                                                {
                                                                                                                    state.repeat(|state|
                                                                                                                                     {
                                                                                                                                         state.sequence(|state|
                                                                                                                                                            {
                                                                                                                                                                super::visible::COMMENT(state).and_then(|state|
                                                                                                                                                                                                            {
                                                                                                                                                                                                                state.repeat(|state|
                                                                                                                                                                                                                                 super::visible::WHITESPACE(state))
                                                                                                                                                                                                            })
                                                                                                                                                            })
                                                                                                                                     })
                                                                                                                })
                                           })
                    } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn file(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           self::SOI(state).and_then(|state|
                                                                         {
                                                                             super::hidden::skip(state)
                                                                         }).and_then(|state|
                                                                                         {
                                                                                             state.sequence(|state|
                                                                                                                {
                                                                                                                    state.optional(|state|
                                                                                                                                       {
                                                                                                                                           state.restore_on_err(|state|
                                                                                                                                                                    self::item(state)).and_then(|state|
                                                                                                                                                                                                    {
                                                                                                                                                                                                        state.repeat(|state|
                                                                                                                                                                                                                         {
                                                                                                                                                                                                                             state.sequence(|state|
                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                    super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                         self::item(state))
                                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                })
                                                                                                                                                                                                                         })
                                                                                                                                                                                                    })
                                                                                                                                       })
                                                                                                                })
                                                                                         }).and_then(|state|
                                                                                                         {
                                                                                                             super::hidden::skip(state)
                                                                                                         }).and_then(|state|
                                                                                                                         {
                                                                                                                             self::EOI(state)
                                                                                                                         })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn item(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::item,
                               |state|
                                   {
                                       state.restore_on_err(|state|
                                                                self::struct_decl(state)).or_else(|state|
                                                                                                      {
                                                                                                          state.restore_on_err(|state|
                                                                                                                                   self::function(state))
                                                                                                      }).or_else(|state|
                                                                                                                     {
                                                                                                                         state.restore_on_err(|state|
                                                                                                                                                  self::const_item(state))
                                                                                                                     }).or_else(|state|
                                                                                                                                    {
                                                                                                                                        state.restore_on_err(|state|
                                                                                                                                                                 self::impl_block(state))
                                                                                                                                    }).or_else(|state|
                                                                                                                                                   {
                                                                                                                                                       state.restore_on_err(|state|
                                                                                                                                                                                self::impl_trait(state))
                                                                                                                                                   }).or_else(|state|
                                                                                                                                                                  {
                                                                                                                                                                      state.restore_on_err(|state|
                                                                                                                                                                                               self::type_alias(state))
                                                                                                                                                                  })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn const_item(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::const_item,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::const_keyword(state).and_then(|state|
                                                                                                      {
                                                                                                          super::hidden::skip(state)
                                                                                                      }).and_then(|state|
                                                                                                                      {
                                                                                                                          self::ident(state)
                                                                                                                      }).and_then(|state|
                                                                                                                                      {
                                                                                                                                          super::hidden::skip(state)
                                                                                                                                      }).and_then(|state|
                                                                                                                                                      {
                                                                                                                                                          self::typespec(state)
                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                      {
                                                                                                                                                                          super::hidden::skip(state)
                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                      {
                                                                                                                                                                                          self::assign(state)
                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                      {
                                                                                                                                                                                                          super::hidden::skip(state)
                                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                                      {
                                                                                                                                                                                                                          self::expr(state)
                                                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                          super::hidden::skip(state)
                                                                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                          self::semicolon(state)
                                                                                                                                                                                                                                                      })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ident(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ident,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   self::XID_START(state).or_else(|state|
                                                                                                                      {
                                                                                                                          state.match_string("_")
                                                                                                                      }).or_else(|state|
                                                                                                                                     {
                                                                                                                                         state.match_string("$")
                                                                                                                                     }).and_then(|state|
                                                                                                                                                     {
                                                                                                                                                         state.repeat(|state|
                                                                                                                                                                          {
                                                                                                                                                                              self::XID_CONTINUE(state)
                                                                                                                                                                          })
                                                                                                                                                     })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic,
                                 |state|
                                     {
                                         state.sequence(|state|
                                                            {
                                                                state.match_string(" ").or_else(|state|
                                                                                                    {
                                                                                                        state.match_string("\t")
                                                                                                    }).or_else(|state|
                                                                                                                   {
                                                                                                                       self::NEWLINE(state)
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       state.repeat(|state|
                                                                                                                                                        {
                                                                                                                                                            state.match_string(" ").or_else(|state|
                                                                                                                                                                                                {
                                                                                                                                                                                                    state.match_string("\t")
                                                                                                                                                                                                }).or_else(|state|
                                                                                                                                                                                                               {
                                                                                                                                                                                                                   self::NEWLINE(state)
                                                                                                                                                                                                               })
                                                                                                                                                        })
                                                                                                                                   })
                                                            })
                                     })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic,
                                 |state|
                                     {
                                         self::comment_block(state).or_else(|state|
                                                                                {
                                                                                    self::comment_line(state)
                                                                                })
                                     })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comment_block(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::comment_block,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   state.match_string("/*").and_then(|state|
                                                                                                                         {
                                                                                                                             let strings =
                                                                                                                                 ["*/"];
                                                                                                                             state.skip_until(&strings)
                                                                                                                         }).and_then(|state|
                                                                                                                                         {
                                                                                                                                             state.match_string("*/")
                                                                                                                                         })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comment_line(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::comment_line,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   state.match_string("//").and_then(|state|
                                                                                                                         {
                                                                                                                             state.repeat(|state|
                                                                                                                                              {
                                                                                                                                                  state.sequence(|state|
                                                                                                                                                                     {
                                                                                                                                                                         state.lookahead(false,
                                                                                                                                                                                         |state|
                                                                                                                                                                                             {
                                                                                                                                                                                                 self::NEWLINE(state)
                                                                                                                                                                                             }).and_then(|state|
                                                                                                                                                                                                             {
                                                                                                                                                                                                                 self::ANY(state)
                                                                                                                                                                                                             })
                                                                                                                                                                     })
                                                                                                                                              })
                                                                                                                         })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn mut_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::mut_keyword,
                               |state| { state.match_string("mut") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn const_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::const_keyword,
                               |state| { state.match_string("const") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn unsafe_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::unsafe_keyword,
                               |state| { state.match_string("unsafe") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn self_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::self_keyword,
                               |state| { state.match_string("self") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn trait_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::trait_keyword,
                               |state| { state.match_string("trait") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn let_leyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("let")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn return_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("return")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("if")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn else_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("else")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn while_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("while")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn loop_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("loop")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn for_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("for")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn in_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("in")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn fn_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("fn")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn as_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("as")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("struct")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn break_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("break")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn impl_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("impl")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_keyword(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("type")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn typespec(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::typespec,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::colon(state).and_then(|state|
                                                                                              {
                                                                                                  super::hidden::skip(state)
                                                                                              }).and_then(|state|
                                                                                                              {
                                                                                                                  self::ty(state)
                                                                                                              })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ty(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state|
                                             self::generic_ty(state)).or_else(|state|
                                                                                  {
                                                                                      state.restore_on_err(|state|
                                                                                                               self::basic_ty(state))
                                                                                  })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn basic_ty(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state|
                                             self::array_ty(state)).or_else(|state|
                                                                                {
                                                                                    state.restore_on_err(|state|
                                                                                                             self::tuple_ty(state))
                                                                                }).or_else(|state|
                                                                                               {
                                                                                                   state.restore_on_err(|state|
                                                                                                                            self::ptr_ty(state))
                                                                                               }).or_else(|state|
                                                                                                              {
                                                                                                                  self::ident(state)
                                                                                                              })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn generic_ty(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::generic_ty,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::basic_ty(state).and_then(|state|
                                                                                                 {
                                                                                                     super::hidden::skip(state)
                                                                                                 }).and_then(|state|
                                                                                                                 {
                                                                                                                     self::ty_args(state)
                                                                                                                 })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ptr_ty(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ptr_ty,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.match_string("*").and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       self::const_keyword(state).or_else(|state|
                                                                                                                                                              {
                                                                                                                                                                  self::mut_keyword(state)
                                                                                                                                                              })
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       self::ty(state)
                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn code_block(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::code_block,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_block(state).and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       state.sequence(|state|
                                                                                                                                          {
                                                                                                                                              state.optional(|state|
                                                                                                                                                                 {
                                                                                                                                                                     state.restore_on_err(|state|
                                                                                                                                                                                              self::stmt(state)).and_then(|state|
                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                  state.repeat(|state|
                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                       state.sequence(|state|
                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                              super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                                                          state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                   self::stmt(state))
                                                                                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                                                                                          })
                                                                                                                                                                                                                                                   })
                                                                                                                                                                                                                              })
                                                                                                                                                                 })
                                                                                                                                          })
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       self::close_block(state)
                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn unsafe_code_block(state:
                                             Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::unsafe_code_block,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.optional(|state|
                                                                                 {
                                                                                     self::unsafe_keyword(state)
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     super::hidden::skip(state)
                                                                                                 }).and_then(|state|
                                                                                                                 {
                                                                                                                     self::code_block(state)
                                                                                                                 })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.optional(|state|
                                                                                 {
                                                                                     self::unsafe_keyword(state)
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     super::hidden::skip(state)
                                                                                                 }).and_then(|state|
                                                                                                                 {
                                                                                                                     self::fn_keyword(state)
                                                                                                                 }).and_then(|state|
                                                                                                                                 {
                                                                                                                                     super::hidden::skip(state)
                                                                                                                                 }).and_then(|state|
                                                                                                                                                 {
                                                                                                                                                     self::ident(state)
                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                 {
                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                 {
                                                                                                                                                                                     self::function_args(state)
                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                 {
                                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                 {
                                                                                                                                                                                                                     state.optional(|state|
                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                            state.restore_on_err(|state|
                                                                                                                                                                                                                                                                     self::function_return(state))
                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                     self::code_block(state)
                                                                                                                                                                                                                                                 })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_args(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           self::open_paren(state).and_then(|state|
                                                                                {
                                                                                    super::hidden::skip(state)
                                                                                }).and_then(|state|
                                                                                                {
                                                                                                    state.sequence(|state|
                                                                                                                       {
                                                                                                                           state.optional(|state|
                                                                                                                                              {
                                                                                                                                                  state.restore_on_err(|state|
                                                                                                                                                                           state.sequence(|state|
                                                                                                                                                                                              {
                                                                                                                                                                                                  self::function_arg(state).and_then(|state|
                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                             super::hidden::skip(state)
                                                                                                                                                                                                                                         }).and_then(|state|
                                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                                             self::comma(state)
                                                                                                                                                                                                                                                         })
                                                                                                                                                                                              })).and_then(|state|
                                                                                                                                                                                                               {
                                                                                                                                                                                                                   state.repeat(|state|
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        state.sequence(|state|
                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                               super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                                                                           state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                    state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                                                                                                                           self::function_arg(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                                                                                      super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                  }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                                                                                                      self::comma(state)
                                                                                                                                                                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                                                                                                                                                       }))
                                                                                                                                                                                                                                                                                                       })
                                                                                                                                                                                                                                                           })
                                                                                                                                                                                                                                    })
                                                                                                                                                                                                               })
                                                                                                                                              })
                                                                                                                       })
                                                                                                }).and_then(|state|
                                                                                                                {
                                                                                                                    super::hidden::skip(state)
                                                                                                                }).and_then(|state|
                                                                                                                                {
                                                                                                                                    state.optional(|state|
                                                                                                                                                       {
                                                                                                                                                           state.restore_on_err(|state|
                                                                                                                                                                                    self::function_arg(state))
                                                                                                                                                       })
                                                                                                                                }).and_then(|state|
                                                                                                                                                {
                                                                                                                                                    super::hidden::skip(state)
                                                                                                                                                }).and_then(|state|
                                                                                                                                                                {
                                                                                                                                                                    self::close_paren(state)
                                                                                                                                                                })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_arg(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function_arg,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::binding(state).and_then(|state|
                                                                                                {
                                                                                                    super::hidden::skip(state)
                                                                                                }).and_then(|state|
                                                                                                                {
                                                                                                                    self::typespec(state)
                                                                                                                })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_return(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function_return,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.match_string("->").and_then(|state|
                                                                                                    {
                                                                                                        super::hidden::skip(state)
                                                                                                    }).and_then(|state|
                                                                                                                    {
                                                                                                                        self::ty(state)
                                                                                                                    })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_call(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function_call,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.optional(|state|
                                                                                 {
                                                                                     state.restore_on_err(|state|
                                                                                                              self::turbofish(state))
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     super::hidden::skip(state)
                                                                                                 }).and_then(|state|
                                                                                                                 {
                                                                                                                     self::open_paren(state)
                                                                                                                 }).and_then(|state|
                                                                                                                                 {
                                                                                                                                     super::hidden::skip(state)
                                                                                                                                 }).and_then(|state|
                                                                                                                                                 {
                                                                                                                                                     state.sequence(|state|
                                                                                                                                                                        {
                                                                                                                                                                            state.optional(|state|
                                                                                                                                                                                               {
                                                                                                                                                                                                   state.restore_on_err(|state|
                                                                                                                                                                                                                            state.sequence(|state|
                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                   self::expr(state).and_then(|state|
                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                      super::hidden::skip(state)
                                                                                                                                                                                                                                                                                  }).and_then(|state|
                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                      self::comma(state)
                                                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                                               })).and_then(|state|
                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                    state.repeat(|state|
                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                         state.sequence(|state|
                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                                            state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                                                     state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                                                                                            self::expr(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                                                                                                                                               super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                           }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                                                                                                                                                               self::comma(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                           })
                                                                                                                                                                                                                                                                                                                                                                                                        }))
                                                                                                                                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                                                })
                                                                                                                                                                                               })
                                                                                                                                                                        })
                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                 {
                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                 {
                                                                                                                                                                                     state.optional(|state|
                                                                                                                                                                                                        {
                                                                                                                                                                                                            state.restore_on_err(|state|
                                                                                                                                                                                                                                     self::expr(state))
                                                                                                                                                                                                        })
                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                 {
                                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                 {
                                                                                                                                                                                                                     self::close_paren(state)
                                                                                                                                                                                                                 })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn turbofish(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::turbofish,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::double_colon(state).and_then(|state|
                                                                                                     {
                                                                                                         super::hidden::skip(state)
                                                                                                     }).and_then(|state|
                                                                                                                     {
                                                                                                                         self::ty_args(state)
                                                                                                                     })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ty_param(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ty_param, |state| { self::ty(state) })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ty_params(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ty_params,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_angle(state).and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       state.sequence(|state|
                                                                                                                                          {
                                                                                                                                              state.optional(|state|
                                                                                                                                                                 {
                                                                                                                                                                     state.restore_on_err(|state|
                                                                                                                                                                                              state.sequence(|state|
                                                                                                                                                                                                                 {
                                                                                                                                                                                                                     self::ty_param(state).and_then(|state|
                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                            super::hidden::skip(state)
                                                                                                                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                            self::comma(state)
                                                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                 })).and_then(|state|
                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                      state.repeat(|state|
                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                           state.sequence(|state|
                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                  super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                              state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                       state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                                              self::ty_param(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                                                                                                                                                                                                     self::comma(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                 })
                                                                                                                                                                                                                                                                                                                                                                          }))
                                                                                                                                                                                                                                                                                                                          })
                                                                                                                                                                                                                                                                              })
                                                                                                                                                                                                                                                       })
                                                                                                                                                                                                                                  })
                                                                                                                                                                 })
                                                                                                                                          })
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       state.optional(|state|
                                                                                                                                                                          {
                                                                                                                                                                              state.restore_on_err(|state|
                                                                                                                                                                                                       self::ty_param(state))
                                                                                                                                                                          })
                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                   {
                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                   {
                                                                                                                                                                                       self::close_angle(state)
                                                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ty_args(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ty_args,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_angle(state).and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       state.sequence(|state|
                                                                                                                                          {
                                                                                                                                              state.optional(|state|
                                                                                                                                                                 {
                                                                                                                                                                     state.restore_on_err(|state|
                                                                                                                                                                                              state.sequence(|state|
                                                                                                                                                                                                                 {
                                                                                                                                                                                                                     self::ty(state).and_then(|state|
                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                      super::hidden::skip(state)
                                                                                                                                                                                                                                                  }).and_then(|state|
                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                      self::comma(state)
                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                 })).and_then(|state|
                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                      state.repeat(|state|
                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                           state.sequence(|state|
                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                  super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                              state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                       state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                                              self::ty(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                                                                                                               super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                           }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                                                                                                                               self::comma(state)
                                                                                                                                                                                                                                                                                                                                                                                                                           })
                                                                                                                                                                                                                                                                                                                                                                          }))
                                                                                                                                                                                                                                                                                                                          })
                                                                                                                                                                                                                                                                              })
                                                                                                                                                                                                                                                       })
                                                                                                                                                                                                                                  })
                                                                                                                                                                 })
                                                                                                                                          })
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       state.optional(|state|
                                                                                                                                                                          {
                                                                                                                                                                              state.restore_on_err(|state|
                                                                                                                                                                                                       self::ty(state))
                                                                                                                                                                          })
                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                   {
                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                   {
                                                                                                                                                                                       self::close_angle(state)
                                                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn impl_block(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::impl_block,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::impl_keyword(state).and_then(|state|
                                                                                                     {
                                                                                                         super::hidden::skip(state)
                                                                                                     }).and_then(|state|
                                                                                                                     {
                                                                                                                         state.optional(|state|
                                                                                                                                            {
                                                                                                                                                state.restore_on_err(|state|
                                                                                                                                                                         self::ty_params(state))
                                                                                                                                            })
                                                                                                                     }).and_then(|state|
                                                                                                                                     {
                                                                                                                                         super::hidden::skip(state)
                                                                                                                                     }).and_then(|state|
                                                                                                                                                     {
                                                                                                                                                         self::ty(state)
                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                     {
                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                     {
                                                                                                                                                                                         self::open_block(state)
                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                     {
                                                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                     {
                                                                                                                                                                                                                         state.sequence(|state|
                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                state.optional(|state|
                                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                                       state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                self::impl_item(state)).and_then(|state|
                                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                                         state.repeat(|state|
                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                              state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                                                                                                                                                     super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                                                                                                                                                 state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                          self::impl_item(state))
                                                                                                                                                                                                                                                                                                                                                                                                                             })
                                                                                                                                                                                                                                                                                                                                                                                 })
                                                                                                                                                                                                                                                                                                                                                          })
                                                                                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                                                   })
                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                         self::close_block(state)
                                                                                                                                                                                                                                                     })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn impl_trait(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::impl_trait,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::impl_keyword(state).and_then(|state|
                                                                                                     {
                                                                                                         super::hidden::skip(state)
                                                                                                     }).and_then(|state|
                                                                                                                     {
                                                                                                                         state.optional(|state|
                                                                                                                                            {
                                                                                                                                                state.restore_on_err(|state|
                                                                                                                                                                         self::ty_params(state))
                                                                                                                                            })
                                                                                                                     }).and_then(|state|
                                                                                                                                     {
                                                                                                                                         super::hidden::skip(state)
                                                                                                                                     }).and_then(|state|
                                                                                                                                                     {
                                                                                                                                                         self::ty(state)
                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                     {
                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                     {
                                                                                                                                                                                         self::for_keyword(state)
                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                     {
                                                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                     {
                                                                                                                                                                                                                         self::ty(state)
                                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                         self::open_block(state)
                                                                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                         state.sequence(|state|
                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                state.optional(|state|
                                                                                                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                                                                                                       state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                                self::impl_item(state)).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                                                                                                         state.repeat(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                                                                                              state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                                                                                                                                                                                                                     super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          self::impl_item(state))
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             })
                                                                                                                                                                                                                                                                                                                                                                                                                                                 })
                                                                                                                                                                                                                                                                                                                                                                                                                          })
                                                                                                                                                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                                                                                                                   })
                                                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                         self::close_block(state)
                                                                                                                                                                                                                                                                                                                     })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn impl_item(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state|
                                             self::function(state)).or_else(|state|
                                                                                {
                                                                                    state.restore_on_err(|state|
                                                                                                             self::const_item(state))
                                                                                }).or_else(|state|
                                                                                               {
                                                                                                   state.restore_on_err(|state|
                                                                                                                            self::type_alias(state))
                                                                                               })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn stmt(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state|
                                             self::item(state)).or_else(|state|
                                                                            {
                                                                                state.restore_on_err(|state|
                                                                                                         self::local(state))
                                                                            }).or_else(|state|
                                                                                           {
                                                                                               state.restore_on_err(|state|
                                                                                                                        state.sequence(|state|
                                                                                                                                           {
                                                                                                                                               self::expr(state).and_then(|state|
                                                                                                                                                                              {
                                                                                                                                                                                  super::hidden::skip(state)
                                                                                                                                                                              }).and_then(|state|
                                                                                                                                                                                              {
                                                                                                                                                                                                  state.optional(|state|
                                                                                                                                                                                                                     {
                                                                                                                                                                                                                         self::semicolon(state)
                                                                                                                                                                                                                     })
                                                                                                                                                                                              })
                                                                                                                                           }))
                                                                                           })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn local(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::local,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::let_leyword(state).and_then(|state|
                                                                                                    {
                                                                                                        super::hidden::skip(state)
                                                                                                    }).and_then(|state|
                                                                                                                    {
                                                                                                                        self::binding(state)
                                                                                                                    }).and_then(|state|
                                                                                                                                    {
                                                                                                                                        super::hidden::skip(state)
                                                                                                                                    }).and_then(|state|
                                                                                                                                                    {
                                                                                                                                                        state.optional(|state|
                                                                                                                                                                           {
                                                                                                                                                                               state.restore_on_err(|state|
                                                                                                                                                                                                        self::typespec(state))
                                                                                                                                                                           })
                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                    {
                                                                                                                                                                        super::hidden::skip(state)
                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                    {
                                                                                                                                                                                        state.optional(|state|
                                                                                                                                                                                                           {
                                                                                                                                                                                                               state.restore_on_err(|state|
                                                                                                                                                                                                                                        state.sequence(|state|
                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                               self::assign(state).and_then(|state|
                                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                                    super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                }).and_then(|state|
                                                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                                                    self::expr(state)
                                                                                                                                                                                                                                                                                                                })
                                                                                                                                                                                                                                                           }))
                                                                                                                                                                                                           })
                                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                                    {
                                                                                                                                                                                                        super::hidden::skip(state)
                                                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        self::semicolon(state)
                                                                                                                                                                                                                    })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_alias(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::type_alias,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::type_keyword(state).and_then(|state|
                                                                                                     {
                                                                                                         super::hidden::skip(state)
                                                                                                     }).and_then(|state|
                                                                                                                     {
                                                                                                                         self::ident(state)
                                                                                                                     }).and_then(|state|
                                                                                                                                     {
                                                                                                                                         super::hidden::skip(state)
                                                                                                                                     }).and_then(|state|
                                                                                                                                                     {
                                                                                                                                                         state.optional(|state|
                                                                                                                                                                            {
                                                                                                                                                                                state.restore_on_err(|state|
                                                                                                                                                                                                         self::ty_params(state))
                                                                                                                                                                            })
                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                     {
                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                     {
                                                                                                                                                                                         self::assign(state)
                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                     {
                                                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                     {
                                                                                                                                                                                                                         self::ty(state)
                                                                                                                                                                                                                     })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn binding(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::binding,
                               |state|
                                   {
                                       self::destructure(state).or_else(|state|
                                                                            {
                                                                                state.sequence(|state|
                                                                                                   {
                                                                                                       state.optional(|state|
                                                                                                                          {
                                                                                                                              self::mut_keyword(state)
                                                                                                                          }).and_then(|state|
                                                                                                                                          {
                                                                                                                                              super::hidden::skip(state)
                                                                                                                                          }).and_then(|state|
                                                                                                                                                          {
                                                                                                                                                              self::ident(state)
                                                                                                                                                          })
                                                                                                   })
                                                                            })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn destructure(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::destructure,
                               |state|
                                   {
                                       self::array_destructure(state).or_else(|state|
                                                                                  {
                                                                                      self::tuple_destructure(state)
                                                                                  }).or_else(|state|
                                                                                                 {
                                                                                                     self::struct_destructure(state)
                                                                                                 })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expr,
                               |state|
                                   {
                                       state.restore_on_err(|state|
                                                                self::cast_expr(state)).or_else(|state|
                                                                                                    {
                                                                                                        state.restore_on_err(|state|
                                                                                                                                 self::assign_expr(state))
                                                                                                    }).or_else(|state|
                                                                                                                   {
                                                                                                                       state.restore_on_err(|state|
                                                                                                                                                self::return_expr(state))
                                                                                                                   }).or_else(|state|
                                                                                                                                  {
                                                                                                                                      state.restore_on_err(|state|
                                                                                                                                                               self::break_expr(state))
                                                                                                                                  }).or_else(|state|
                                                                                                                                                 {
                                                                                                                                                     state.restore_on_err(|state|
                                                                                                                                                                              self::while_loop(state))
                                                                                                                                                 }).or_else(|state|
                                                                                                                                                                {
                                                                                                                                                                    state.restore_on_err(|state|
                                                                                                                                                                                             self::for_loop(state))
                                                                                                                                                                }).or_else(|state|
                                                                                                                                                                               {
                                                                                                                                                                                   state.restore_on_err(|state|
                                                                                                                                                                                                            self::basic_expr(state))
                                                                                                                                                                               })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn basic_expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state|
                                             self::infix(state)).or_else(|state|
                                                                             {
                                                                                 state.restore_on_err(|state|
                                                                                                          self::inner_expr(state))
                                                                             })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn inner_expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state|
                                             self::deref_expr(state)).or_else(|state|
                                                                                  {
                                                                                      state.restore_on_err(|state|
                                                                                                               self::dereferenceable_expr(state))
                                                                                  })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dereferenceable_expr(state:
                                                Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state|
                                             self::braced_expr(state)).or_else(|state|
                                                                                   {
                                                                                       state.restore_on_err(|state|
                                                                                                                self::unsafe_code_block(state))
                                                                                   }).or_else(|state|
                                                                                                  {
                                                                                                      state.restore_on_err(|state|
                                                                                                                               self::static_access(state))
                                                                                                  }).or_else(|state|
                                                                                                                 {
                                                                                                                     state.restore_on_err(|state|
                                                                                                                                              self::if_expr(state))
                                                                                                                 }).or_else(|state|
                                                                                                                                {
                                                                                                                                    state.restore_on_err(|state|
                                                                                                                                                             self::inf_loop(state))
                                                                                                                                }).or_else(|state|
                                                                                                                                               {
                                                                                                                                                   state.restore_on_err(|state|
                                                                                                                                                                            self::struct_construct_expr(state))
                                                                                                                                               }).or_else(|state|
                                                                                                                                                              {
                                                                                                                                                                  state.restore_on_err(|state|
                                                                                                                                                                                           self::array(state))
                                                                                                                                                              }).or_else(|state|
                                                                                                                                                                             {
                                                                                                                                                                                 state.restore_on_err(|state|
                                                                                                                                                                                                          self::tuple(state))
                                                                                                                                                                             }).or_else(|state|
                                                                                                                                                                                            {
                                                                                                                                                                                                state.restore_on_err(|state|
                                                                                                                                                                                                                         self::reference_expr(state))
                                                                                                                                                                                            }).or_else(|state|
                                                                                                                                                                                                           {
                                                                                                                                                                                                               state.restore_on_err(|state|
                                                                                                                                                                                                                                        self::prefix(state))
                                                                                                                                                                                                           }).or_else(|state|
                                                                                                                                                                                                                          {
                                                                                                                                                                                                                              state.restore_on_err(|state|
                                                                                                                                                                                                                                                       self::literal(state))
                                                                                                                                                                                                                          }).or_else(|state|
                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                             self::ident(state)
                                                                                                                                                                                                                                         })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn cast_expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::cast_expr,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::basic_expr(state).and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       state.sequence(|state|
                                                                                                                                          {
                                                                                                                                              self::as_keyword(state).and_then(|state|
                                                                                                                                                                                   {
                                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                                   {
                                                                                                                                                                                                       self::ty(state)
                                                                                                                                                                                                   })
                                                                                                                                          })
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       state.sequence(|state|
                                                                                                                                                                          {
                                                                                                                                                                              state.optional(|state|
                                                                                                                                                                                                 {
                                                                                                                                                                                                     state.restore_on_err(|state|
                                                                                                                                                                                                                              state.sequence(|state|
                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                     self::as_keyword(state).and_then(|state|
                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                              super::hidden::skip(state)
                                                                                                                                                                                                                                                                                          }).and_then(|state|
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                              self::ty(state)
                                                                                                                                                                                                                                                                                                          })
                                                                                                                                                                                                                                                 })).and_then(|state|
                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                      state.repeat(|state|
                                                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                                                           state.sequence(|state|
                                                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                                                  super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                              state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                                                       state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                                                                              self::as_keyword(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                                                                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                       self::ty(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                   })
                                                                                                                                                                                                                                                                                                                                                                                                          }))
                                                                                                                                                                                                                                                                                                                                                          })
                                                                                                                                                                                                                                                                                                              })
                                                                                                                                                                                                                                                                                       })
                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                 })
                                                                                                                                                                          })
                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn prefix(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::prefix,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::unary_operator(state).and_then(|state|
                                                                                                       {
                                                                                                           super::hidden::skip(state)
                                                                                                       }).and_then(|state|
                                                                                                                       {
                                                                                                                           self::expr(state)
                                                                                                                       })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn infix(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::infix,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::inner_expr(state).and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       state.sequence(|state|
                                                                                                                                          {
                                                                                                                                              self::binary_operator(state).and_then(|state|
                                                                                                                                                                                        {
                                                                                                                                                                                            super::hidden::skip(state)
                                                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                                                        {
                                                                                                                                                                                                            self::inner_expr(state)
                                                                                                                                                                                                        })
                                                                                                                                          })
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       state.sequence(|state|
                                                                                                                                                                          {
                                                                                                                                                                              state.optional(|state|
                                                                                                                                                                                                 {
                                                                                                                                                                                                     state.restore_on_err(|state|
                                                                                                                                                                                                                              state.sequence(|state|
                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                     self::binary_operator(state).and_then(|state|
                                                                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                                                                   super::hidden::skip(state)
                                                                                                                                                                                                                                                                                               }).and_then(|state|
                                                                                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                                                                                   self::inner_expr(state)
                                                                                                                                                                                                                                                                                                               })
                                                                                                                                                                                                                                                 })).and_then(|state|
                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                      state.repeat(|state|
                                                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                                                           state.sequence(|state|
                                                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                                                  super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                              state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                                                       state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                                                                              self::binary_operator(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            self::inner_expr(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                                                                                                                                                                                                          }))
                                                                                                                                                                                                                                                                                                                                                          })
                                                                                                                                                                                                                                                                                                              })
                                                                                                                                                                                                                                                                                       })
                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                 })
                                                                                                                                                                          })
                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn braced_expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           self::open_paren(state).and_then(|state|
                                                                                {
                                                                                    super::hidden::skip(state)
                                                                                }).and_then(|state|
                                                                                                {
                                                                                                    self::expr(state)
                                                                                                }).and_then(|state|
                                                                                                                {
                                                                                                                    super::hidden::skip(state)
                                                                                                                }).and_then(|state|
                                                                                                                                {
                                                                                                                                    self::close_paren(state)
                                                                                                                                })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn static_access(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::static_access,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::ty(state).and_then(|state|
                                                                                           {
                                                                                               super::hidden::skip(state)
                                                                                           }).and_then(|state|
                                                                                                           {
                                                                                                               state.sequence(|state|
                                                                                                                                  {
                                                                                                                                      self::double_colon(state).and_then(|state|
                                                                                                                                                                             {
                                                                                                                                                                                 super::hidden::skip(state)
                                                                                                                                                                             }).and_then(|state|
                                                                                                                                                                                             {
                                                                                                                                                                                                 self::ident(state)
                                                                                                                                                                                             })
                                                                                                                                  })
                                                                                                           }).and_then(|state|
                                                                                                                           {
                                                                                                                               super::hidden::skip(state)
                                                                                                                           }).and_then(|state|
                                                                                                                                           {
                                                                                                                                               state.sequence(|state|
                                                                                                                                                                  {
                                                                                                                                                                      state.optional(|state|
                                                                                                                                                                                         {
                                                                                                                                                                                             state.sequence(|state|
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    self::double_colon(state).and_then(|state|
                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                               super::hidden::skip(state)
                                                                                                                                                                                                                                                           }).and_then(|state|
                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                               self::ident(state)
                                                                                                                                                                                                                                                                           })
                                                                                                                                                                                                                }).and_then(|state|
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    state.repeat(|state|
                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                         state.sequence(|state|
                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                            state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                                                                                                                   self::double_colon(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                                                              super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                          }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                                                                              self::ident(state)
                                                                                                                                                                                                                                                                                                                                                                                                          })
                                                                                                                                                                                                                                                                                                                                               })
                                                                                                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                })
                                                                                                                                                                                         })
                                                                                                                                                                  })
                                                                                                                                           })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn field_access(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::field_access,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.match_string(".").and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       self::ident(state).or_else(|state|
                                                                                                                                                      {
                                                                                                                                                          self::decinteger(state)
                                                                                                                                                      })
                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn array_access(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::array_access,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_square(state).and_then(|state|
                                                                                                    {
                                                                                                        super::hidden::skip(state)
                                                                                                    }).and_then(|state|
                                                                                                                    {
                                                                                                                        self::expr(state)
                                                                                                                    }).and_then(|state|
                                                                                                                                    {
                                                                                                                                        super::hidden::skip(state)
                                                                                                                                    }).and_then(|state|
                                                                                                                                                    {
                                                                                                                                                        self::close_square(state)
                                                                                                                                                    })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn reference_expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::reference_expr,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.match_string("&").and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       state.optional(|state|
                                                                                                                                          {
                                                                                                                                              self::mut_keyword(state)
                                                                                                                                          })
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       self::expr(state)
                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dereference(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::dereference,
                               |state| { state.match_string("*") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn implicit_deref_expr(state:
                                               Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           self::dereferenceable_expr(state).and_then(|state|
                                                                                          {
                                                                                              super::hidden::skip(state)
                                                                                          }).and_then(|state|
                                                                                                          {
                                                                                                              self::field_access(state).or_else(|state|
                                                                                                                                                    {
                                                                                                                                                        state.restore_on_err(|state|
                                                                                                                                                                                 self::array_access(state))
                                                                                                                                                    }).or_else(|state|
                                                                                                                                                                   {
                                                                                                                                                                       state.restore_on_err(|state|
                                                                                                                                                                                                self::function_call(state))
                                                                                                                                                                   })
                                                                                                          }).and_then(|state|
                                                                                                                          {
                                                                                                                              super::hidden::skip(state)
                                                                                                                          }).and_then(|state|
                                                                                                                                          {
                                                                                                                                              state.sequence(|state|
                                                                                                                                                                 {
                                                                                                                                                                     state.optional(|state|
                                                                                                                                                                                        {
                                                                                                                                                                                            self::field_access(state).or_else(|state|
                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                      state.restore_on_err(|state|
                                                                                                                                                                                                                                                               self::array_access(state))
                                                                                                                                                                                                                                  }).or_else(|state|
                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                     state.restore_on_err(|state|
                                                                                                                                                                                                                                                                              self::function_call(state))
                                                                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                                     state.repeat(|state|
                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                          state.sequence(|state|
                                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                                 super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                                                                                                                                             self::field_access(state).or_else(|state|
                                                                                                                                                                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                                                                                                                                                                       state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                self::array_access(state))
                                                                                                                                                                                                                                                                                                                                                                                                   }).or_else(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                                                                                                      state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                               self::function_call(state))
                                                                                                                                                                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                                                                                                                                                         })
                                                                                                                                                                                                                                                                                                             })
                                                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                                                                                 })
                                                                                                                                                                                        })
                                                                                                                                                                 })
                                                                                                                                          })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn explicit_deref_expr(state:
                                               Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           self::dereference(state).and_then(|state|
                                                                                 {
                                                                                     super::hidden::skip(state)
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     self::inner_expr(state)
                                                                                                 })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn deref_expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::deref_expr,
                               |state|
                                   {
                                       state.restore_on_err(|state|
                                                                self::implicit_deref_expr(state)).or_else(|state|
                                                                                                              {
                                                                                                                  state.restore_on_err(|state|
                                                                                                                                           self::explicit_deref_expr(state))
                                                                                                              })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn return_expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::return_expr,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::return_keyword(state).and_then(|state|
                                                                                                       {
                                                                                                           super::hidden::skip(state)
                                                                                                       }).and_then(|state|
                                                                                                                       {
                                                                                                                           self::expr(state)
                                                                                                                       })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn break_expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::break_expr,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::break_keyword(state).and_then(|state|
                                                                                                      {
                                                                                                          super::hidden::skip(state)
                                                                                                      }).and_then(|state|
                                                                                                                      {
                                                                                                                          state.optional(|state|
                                                                                                                                             {
                                                                                                                                                 state.sequence(|state|
                                                                                                                                                                    {
                                                                                                                                                                        state.match_string("\'").and_then(|state|
                                                                                                                                                                                                              {
                                                                                                                                                                                                                  super::hidden::skip(state)
                                                                                                                                                                                                              }).and_then(|state|
                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                  self::ident(state)
                                                                                                                                                                                                                              })
                                                                                                                                                                    })
                                                                                                                                             })
                                                                                                                      }).and_then(|state|
                                                                                                                                      {
                                                                                                                                          super::hidden::skip(state)
                                                                                                                                      }).and_then(|state|
                                                                                                                                                      {
                                                                                                                                                          state.optional(|state|
                                                                                                                                                                             {
                                                                                                                                                                                 state.restore_on_err(|state|
                                                                                                                                                                                                          self::expr(state))
                                                                                                                                                                             })
                                                                                                                                                      })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assign_expr,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.restore_on_err(|state|
                                                                                       self::deref_expr(state)).or_else(|state|
                                                                                                                            {
                                                                                                                                self::ident(state)
                                                                                                                            }).and_then(|state|
                                                                                                                                            {
                                                                                                                                                super::hidden::skip(state)
                                                                                                                                            }).and_then(|state|
                                                                                                                                                            {
                                                                                                                                                                self::assign_operator(state)
                                                                                                                                                            }).and_then(|state|
                                                                                                                                                                            {
                                                                                                                                                                                super::hidden::skip(state)
                                                                                                                                                                            }).and_then(|state|
                                                                                                                                                                                            {
                                                                                                                                                                                                self::expr(state)
                                                                                                                                                                                            })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_expr(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::if_expr,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::if_keyword(state).and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       self::expr(state)
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       self::code_block(state)
                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                   {
                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                   {
                                                                                                                                                                                       state.optional(|state|
                                                                                                                                                                                                          {
                                                                                                                                                                                                              state.restore_on_err(|state|
                                                                                                                                                                                                                                       self::if_else(state))
                                                                                                                                                                                                          })
                                                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_else(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           self::else_keyword(state).and_then(|state|
                                                                                  {
                                                                                      super::hidden::skip(state)
                                                                                  }).and_then(|state|
                                                                                                  {
                                                                                                      state.restore_on_err(|state|
                                                                                                                               self::if_expr(state)).or_else(|state|
                                                                                                                                                                 {
                                                                                                                                                                     state.restore_on_err(|state|
                                                                                                                                                                                              self::code_block(state))
                                                                                                                                                                 })
                                                                                                  })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn loop_label(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           state.match_string("\'").and_then(|state|
                                                                                 {
                                                                                     super::hidden::skip(state)
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     self::ident(state)
                                                                                                 }).and_then(|state|
                                                                                                                 {
                                                                                                                     super::hidden::skip(state)
                                                                                                                 }).and_then(|state|
                                                                                                                                 {
                                                                                                                                     state.match_string(":")
                                                                                                                                 })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn while_loop(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::while_loop,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.optional(|state|
                                                                                 {
                                                                                     self::loop_label(state)
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     super::hidden::skip(state)
                                                                                                 }).and_then(|state|
                                                                                                                 {
                                                                                                                     self::while_keyword(state)
                                                                                                                 }).and_then(|state|
                                                                                                                                 {
                                                                                                                                     super::hidden::skip(state)
                                                                                                                                 }).and_then(|state|
                                                                                                                                                 {
                                                                                                                                                     self::expr(state)
                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                 {
                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                 {
                                                                                                                                                                                     self::code_block(state)
                                                                                                                                                                                 })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn for_loop(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::for_loop,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.optional(|state|
                                                                                 {
                                                                                     self::loop_label(state)
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     super::hidden::skip(state)
                                                                                                 }).and_then(|state|
                                                                                                                 {
                                                                                                                     self::for_keyword(state)
                                                                                                                 }).and_then(|state|
                                                                                                                                 {
                                                                                                                                     super::hidden::skip(state)
                                                                                                                                 }).and_then(|state|
                                                                                                                                                 {
                                                                                                                                                     self::binding(state)
                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                 {
                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                 {
                                                                                                                                                                                     self::in_keyword(state)
                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                 {
                                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                 {
                                                                                                                                                                                                                     self::expr(state)
                                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                     self::code_block(state)
                                                                                                                                                                                                                                                 })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn inf_loop(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::inf_loop,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.optional(|state|
                                                                                 {
                                                                                     self::loop_label(state)
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     super::hidden::skip(state)
                                                                                                 }).and_then(|state|
                                                                                                                 {
                                                                                                                     self::loop_keyword(state)
                                                                                                                 }).and_then(|state|
                                                                                                                                 {
                                                                                                                                     super::hidden::skip(state)
                                                                                                                                 }).and_then(|state|
                                                                                                                                                 {
                                                                                                                                                     self::code_block(state)
                                                                                                                                                 })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn array(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state|
                                             self::array_lit(state)).or_else(|state|
                                                                                 {
                                                                                     state.restore_on_err(|state|
                                                                                                              self::array_splat(state))
                                                                                 })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn array_lit(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::array_lit,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_square(state).and_then(|state|
                                                                                                    {
                                                                                                        super::hidden::skip(state)
                                                                                                    }).and_then(|state|
                                                                                                                    {
                                                                                                                        state.sequence(|state|
                                                                                                                                           {
                                                                                                                                               state.optional(|state|
                                                                                                                                                                  {
                                                                                                                                                                      state.restore_on_err(|state|
                                                                                                                                                                                               state.sequence(|state|
                                                                                                                                                                                                                  {
                                                                                                                                                                                                                      self::expr(state).and_then(|state|
                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                         self::comma(state)
                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                  })).and_then(|state|
                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                       state.repeat(|state|
                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                            state.sequence(|state|
                                                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                                                   super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                               state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                        state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                                                                               self::expr(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                                                                                                                                                  super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                              }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                                                                                                                                                                  self::comma(state)
                                                                                                                                                                                                                                                                                                                                                                                                                              })
                                                                                                                                                                                                                                                                                                                                                                           }))
                                                                                                                                                                                                                                                                                                                           })
                                                                                                                                                                                                                                                                               })
                                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                                   })
                                                                                                                                                                  })
                                                                                                                                           })
                                                                                                                    }).and_then(|state|
                                                                                                                                    {
                                                                                                                                        super::hidden::skip(state)
                                                                                                                                    }).and_then(|state|
                                                                                                                                                    {
                                                                                                                                                        state.optional(|state|
                                                                                                                                                                           {
                                                                                                                                                                               state.restore_on_err(|state|
                                                                                                                                                                                                        self::expr(state))
                                                                                                                                                                           })
                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                    {
                                                                                                                                                                        super::hidden::skip(state)
                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                    {
                                                                                                                                                                                        self::close_square(state)
                                                                                                                                                                                    })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn array_splat(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::array_splat,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_square(state).and_then(|state|
                                                                                                    {
                                                                                                        super::hidden::skip(state)
                                                                                                    }).and_then(|state|
                                                                                                                    {
                                                                                                                        self::expr(state)
                                                                                                                    }).and_then(|state|
                                                                                                                                    {
                                                                                                                                        super::hidden::skip(state)
                                                                                                                                    }).and_then(|state|
                                                                                                                                                    {
                                                                                                                                                        self::semicolon(state)
                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                    {
                                                                                                                                                                        super::hidden::skip(state)
                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                    {
                                                                                                                                                                                        self::expr(state)
                                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                                    {
                                                                                                                                                                                                        super::hidden::skip(state)
                                                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        self::close_square(state)
                                                                                                                                                                                                                    })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn array_ty(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::array_ty,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_square(state).and_then(|state|
                                                                                                    {
                                                                                                        super::hidden::skip(state)
                                                                                                    }).and_then(|state|
                                                                                                                    {
                                                                                                                        self::ty(state)
                                                                                                                    }).and_then(|state|
                                                                                                                                    {
                                                                                                                                        super::hidden::skip(state)
                                                                                                                                    }).and_then(|state|
                                                                                                                                                    {
                                                                                                                                                        self::semicolon(state)
                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                    {
                                                                                                                                                                        super::hidden::skip(state)
                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                    {
                                                                                                                                                                                        self::expr(state)
                                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                                    {
                                                                                                                                                                                                        super::hidden::skip(state)
                                                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        self::close_square(state)
                                                                                                                                                                                                                    })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn array_destructure(state:
                                             Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::array_destructure,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_square(state).and_then(|state|
                                                                                                    {
                                                                                                        super::hidden::skip(state)
                                                                                                    }).and_then(|state|
                                                                                                                    {
                                                                                                                        state.sequence(|state|
                                                                                                                                           {
                                                                                                                                               state.sequence(|state|
                                                                                                                                                                  {
                                                                                                                                                                      self::binding(state).and_then(|state|
                                                                                                                                                                                                        {
                                                                                                                                                                                                            super::hidden::skip(state)
                                                                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            self::comma(state)
                                                                                                                                                                                                                        })
                                                                                                                                                                  }).and_then(|state|
                                                                                                                                                                                  {
                                                                                                                                                                                      super::hidden::skip(state)
                                                                                                                                                                                  }).and_then(|state|
                                                                                                                                                                                                  {
                                                                                                                                                                                                      state.sequence(|state|
                                                                                                                                                                                                                         {
                                                                                                                                                                                                                             state.optional(|state|
                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                    state.sequence(|state|
                                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                                           self::binding(state).and_then(|state|
                                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                                 super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                             }).and_then(|state|
                                                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                                                 self::comma(state)
                                                                                                                                                                                                                                                                                                                             })
                                                                                                                                                                                                                                                                       }).and_then(|state|
                                                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                                                           state.repeat(|state|
                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                                                                                                       super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                                                                                                                                                   state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                                                                                                                                          self::binding(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                                                                                                                                super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                            }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                self::comma(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                                                                                                                                                                                               })
                                                                                                                                                                                                                                                                                                                                   })
                                                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                                                       })
                                                                                                                                                                                                                                                })
                                                                                                                                                                                                                         })
                                                                                                                                                                                                  })
                                                                                                                                           })
                                                                                                                    }).and_then(|state|
                                                                                                                                    {
                                                                                                                                        super::hidden::skip(state)
                                                                                                                                    }).and_then(|state|
                                                                                                                                                    {
                                                                                                                                                        state.optional(|state|
                                                                                                                                                                           {
                                                                                                                                                                               self::binding(state)
                                                                                                                                                                           })
                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                    {
                                                                                                                                                                        super::hidden::skip(state)
                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                    {
                                                                                                                                                                                        self::close_square(state)
                                                                                                                                                                                    })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn tuple(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::tuple,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_paren(state).and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       state.sequence(|state|
                                                                                                                                          {
                                                                                                                                              state.sequence(|state|
                                                                                                                                                                 {
                                                                                                                                                                     self::expr(state).and_then(|state|
                                                                                                                                                                                                    {
                                                                                                                                                                                                        super::hidden::skip(state)
                                                                                                                                                                                                    }).and_then(|state|
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        self::comma(state)
                                                                                                                                                                                                                    })
                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                 {
                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                 {
                                                                                                                                                                                                     state.sequence(|state|
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            state.optional(|state|
                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                   state.restore_on_err(|state|
                                                                                                                                                                                                                                                                            state.sequence(|state|
                                                                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                                                                   self::expr(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                      super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                  }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                                      self::comma(state)
                                                                                                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                                                                                               })).and_then(|state|
                                                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                                                    state.repeat(|state|
                                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                                         state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                                                super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                                                                                            state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                     state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            self::expr(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               self::comma(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           })
                                                                                                                                                                                                                                                                                                                                                                                                                                                        }))
                                                                                                                                                                                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                                                                                                })
                                                                                                                                                                                                                                               })
                                                                                                                                                                                                                        })
                                                                                                                                                                                                 })
                                                                                                                                          })
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       state.optional(|state|
                                                                                                                                                                          {
                                                                                                                                                                              state.restore_on_err(|state|
                                                                                                                                                                                                       self::expr(state))
                                                                                                                                                                          })
                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                   {
                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                   {
                                                                                                                                                                                       self::close_paren(state)
                                                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn tuple_ty(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::tuple_ty,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_paren(state).and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       state.sequence(|state|
                                                                                                                                          {
                                                                                                                                              state.sequence(|state|
                                                                                                                                                                 {
                                                                                                                                                                     self::ty(state).and_then(|state|
                                                                                                                                                                                                  {
                                                                                                                                                                                                      super::hidden::skip(state)
                                                                                                                                                                                                  }).and_then(|state|
                                                                                                                                                                                                                  {
                                                                                                                                                                                                                      self::comma(state)
                                                                                                                                                                                                                  })
                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                 {
                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                 {
                                                                                                                                                                                                     state.sequence(|state|
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            state.optional(|state|
                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                   state.restore_on_err(|state|
                                                                                                                                                                                                                                                                            state.sequence(|state|
                                                                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                                                                   self::ty(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                                                                    super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                                                                                    self::comma(state)
                                                                                                                                                                                                                                                                                                                                                })
                                                                                                                                                                                                                                                                                               })).and_then(|state|
                                                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                                                    state.repeat(|state|
                                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                                         state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                                                super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                                                                                            state.restore_on_err(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                     state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            self::ty(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             self::comma(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         })
                                                                                                                                                                                                                                                                                                                                                                                                                                                        }))
                                                                                                                                                                                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                                                                                                })
                                                                                                                                                                                                                                               })
                                                                                                                                                                                                                        })
                                                                                                                                                                                                 })
                                                                                                                                          })
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       state.optional(|state|
                                                                                                                                                                          {
                                                                                                                                                                              state.restore_on_err(|state|
                                                                                                                                                                                                       self::ty(state))
                                                                                                                                                                          })
                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                   {
                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                   {
                                                                                                                                                                                       self::close_paren(state)
                                                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn tuple_destructure(state:
                                             Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::tuple_destructure,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::open_paren(state).and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       state.sequence(|state|
                                                                                                                                          {
                                                                                                                                              state.sequence(|state|
                                                                                                                                                                 {
                                                                                                                                                                     self::binding(state).and_then(|state|
                                                                                                                                                                                                       {
                                                                                                                                                                                                           super::hidden::skip(state)
                                                                                                                                                                                                       }).and_then(|state|
                                                                                                                                                                                                                       {
                                                                                                                                                                                                                           self::comma(state)
                                                                                                                                                                                                                       })
                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                 {
                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                 {
                                                                                                                                                                                                     state.sequence(|state|
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            state.optional(|state|
                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                   state.sequence(|state|
                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                          self::binding(state).and_then(|state|
                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                            }).and_then(|state|
                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                self::comma(state)
                                                                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                          state.repeat(|state|
                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                               state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                      super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                                                                                                                  state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                                                                                                         self::binding(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                                                                                                                                               super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                           }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                                                                                                                                                               self::comma(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                           })
                                                                                                                                                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                                                                                                                                                              })
                                                                                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                                                                                                           })
                                                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                                                               })
                                                                                                                                                                                                                        })
                                                                                                                                                                                                 })
                                                                                                                                          })
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       state.optional(|state|
                                                                                                                                                                          {
                                                                                                                                                                              self::binding(state)
                                                                                                                                                                          })
                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                   {
                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                   {
                                                                                                                                                                                       self::close_paren(state)
                                                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_field(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::struct_field,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::ident(state).and_then(|state|
                                                                                              {
                                                                                                  super::hidden::skip(state)
                                                                                              }).and_then(|state|
                                                                                                              {
                                                                                                                  self::typespec(state)
                                                                                                              })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_fields(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           state.sequence(|state|
                                                              {
                                                                  state.optional(|state|
                                                                                     {
                                                                                         state.restore_on_err(|state|
                                                                                                                  state.sequence(|state|
                                                                                                                                     {
                                                                                                                                         self::struct_field(state).and_then(|state|
                                                                                                                                                                                {
                                                                                                                                                                                    super::hidden::skip(state)
                                                                                                                                                                                }).and_then(|state|
                                                                                                                                                                                                {
                                                                                                                                                                                                    self::comma(state)
                                                                                                                                                                                                })
                                                                                                                                     })).and_then(|state|
                                                                                                                                                      {
                                                                                                                                                          state.repeat(|state|
                                                                                                                                                                           {
                                                                                                                                                                               state.sequence(|state|
                                                                                                                                                                                                  {
                                                                                                                                                                                                      super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                  state.restore_on_err(|state|
                                                                                                                                                                                                                                                                           state.sequence(|state|
                                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                                  self::struct_field(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                                                                                                                             super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                         }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                                                                                                                                             self::comma(state)
                                                                                                                                                                                                                                                                                                                                                         })
                                                                                                                                                                                                                                                                                              }))
                                                                                                                                                                                                                                              })
                                                                                                                                                                                                  })
                                                                                                                                                                           })
                                                                                                                                                      })
                                                                                     })
                                                              }).and_then(|state|
                                                                              {
                                                                                  super::hidden::skip(state)
                                                                              }).and_then(|state|
                                                                                              {
                                                                                                  state.optional(|state|
                                                                                                                     {
                                                                                                                         state.restore_on_err(|state|
                                                                                                                                                  self::struct_field(state))
                                                                                                                     })
                                                                                              })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_decl(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::struct_decl,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::struct_keyword(state).and_then(|state|
                                                                                                       {
                                                                                                           super::hidden::skip(state)
                                                                                                       }).and_then(|state|
                                                                                                                       {
                                                                                                                           self::ident(state)
                                                                                                                       }).and_then(|state|
                                                                                                                                       {
                                                                                                                                           super::hidden::skip(state)
                                                                                                                                       }).and_then(|state|
                                                                                                                                                       {
                                                                                                                                                           self::open_block(state)
                                                                                                                                                       }).and_then(|state|
                                                                                                                                                                       {
                                                                                                                                                                           super::hidden::skip(state)
                                                                                                                                                                       }).and_then(|state|
                                                                                                                                                                                       {
                                                                                                                                                                                           self::struct_fields(state)
                                                                                                                                                                                       }).and_then(|state|
                                                                                                                                                                                                       {
                                                                                                                                                                                                           super::hidden::skip(state)
                                                                                                                                                                                                       }).and_then(|state|
                                                                                                                                                                                                                       {
                                                                                                                                                                                                                           self::close_block(state)
                                                                                                                                                                                                                       })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_construct_field(state:
                                                  Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::struct_construct_field,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::ident(state).and_then(|state|
                                                                                              {
                                                                                                  super::hidden::skip(state)
                                                                                              }).and_then(|state|
                                                                                                              {
                                                                                                                  state.optional(|state|
                                                                                                                                     {
                                                                                                                                         state.restore_on_err(|state|
                                                                                                                                                                  state.sequence(|state|
                                                                                                                                                                                     {
                                                                                                                                                                                         self::colon(state).and_then(|state|
                                                                                                                                                                                                                         {
                                                                                                                                                                                                                             super::hidden::skip(state)
                                                                                                                                                                                                                         }).and_then(|state|
                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                             self::expr(state)
                                                                                                                                                                                                                                         })
                                                                                                                                                                                     }))
                                                                                                                                     })
                                                                                                              })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_construct_fields(state:
                                                   Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           state.sequence(|state|
                                                              {
                                                                  state.optional(|state|
                                                                                     {
                                                                                         state.restore_on_err(|state|
                                                                                                                  state.sequence(|state|
                                                                                                                                     {
                                                                                                                                         self::struct_construct_field(state).and_then(|state|
                                                                                                                                                                                          {
                                                                                                                                                                                              super::hidden::skip(state)
                                                                                                                                                                                          }).and_then(|state|
                                                                                                                                                                                                          {
                                                                                                                                                                                                              self::comma(state)
                                                                                                                                                                                                          })
                                                                                                                                     })).and_then(|state|
                                                                                                                                                      {
                                                                                                                                                          state.repeat(|state|
                                                                                                                                                                           {
                                                                                                                                                                               state.sequence(|state|
                                                                                                                                                                                                  {
                                                                                                                                                                                                      super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                  state.restore_on_err(|state|
                                                                                                                                                                                                                                                                           state.sequence(|state|
                                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                                  self::struct_construct_field(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                                                                                                                                       self::comma(state)
                                                                                                                                                                                                                                                                                                                                                                   })
                                                                                                                                                                                                                                                                                              }))
                                                                                                                                                                                                                                              })
                                                                                                                                                                                                  })
                                                                                                                                                                           })
                                                                                                                                                      })
                                                                                     })
                                                              }).and_then(|state|
                                                                              {
                                                                                  super::hidden::skip(state)
                                                                              }).and_then(|state|
                                                                                              {
                                                                                                  state.optional(|state|
                                                                                                                     {
                                                                                                                         state.restore_on_err(|state|
                                                                                                                                                  self::struct_construct_field(state))
                                                                                                                     })
                                                                                              })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_construct_expr(state:
                                                 Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::struct_construct_expr,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::ident(state).and_then(|state|
                                                                                              {
                                                                                                  super::hidden::skip(state)
                                                                                              }).and_then(|state|
                                                                                                              {
                                                                                                                  self::open_block(state)
                                                                                                              }).and_then(|state|
                                                                                                                              {
                                                                                                                                  super::hidden::skip(state)
                                                                                                                              }).and_then(|state|
                                                                                                                                              {
                                                                                                                                                  self::struct_construct_fields(state)
                                                                                                                                              }).and_then(|state|
                                                                                                                                                              {
                                                                                                                                                                  super::hidden::skip(state)
                                                                                                                                                              }).and_then(|state|
                                                                                                                                                                              {
                                                                                                                                                                                  self::close_block(state)
                                                                                                                                                                              })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_destructure_field(state:
                                                    Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::struct_destructure_field,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::mut_keyword(state).and_then(|state|
                                                                                                    {
                                                                                                        super::hidden::skip(state)
                                                                                                    }).and_then(|state|
                                                                                                                    {
                                                                                                                        self::ident(state)
                                                                                                                    })
                                                          }).or_else(|state|
                                                                         {
                                                                             state.sequence(|state|
                                                                                                {
                                                                                                    self::ident(state).and_then(|state|
                                                                                                                                    {
                                                                                                                                        super::hidden::skip(state)
                                                                                                                                    }).and_then(|state|
                                                                                                                                                    {
                                                                                                                                                        state.optional(|state|
                                                                                                                                                                           {
                                                                                                                                                                               state.sequence(|state|
                                                                                                                                                                                                  {
                                                                                                                                                                                                      self::colon(state).and_then(|state|
                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                          super::hidden::skip(state)
                                                                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                          self::binding(state)
                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                  })
                                                                                                                                                                           })
                                                                                                                                                    })
                                                                                                })
                                                                         })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_destructure_fields(state:
                                                     Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           state.sequence(|state|
                                                              {
                                                                  state.optional(|state|
                                                                                     {
                                                                                         state.sequence(|state|
                                                                                                            {
                                                                                                                self::struct_destructure_field(state).and_then(|state|
                                                                                                                                                                   {
                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                   {
                                                                                                                                                                                       self::comma(state)
                                                                                                                                                                                   })
                                                                                                            }).and_then(|state|
                                                                                                                            {
                                                                                                                                state.repeat(|state|
                                                                                                                                                 {
                                                                                                                                                     state.sequence(|state|
                                                                                                                                                                        {
                                                                                                                                                                            super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        state.sequence(|state|
                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                               self::struct_destructure_field(state).and_then(|state|
                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                      super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                  }).and_then(|state|
                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                      self::comma(state)
                                                                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                                           })
                                                                                                                                                                                                                    })
                                                                                                                                                                        })
                                                                                                                                                 })
                                                                                                                            })
                                                                                     })
                                                              }).and_then(|state|
                                                                              {
                                                                                  super::hidden::skip(state)
                                                                              }).and_then(|state|
                                                                                              {
                                                                                                  state.optional(|state|
                                                                                                                     {
                                                                                                                         self::struct_destructure_field(state)
                                                                                                                     })
                                                                                              })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_destructure(state:
                                              Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::struct_destructure,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::ident(state).and_then(|state|
                                                                                              {
                                                                                                  super::hidden::skip(state)
                                                                                              }).and_then(|state|
                                                                                                              {
                                                                                                                  self::open_block(state)
                                                                                                              }).and_then(|state|
                                                                                                                              {
                                                                                                                                  super::hidden::skip(state)
                                                                                                                              }).and_then(|state|
                                                                                                                                              {
                                                                                                                                                  self::struct_destructure_fields(state)
                                                                                                                                              }).and_then(|state|
                                                                                                                                                              {
                                                                                                                                                                  super::hidden::skip(state)
                                                                                                                                                              }).and_then(|state|
                                                                                                                                                                              {
                                                                                                                                                                                  self::close_block(state)
                                                                                                                                                                              })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn colon(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string(":")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn semicolon(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string(";")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comma(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string(",")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn open_block(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("{")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn close_block(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("}")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn open_square(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("[")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn close_square(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("]")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn open_angle(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("<")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn close_angle(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string(">")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn open_paren(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("(")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn close_paren(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string(")")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn low_line(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("_")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn double_colon(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("::")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn literal(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::literal,
                               |state|
                                   {
                                       self::float(state).or_else(|state|
                                                                      {
                                                                          self::signed_integer(state)
                                                                      }).or_else(|state|
                                                                                     {
                                                                                         self::boolean(state)
                                                                                     }).or_else(|state|
                                                                                                    {
                                                                                                        state.restore_on_err(|state|
                                                                                                                                 self::string(state))
                                                                                                    })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::string_single_line(state).or_else(|state|
                                                                {
                                                                    state.restore_on_err(|state|
                                                                                             self::raw_string(state))
                                                                })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string_single_line(state:
                                              Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           self::string_delimiter(state).and_then(|state|
                                                                                      {
                                                                                          super::hidden::skip(state)
                                                                                      }).and_then(|state|
                                                                                                      {
                                                                                                          self::string_content(state)
                                                                                                      }).and_then(|state|
                                                                                                                      {
                                                                                                                          super::hidden::skip(state)
                                                                                                                      }).and_then(|state|
                                                                                                                                      {
                                                                                                                                          self::string_delimiter(state)
                                                                                                                                      })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string_content(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic,
                                 |state|
                                     {
                                         state.rule(Rule::string_content,
                                                    |state|
                                                        {
                                                            state.repeat(|state|
                                                                             {
                                                                                 self::string_escape(state).or_else(|state|
                                                                                                                        {
                                                                                                                            state.sequence(|state|
                                                                                                                                               {
                                                                                                                                                   state.lookahead(false,
                                                                                                                                                                   |state|
                                                                                                                                                                       {
                                                                                                                                                                           self::string_delimiter(state).or_else(|state|
                                                                                                                                                                                                                     {
                                                                                                                                                                                                                         state.match_string("\\")
                                                                                                                                                                                                                     })
                                                                                                                                                                       }).and_then(|state|
                                                                                                                                                                                       {
                                                                                                                                                                                           self::ANY(state)
                                                                                                                                                                                       })
                                                                                                                                               })
                                                                                                                        })
                                                                             })
                                                        })
                                     })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string_delimiter(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\"")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string_unicode(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           state.match_string("u").and_then(|state|
                                                                                {
                                                                                    super::hidden::skip(state)
                                                                                }).and_then(|state|
                                                                                                {
                                                                                                    self::ASCII_HEX_DIGIT(state)
                                                                                                }).and_then(|state|
                                                                                                                {
                                                                                                                    super::hidden::skip(state)
                                                                                                                }).and_then(|state|
                                                                                                                                {
                                                                                                                                    self::ASCII_HEX_DIGIT(state)
                                                                                                                                }).and_then(|state|
                                                                                                                                                {
                                                                                                                                                    super::hidden::skip(state)
                                                                                                                                                }).and_then(|state|
                                                                                                                                                                {
                                                                                                                                                                    self::ASCII_HEX_DIGIT(state)
                                                                                                                                                                }).and_then(|state|
                                                                                                                                                                                {
                                                                                                                                                                                    super::hidden::skip(state)
                                                                                                                                                                                }).and_then(|state|
                                                                                                                                                                                                {
                                                                                                                                                                                                    self::ASCII_HEX_DIGIT(state)
                                                                                                                                                                                                })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string_escape(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           state.match_string("\\").and_then(|state|
                                                                                 {
                                                                                     super::hidden::skip(state)
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     state.match_string("\"").or_else(|state|
                                                                                                                                          {
                                                                                                                                              state.match_string("\\")
                                                                                                                                          }).or_else(|state|
                                                                                                                                                         {
                                                                                                                                                             state.match_string("a")
                                                                                                                                                         }).or_else(|state|
                                                                                                                                                                        {
                                                                                                                                                                            state.match_string("b")
                                                                                                                                                                        }).or_else(|state|
                                                                                                                                                                                       {
                                                                                                                                                                                           state.match_string("f")
                                                                                                                                                                                       }).or_else(|state|
                                                                                                                                                                                                      {
                                                                                                                                                                                                          state.match_string("n")
                                                                                                                                                                                                      }).or_else(|state|
                                                                                                                                                                                                                     {
                                                                                                                                                                                                                         state.match_string("r")
                                                                                                                                                                                                                     }).or_else(|state|
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        state.match_string("t")
                                                                                                                                                                                                                                    }).or_else(|state|
                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                       state.match_string("v")
                                                                                                                                                                                                                                                   }).or_else(|state|
                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                      self::string_unicode(state)
                                                                                                                                                                                                                                                                  })
                                                                                                 })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn raw_string(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic,
                                 |state|
                                     {
                                         state.rule(Rule::raw_string,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   state.match_string("r").and_then(|state|
                                                                                                                        {
                                                                                                                            state.stack_push(|state|
                                                                                                                                                 state.repeat(|state|
                                                                                                                                                                  {
                                                                                                                                                                      state.match_string("#")
                                                                                                                                                                  }))
                                                                                                                        }).and_then(|state|
                                                                                                                                        {
                                                                                                                                            state.match_string("\"")
                                                                                                                                        }).and_then(|state|
                                                                                                                                                        {
                                                                                                                                                            self::raw_string_interior(state)
                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                        {
                                                                                                                                                                            state.match_string("\"")
                                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                                        {
                                                                                                                                                                                            self::POP(state)
                                                                                                                                                                                        })
                                                                               })
                                                        })
                                     })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn raw_string_interior(state:
                                               Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::raw_string_interior,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.repeat(|state|
                                                                             {
                                                                                 state.sequence(|state|
                                                                                                    {
                                                                                                        state.lookahead(false,
                                                                                                                        |state|
                                                                                                                            {
                                                                                                                                state.sequence(|state|
                                                                                                                                                   {
                                                                                                                                                       state.match_string("\"").and_then(|state|
                                                                                                                                                                                             {
                                                                                                                                                                                                 self::PEEK(state)
                                                                                                                                                                                             })
                                                                                                                                                   })
                                                                                                                            }).and_then(|state|
                                                                                                                                            {
                                                                                                                                                self::ANY(state)
                                                                                                                                            })
                                                                                                    })
                                                                             })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn float(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::float,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   state.sequence(|state|
                                                                                                      {
                                                                                                          self::signed_integer(state).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       state.repeat(|state|
                                                                                                                                                                        {
                                                                                                                                                                            self::signed_integer(state)
                                                                                                                                                                        })
                                                                                                                                                   })
                                                                                                      }).and_then(|state|
                                                                                                                      {
                                                                                                                          state.match_string(".")
                                                                                                                      }).and_then(|state|
                                                                                                                                      {
                                                                                                                                          state.sequence(|state|
                                                                                                                                                             {
                                                                                                                                                                 self::integer(state).and_then(|state|
                                                                                                                                                                                                   {
                                                                                                                                                                                                       state.repeat(|state|
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            self::integer(state)
                                                                                                                                                                                                                        })
                                                                                                                                                                                                   })
                                                                                                                                                             })
                                                                                                                                      }).and_then(|state|
                                                                                                                                                      {
                                                                                                                                                          state.optional(|state|
                                                                                                                                                                             {
                                                                                                                                                                                 state.sequence(|state|
                                                                                                                                                                                                    {
                                                                                                                                                                                                        state.match_insensitive("e").and_then(|state|
                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                      self::signed_integer(state)
                                                                                                                                                                                                                                                  }).and_then(|state|
                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                      state.repeat(|state|
                                                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                                                           self::signed_integer(state)
                                                                                                                                                                                                                                                                                       })
                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                    })
                                                                                                                                                                             })
                                                                                                                                                      })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn signed_integer(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           state.optional(|state|
                                                              {
                                                                  self::plus(state).or_else(|state|
                                                                                                {
                                                                                                    self::minus(state)
                                                                                                })
                                                              }).and_then(|state|
                                                                              {
                                                                                  super::hidden::skip(state)
                                                                              }).and_then(|state|
                                                                                              {
                                                                                                  self::integer(state)
                                                                                              })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn integer(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::hexinteger(state).or_else(|state|
                                                        {
                                                            self::bininteger(state)
                                                        }).or_else(|state|
                                                                       {
                                                                           self::octinteger(state)
                                                                       }).or_else(|state|
                                                                                      {
                                                                                          self::decinteger(state)
                                                                                      })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn decinteger(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::decinteger,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   self::ASCII_DIGIT(state).or_else(|state|
                                                                                                                        {
                                                                                                                            self::low_line(state)
                                                                                                                        }).and_then(|state|
                                                                                                                                        {
                                                                                                                                            state.repeat(|state|
                                                                                                                                                             {
                                                                                                                                                                 self::ASCII_DIGIT(state).or_else(|state|
                                                                                                                                                                                                      {
                                                                                                                                                                                                          self::low_line(state)
                                                                                                                                                                                                      })
                                                                                                                                                             })
                                                                                                                                        })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn bininteger(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::bininteger,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   state.match_insensitive("0b").and_then(|state|
                                                                                                                              {
                                                                                                                                  self::ASCII_BIN_DIGIT(state).or_else(|state|
                                                                                                                                                                           {
                                                                                                                                                                               self::low_line(state)
                                                                                                                                                                           })
                                                                                                                              }).and_then(|state|
                                                                                                                                              {
                                                                                                                                                  state.repeat(|state|
                                                                                                                                                                   {
                                                                                                                                                                       self::ASCII_BIN_DIGIT(state).or_else(|state|
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    self::low_line(state)
                                                                                                                                                                                                                })
                                                                                                                                                                   })
                                                                                                                                              })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn hexinteger(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::hexinteger,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   state.match_insensitive("0x").and_then(|state|
                                                                                                                              {
                                                                                                                                  self::ASCII_HEX_DIGIT(state).or_else(|state|
                                                                                                                                                                           {
                                                                                                                                                                               self::low_line(state)
                                                                                                                                                                           })
                                                                                                                              }).and_then(|state|
                                                                                                                                              {
                                                                                                                                                  state.repeat(|state|
                                                                                                                                                                   {
                                                                                                                                                                       self::ASCII_HEX_DIGIT(state).or_else(|state|
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    self::low_line(state)
                                                                                                                                                                                                                })
                                                                                                                                                                   })
                                                                                                                                              })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn octinteger(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::octinteger,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   state.match_insensitive("0o").and_then(|state|
                                                                                                                              {
                                                                                                                                  self::ASCII_OCT_DIGIT(state).or_else(|state|
                                                                                                                                                                           {
                                                                                                                                                                               self::low_line(state)
                                                                                                                                                                           })
                                                                                                                              }).and_then(|state|
                                                                                                                                              {
                                                                                                                                                  state.repeat(|state|
                                                                                                                                                                   {
                                                                                                                                                                       self::ASCII_OCT_DIGIT(state).or_else(|state|
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    self::low_line(state)
                                                                                                                                                                                                                })
                                                                                                                                                                   })
                                                                                                                                              })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn boolean(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::boolean_true(state).or_else(|state|
                                                          {
                                                              self::boolean_false(state)
                                                          })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn boolean_true(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::boolean_true,
                               |state| { state.match_string("true") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn boolean_false(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::boolean_false,
                               |state| { state.match_string("false") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn multiply(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::multiply,
                               |state| { state.match_string("*") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn divide(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::divide,
                               |state| { state.match_string("/") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn modulus(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::modulus,
                               |state| { state.match_string("%") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn plus(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::plus,
                               |state| { state.match_string("+") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn minus(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::minus,
                               |state| { state.match_string("-") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn shift_left(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::shift_left,
                               |state| { state.match_string("<<") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn shift_right(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::shift_right,
                               |state| { state.match_string(">>") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn less_than(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::less_than,
                               |state| { state.match_string("<") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn less_than_or_equal(state:
                                              Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::less_than_or_equal,
                               |state| { state.match_string("<=") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn greater_than(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::greater_than,
                               |state| { state.match_string(">") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn greater_than_or_equal(state:
                                                 Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::greater_than_or_equal,
                               |state| { state.match_string(">=") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn not_equal(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::not_equal,
                               |state| { state.match_string("!=") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn equal(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::equal,
                               |state| { state.match_string("==") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_and(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::logical_and,
                               |state| { state.match_string("&&") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_or(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::logical_or,
                               |state| { state.match_string("||") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_not(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::logical_not,
                               |state| { state.match_string("!") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn bitwise_and(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::bitwise_and,
                               |state| { state.match_string("&") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn bitwise_or(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::bitwise_or,
                               |state| { state.match_string("|") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn bitwise_xor(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::bitwise_xor,
                               |state| { state.match_string("^") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assign,
                               |state| { state.match_string("=") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn range(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::range,
                               |state| { state.match_string("..") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn unary_operator(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::plus(state).or_else(|state|
                                                  {
                                                      self::minus(state)
                                                  }).or_else(|state|
                                                                 {
                                                                     self::logical_not(state)
                                                                 })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn binary_operator(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::multiply(state).or_else(|state|
                                                      {
                                                          self::divide(state)
                                                      }).or_else(|state|
                                                                     {
                                                                         self::modulus(state)
                                                                     }).or_else(|state|
                                                                                    {
                                                                                        self::plus(state)
                                                                                    }).or_else(|state|
                                                                                                   {
                                                                                                       self::minus(state)
                                                                                                   }).or_else(|state|
                                                                                                                  {
                                                                                                                      self::shift_left(state)
                                                                                                                  }).or_else(|state|
                                                                                                                                 {
                                                                                                                                     self::shift_right(state)
                                                                                                                                 }).or_else(|state|
                                                                                                                                                {
                                                                                                                                                    self::less_than_or_equal(state)
                                                                                                                                                }).or_else(|state|
                                                                                                                                                               {
                                                                                                                                                                   self::less_than(state)
                                                                                                                                                               }).or_else(|state|
                                                                                                                                                                              {
                                                                                                                                                                                  self::greater_than_or_equal(state)
                                                                                                                                                                              }).or_else(|state|
                                                                                                                                                                                             {
                                                                                                                                                                                                 self::greater_than(state)
                                                                                                                                                                                             }).or_else(|state|
                                                                                                                                                                                                            {
                                                                                                                                                                                                                self::not_equal(state)
                                                                                                                                                                                                            }).or_else(|state|
                                                                                                                                                                                                                           {
                                                                                                                                                                                                                               self::equal(state)
                                                                                                                                                                                                                           }).or_else(|state|
                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                              self::logical_and(state)
                                                                                                                                                                                                                                          }).or_else(|state|
                                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                                             self::logical_or(state)
                                                                                                                                                                                                                                                         }).or_else(|state|
                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                            self::bitwise_and(state)
                                                                                                                                                                                                                                                                        }).or_else(|state|
                                                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                                                           self::bitwise_or(state)
                                                                                                                                                                                                                                                                                       }).or_else(|state|
                                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                                          self::bitwise_xor(state)
                                                                                                                                                                                                                                                                                                      }).or_else(|state|
                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                         self::range(state)
                                                                                                                                                                                                                                                                                                                     })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_operator(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assign_operator,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.optional(|state|
                                                                                 {
                                                                                     self::multiply(state).or_else(|state|
                                                                                                                       {
                                                                                                                           self::divide(state)
                                                                                                                       }).or_else(|state|
                                                                                                                                      {
                                                                                                                                          self::modulus(state)
                                                                                                                                      }).or_else(|state|
                                                                                                                                                     {
                                                                                                                                                         self::plus(state)
                                                                                                                                                     }).or_else(|state|
                                                                                                                                                                    {
                                                                                                                                                                        self::minus(state)
                                                                                                                                                                    }).or_else(|state|
                                                                                                                                                                                   {
                                                                                                                                                                                       self::shift_left(state)
                                                                                                                                                                                   }).or_else(|state|
                                                                                                                                                                                                  {
                                                                                                                                                                                                      self::shift_right(state)
                                                                                                                                                                                                  }).or_else(|state|
                                                                                                                                                                                                                 {
                                                                                                                                                                                                                     self::bitwise_and(state)
                                                                                                                                                                                                                 }).or_else(|state|
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    self::bitwise_or(state)
                                                                                                                                                                                                                                }).or_else(|state|
                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                   self::bitwise_xor(state)
                                                                                                                                                                                                                                               })
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     super::hidden::skip(state)
                                                                                                 }).and_then(|state|
                                                                                                                 {
                                                                                                                     self::assign(state)
                                                                                                                 })
                                                          })
                                   })
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn PEEK(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_peek()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn POP(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_pop()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_BIN_DIGIT(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'1')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_OCT_DIGIT(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'7')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9').or_else(|state|
                                                            state.match_range('a'..'f')).or_else(|state|
                                                                                                     state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state|
                                                         state.match_string("\r\n")).or_else(|state|
                                                                                                 state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input,
                      |state|
                          {
                              match rule {
                                  Rule::file => rules::file(state),
                                  Rule::item => rules::item(state),
                                  Rule::const_item =>
                                  rules::const_item(state),
                                  Rule::ident => rules::ident(state),
                                  Rule::WHITESPACE =>
                                  rules::WHITESPACE(state),
                                  Rule::COMMENT => rules::COMMENT(state),
                                  Rule::comment_block =>
                                  rules::comment_block(state),
                                  Rule::comment_line =>
                                  rules::comment_line(state),
                                  Rule::mut_keyword =>
                                  rules::mut_keyword(state),
                                  Rule::const_keyword =>
                                  rules::const_keyword(state),
                                  Rule::unsafe_keyword =>
                                  rules::unsafe_keyword(state),
                                  Rule::self_keyword =>
                                  rules::self_keyword(state),
                                  Rule::trait_keyword =>
                                  rules::trait_keyword(state),
                                  Rule::let_leyword =>
                                  rules::let_leyword(state),
                                  Rule::return_keyword =>
                                  rules::return_keyword(state),
                                  Rule::if_keyword =>
                                  rules::if_keyword(state),
                                  Rule::else_keyword =>
                                  rules::else_keyword(state),
                                  Rule::while_keyword =>
                                  rules::while_keyword(state),
                                  Rule::loop_keyword =>
                                  rules::loop_keyword(state),
                                  Rule::for_keyword =>
                                  rules::for_keyword(state),
                                  Rule::in_keyword =>
                                  rules::in_keyword(state),
                                  Rule::fn_keyword =>
                                  rules::fn_keyword(state),
                                  Rule::as_keyword =>
                                  rules::as_keyword(state),
                                  Rule::struct_keyword =>
                                  rules::struct_keyword(state),
                                  Rule::break_keyword =>
                                  rules::break_keyword(state),
                                  Rule::impl_keyword =>
                                  rules::impl_keyword(state),
                                  Rule::type_keyword =>
                                  rules::type_keyword(state),
                                  Rule::typespec => rules::typespec(state),
                                  Rule::ty => rules::ty(state),
                                  Rule::basic_ty => rules::basic_ty(state),
                                  Rule::generic_ty =>
                                  rules::generic_ty(state),
                                  Rule::ptr_ty => rules::ptr_ty(state),
                                  Rule::code_block =>
                                  rules::code_block(state),
                                  Rule::unsafe_code_block =>
                                  rules::unsafe_code_block(state),
                                  Rule::function => rules::function(state),
                                  Rule::function_args =>
                                  rules::function_args(state),
                                  Rule::function_arg =>
                                  rules::function_arg(state),
                                  Rule::function_return =>
                                  rules::function_return(state),
                                  Rule::function_call =>
                                  rules::function_call(state),
                                  Rule::turbofish => rules::turbofish(state),
                                  Rule::ty_param => rules::ty_param(state),
                                  Rule::ty_params => rules::ty_params(state),
                                  Rule::ty_args => rules::ty_args(state),
                                  Rule::impl_block =>
                                  rules::impl_block(state),
                                  Rule::impl_trait =>
                                  rules::impl_trait(state),
                                  Rule::impl_item => rules::impl_item(state),
                                  Rule::stmt => rules::stmt(state),
                                  Rule::local => rules::local(state),
                                  Rule::type_alias =>
                                  rules::type_alias(state),
                                  Rule::binding => rules::binding(state),
                                  Rule::destructure =>
                                  rules::destructure(state),
                                  Rule::expr => rules::expr(state),
                                  Rule::basic_expr =>
                                  rules::basic_expr(state),
                                  Rule::inner_expr =>
                                  rules::inner_expr(state),
                                  Rule::dereferenceable_expr =>
                                  rules::dereferenceable_expr(state),
                                  Rule::cast_expr => rules::cast_expr(state),
                                  Rule::prefix => rules::prefix(state),
                                  Rule::infix => rules::infix(state),
                                  Rule::braced_expr =>
                                  rules::braced_expr(state),
                                  Rule::static_access =>
                                  rules::static_access(state),
                                  Rule::field_access =>
                                  rules::field_access(state),
                                  Rule::array_access =>
                                  rules::array_access(state),
                                  Rule::reference_expr =>
                                  rules::reference_expr(state),
                                  Rule::dereference =>
                                  rules::dereference(state),
                                  Rule::implicit_deref_expr =>
                                  rules::implicit_deref_expr(state),
                                  Rule::explicit_deref_expr =>
                                  rules::explicit_deref_expr(state),
                                  Rule::deref_expr =>
                                  rules::deref_expr(state),
                                  Rule::return_expr =>
                                  rules::return_expr(state),
                                  Rule::break_expr =>
                                  rules::break_expr(state),
                                  Rule::assign_expr =>
                                  rules::assign_expr(state),
                                  Rule::if_expr => rules::if_expr(state),
                                  Rule::if_else => rules::if_else(state),
                                  Rule::loop_label =>
                                  rules::loop_label(state),
                                  Rule::while_loop =>
                                  rules::while_loop(state),
                                  Rule::for_loop => rules::for_loop(state),
                                  Rule::inf_loop => rules::inf_loop(state),
                                  Rule::array => rules::array(state),
                                  Rule::array_lit => rules::array_lit(state),
                                  Rule::array_splat =>
                                  rules::array_splat(state),
                                  Rule::array_ty => rules::array_ty(state),
                                  Rule::array_destructure =>
                                  rules::array_destructure(state),
                                  Rule::tuple => rules::tuple(state),
                                  Rule::tuple_ty => rules::tuple_ty(state),
                                  Rule::tuple_destructure =>
                                  rules::tuple_destructure(state),
                                  Rule::struct_field =>
                                  rules::struct_field(state),
                                  Rule::struct_fields =>
                                  rules::struct_fields(state),
                                  Rule::struct_decl =>
                                  rules::struct_decl(state),
                                  Rule::struct_construct_field =>
                                  rules::struct_construct_field(state),
                                  Rule::struct_construct_fields =>
                                  rules::struct_construct_fields(state),
                                  Rule::struct_construct_expr =>
                                  rules::struct_construct_expr(state),
                                  Rule::struct_destructure_field =>
                                  rules::struct_destructure_field(state),
                                  Rule::struct_destructure_fields =>
                                  rules::struct_destructure_fields(state),
                                  Rule::struct_destructure =>
                                  rules::struct_destructure(state),
                                  Rule::colon => rules::colon(state),
                                  Rule::semicolon => rules::semicolon(state),
                                  Rule::comma => rules::comma(state),
                                  Rule::open_block =>
                                  rules::open_block(state),
                                  Rule::close_block =>
                                  rules::close_block(state),
                                  Rule::open_square =>
                                  rules::open_square(state),
                                  Rule::close_square =>
                                  rules::close_square(state),
                                  Rule::open_angle =>
                                  rules::open_angle(state),
                                  Rule::close_angle =>
                                  rules::close_angle(state),
                                  Rule::open_paren =>
                                  rules::open_paren(state),
                                  Rule::close_paren =>
                                  rules::close_paren(state),
                                  Rule::low_line => rules::low_line(state),
                                  Rule::double_colon =>
                                  rules::double_colon(state),
                                  Rule::literal => rules::literal(state),
                                  Rule::string => rules::string(state),
                                  Rule::string_single_line =>
                                  rules::string_single_line(state),
                                  Rule::string_content =>
                                  rules::string_content(state),
                                  Rule::string_delimiter =>
                                  rules::string_delimiter(state),
                                  Rule::string_unicode =>
                                  rules::string_unicode(state),
                                  Rule::string_escape =>
                                  rules::string_escape(state),
                                  Rule::raw_string =>
                                  rules::raw_string(state),
                                  Rule::raw_string_interior =>
                                  rules::raw_string_interior(state),
                                  Rule::float => rules::float(state),
                                  Rule::signed_integer =>
                                  rules::signed_integer(state),
                                  Rule::integer => rules::integer(state),
                                  Rule::decinteger =>
                                  rules::decinteger(state),
                                  Rule::bininteger =>
                                  rules::bininteger(state),
                                  Rule::hexinteger =>
                                  rules::hexinteger(state),
                                  Rule::octinteger =>
                                  rules::octinteger(state),
                                  Rule::boolean => rules::boolean(state),
                                  Rule::boolean_true =>
                                  rules::boolean_true(state),
                                  Rule::boolean_false =>
                                  rules::boolean_false(state),
                                  Rule::multiply => rules::multiply(state),
                                  Rule::divide => rules::divide(state),
                                  Rule::modulus => rules::modulus(state),
                                  Rule::plus => rules::plus(state),
                                  Rule::minus => rules::minus(state),
                                  Rule::shift_left =>
                                  rules::shift_left(state),
                                  Rule::shift_right =>
                                  rules::shift_right(state),
                                  Rule::less_than => rules::less_than(state),
                                  Rule::less_than_or_equal =>
                                  rules::less_than_or_equal(state),
                                  Rule::greater_than =>
                                  rules::greater_than(state),
                                  Rule::greater_than_or_equal =>
                                  rules::greater_than_or_equal(state),
                                  Rule::not_equal => rules::not_equal(state),
                                  Rule::equal => rules::equal(state),
                                  Rule::logical_and =>
                                  rules::logical_and(state),
                                  Rule::logical_or =>
                                  rules::logical_or(state),
                                  Rule::logical_not =>
                                  rules::logical_not(state),
                                  Rule::bitwise_and =>
                                  rules::bitwise_and(state),
                                  Rule::bitwise_or =>
                                  rules::bitwise_or(state),
                                  Rule::bitwise_xor =>
                                  rules::bitwise_xor(state),
                                  Rule::assign => rules::assign(state),
                                  Rule::range => rules::range(state),
                                  Rule::unary_operator =>
                                  rules::unary_operator(state),
                                  Rule::binary_operator =>
                                  rules::binary_operator(state),
                                  Rule::assign_operator =>
                                  rules::assign_operator(state),
                                  Rule::EOI => rules::EOI(state),
                              }
                          })
    }
}

pub mod ast {
    mod assign {
        use super::*;
        pub enum AssignOp {
            Assign,
            Add,
            Sub,
            Mul,
            Div,
            Rem,
            ShiftLeft,
            ShiftRight,
            BitwiseAnd,
            BitwiseOr,
            BitwiseXor,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for AssignOp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&AssignOp::Assign,) => {
                        let mut debug_trait_builder = f.debug_tuple("Assign");
                        debug_trait_builder.finish()
                    }
                    (&AssignOp::Add,) => {
                        let mut debug_trait_builder = f.debug_tuple("Add");
                        debug_trait_builder.finish()
                    }
                    (&AssignOp::Sub,) => {
                        let mut debug_trait_builder = f.debug_tuple("Sub");
                        debug_trait_builder.finish()
                    }
                    (&AssignOp::Mul,) => {
                        let mut debug_trait_builder = f.debug_tuple("Mul");
                        debug_trait_builder.finish()
                    }
                    (&AssignOp::Div,) => {
                        let mut debug_trait_builder = f.debug_tuple("Div");
                        debug_trait_builder.finish()
                    }
                    (&AssignOp::Rem,) => {
                        let mut debug_trait_builder = f.debug_tuple("Rem");
                        debug_trait_builder.finish()
                    }
                    (&AssignOp::ShiftLeft,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("ShiftLeft");
                        debug_trait_builder.finish()
                    }
                    (&AssignOp::ShiftRight,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("ShiftRight");
                        debug_trait_builder.finish()
                    }
                    (&AssignOp::BitwiseAnd,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("BitwiseAnd");
                        debug_trait_builder.finish()
                    }
                    (&AssignOp::BitwiseOr,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("BitwiseOr");
                        debug_trait_builder.finish()
                    }
                    (&AssignOp::BitwiseXor,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("BitwiseXor");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for AssignOp {
            #[inline]
            fn clone(&self) -> AssignOp { { *self } }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for AssignOp { }
        pub fn assign(pair: Pair<Rule>) -> ParseResult<Expression> {
            let mut assign = pair.into_inner();
            let ident_or_deref = assign.next_token()?;
            let access =
                match ident_or_deref.as_rule() {
                    Rule::ident => Expression::Ident(ident(ident_or_deref)?),
                    Rule::deref_expr => deref_expr(ident_or_deref)?,
                    _ =>
                    return Err(ParseError::UnexpectedToken(ident_or_deref)),
                };
            let op = assign_op(assign.next_token()?)?;
            let value = expr(assign.next_token()?)?;
            Ok(Expression::Assign(access.boxed(), op, value.boxed()))
        }
        pub fn assign_op(pair: Pair<Rule>) -> ParseResult<AssignOp> {
            {
                match (&pair.as_rule(), &Rule::assign_operator) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                             "`,\n right: `",
                                                                                             "`"],
                                                                                           &match (&&*left_val,
                                                                                                   &&*right_val)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1)
                                                                                                =>
                                                                                                [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                              ::core::fmt::Debug::fmt),
                                                                                                 ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                              ::core::fmt::Debug::fmt)],
                                                                                            }))
                            }
                        }
                    }
                }
            };
            let mut assign_operator = pair.into_inner().next_token()?;
            Ok(match assign_operator.as_rule() {
                   Rule::assign => AssignOp::Assign,
                   Rule::multiply => AssignOp::Mul,
                   Rule::divide => AssignOp::Div,
                   Rule::modulus => AssignOp::Rem,
                   Rule::plus => AssignOp::Add,
                   Rule::minus => AssignOp::Sub,
                   Rule::shift_left => AssignOp::ShiftLeft,
                   Rule::shift_right => AssignOp::ShiftRight,
                   Rule::bitwise_and => AssignOp::BitwiseAnd,
                   Rule::bitwise_or => AssignOp::BitwiseOr,
                   Rule::bitwise_xor => AssignOp::BitwiseXor,
                   _ =>
                   return Err(ParseError::UnexpectedToken(assign_operator)),
               })
        }
    }
    mod binary_op {
        use super::*;
        pub enum BinaryOp {
            Add,
            Sub,
            Rem,
            Mul,
            Div,
            ShiftLeft,
            ShiftRight,
            LessThanEqual,
            LessThan,
            GreaterThanEqual,
            GreaterThan,
            NotEqual,
            Equal,
            LogicalAnd,
            LogicalOr,
            BitwiseAnd,
            BitwiseOr,
            BitwiseXor,
            Range,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BinaryOp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&BinaryOp::Add,) => {
                        let mut debug_trait_builder = f.debug_tuple("Add");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::Sub,) => {
                        let mut debug_trait_builder = f.debug_tuple("Sub");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::Rem,) => {
                        let mut debug_trait_builder = f.debug_tuple("Rem");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::Mul,) => {
                        let mut debug_trait_builder = f.debug_tuple("Mul");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::Div,) => {
                        let mut debug_trait_builder = f.debug_tuple("Div");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::ShiftLeft,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("ShiftLeft");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::ShiftRight,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("ShiftRight");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::LessThanEqual,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("LessThanEqual");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::LessThan,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("LessThan");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::GreaterThanEqual,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("GreaterThanEqual");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::GreaterThan,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("GreaterThan");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::NotEqual,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("NotEqual");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::Equal,) => {
                        let mut debug_trait_builder = f.debug_tuple("Equal");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::LogicalAnd,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("LogicalAnd");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::LogicalOr,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("LogicalOr");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::BitwiseAnd,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("BitwiseAnd");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::BitwiseOr,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("BitwiseOr");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::BitwiseXor,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("BitwiseXor");
                        debug_trait_builder.finish()
                    }
                    (&BinaryOp::Range,) => {
                        let mut debug_trait_builder = f.debug_tuple("Range");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BinaryOp {
            #[inline]
            fn clone(&self) -> BinaryOp { { *self } }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for BinaryOp { }
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct PREC_CLIMBER {
            __private_field: (),
        }
        #[doc(hidden)]
        static PREC_CLIMBER: PREC_CLIMBER =
            PREC_CLIMBER{__private_field: (),};
        impl ::lazy_static::__Deref for PREC_CLIMBER {
            type Target = PrecClimber<Rule>;
            fn deref(&self) -> &PrecClimber<Rule> {
                #[inline(always)]
                fn __static_ref_initialize() -> PrecClimber<Rule> {
                    {
                        PrecClimber::new(<[_]>::into_vec(box
                                                             [Operator::new(Rule::range,
                                                                            Assoc::Right),
                                                              Operator::new(Rule::logical_or,
                                                                            Assoc::Left),
                                                              Operator::new(Rule::logical_and,
                                                                            Assoc::Left),
                                                              Operator::new(Rule::equal,
                                                                            Assoc::Right)
                                                                  |
                                                                  Operator::new(Rule::not_equal,
                                                                                Assoc::Right)
                                                                  |
                                                                  Operator::new(Rule::greater_than_or_equal,
                                                                                Assoc::Left)
                                                                  |
                                                                  Operator::new(Rule::less_than_or_equal,
                                                                                Assoc::Left)
                                                                  |
                                                                  Operator::new(Rule::greater_than,
                                                                                Assoc::Left)
                                                                  |
                                                                  Operator::new(Rule::less_than,
                                                                                Assoc::Left),
                                                              Operator::new(Rule::bitwise_or,
                                                                            Assoc::Left),
                                                              Operator::new(Rule::bitwise_xor,
                                                                            Assoc::Left),
                                                              Operator::new(Rule::bitwise_and,
                                                                            Assoc::Left),
                                                              Operator::new(Rule::shift_right,
                                                                            Assoc::Left)
                                                                  |
                                                                  Operator::new(Rule::shift_left,
                                                                                Assoc::Left),
                                                              Operator::new(Rule::plus,
                                                                            Assoc::Left)
                                                                  |
                                                                  Operator::new(Rule::minus,
                                                                                Assoc::Left),
                                                              Operator::new(Rule::modulus,
                                                                            Assoc::Left)
                                                                  |
                                                                  Operator::new(Rule::divide,
                                                                                Assoc::Left)
                                                                  |
                                                                  Operator::new(Rule::multiply,
                                                                                Assoc::Left)]))
                    }
                }
                #[inline(always)]
                fn __stability() -> &'static PrecClimber<Rule> {
                    static LAZY: ::lazy_static::lazy::Lazy<PrecClimber<Rule>>
                     =
                        ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for PREC_CLIMBER {
            fn initialize(lazy: &Self) { let _ = &**lazy; }
        }
        pub fn infix(pair: Pair<Rule>) -> ParseResult<Expression> {
            PREC_CLIMBER.climb(pair.into_inner(), |pair| expr(pair),
                               |lhs: ParseResult<Expression>, op: Pair<Rule>,
                                rhs: ParseResult<Expression>|
                                   {
                                       Ok(Expression::Binary{lhs:
                                                                 lhs?.boxed(),
                                                             op:
                                                                 binary_op(op)?,
                                                             rhs:
                                                                 rhs?.boxed(),})
                                   })
        }
        pub fn binary_op(pair: Pair<Rule>) -> ParseResult<BinaryOp> {
            Ok(match pair.as_rule() {
                   Rule::multiply => BinaryOp::Mul,
                   Rule::divide => BinaryOp::Div,
                   Rule::modulus => BinaryOp::Rem,
                   Rule::plus => BinaryOp::Add,
                   Rule::minus => BinaryOp::Sub,
                   Rule::shift_left => BinaryOp::ShiftLeft,
                   Rule::shift_right => BinaryOp::ShiftRight,
                   Rule::less_than_or_equal => BinaryOp::LessThanEqual,
                   Rule::less_than => BinaryOp::LessThan,
                   Rule::greater_than_or_equal => BinaryOp::GreaterThanEqual,
                   Rule::greater_than => BinaryOp::GreaterThan,
                   Rule::not_equal => BinaryOp::NotEqual,
                   Rule::equal => BinaryOp::Equal,
                   Rule::logical_and => BinaryOp::LogicalAnd,
                   Rule::logical_or => BinaryOp::LogicalOr,
                   Rule::bitwise_and => BinaryOp::BitwiseAnd,
                   Rule::bitwise_or => BinaryOp::BitwiseOr,
                   Rule::bitwise_xor => BinaryOp::BitwiseXor,
                   Rule::range => BinaryOp::Range,
                   _ => return Err(ParseError::UnexpectedToken(pair)),
               })
        }
    }
    mod binding {
        use super::*;
        pub enum Binding {
            Named {
                mutable: bool,
                ident: Ident,
            },
            Destructure(Destructure),
            SelfBinding {
                mutable: bool,
            },
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Binding {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&Binding::Named {
                     mutable: ref __self_0, ident: ref __self_1 },) => {
                        let mut debug_trait_builder = f.debug_struct("Named");
                        let _ =
                            debug_trait_builder.field("mutable",
                                                      &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("ident", &&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Binding::Destructure(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Destructure");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Binding::SelfBinding { mutable: ref __self_0 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("SelfBinding");
                        let _ =
                            debug_trait_builder.field("mutable",
                                                      &&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Binding {
            #[inline]
            fn clone(&self) -> Binding {
                match (&*self,) {
                    (&Binding::Named {
                     mutable: ref __self_0, ident: ref __self_1 },) =>
                    Binding::Named{mutable:
                                       ::core::clone::Clone::clone(&(*__self_0)),
                                   ident:
                                       ::core::clone::Clone::clone(&(*__self_1)),},
                    (&Binding::Destructure(ref __self_0),) =>
                    Binding::Destructure(::core::clone::Clone::clone(&(*__self_0))),
                    (&Binding::SelfBinding { mutable: ref __self_0 },) =>
                    Binding::SelfBinding{mutable:
                                             ::core::clone::Clone::clone(&(*__self_0)),},
                }
            }
        }
        pub enum Destructure {
            TupleDestructure(Vec<Binding>),
            StructureDestructure(StructureDestructure),
            ArrayDestructure(Vec<Binding>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Destructure {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&Destructure::TupleDestructure(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("TupleDestructure");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Destructure::StructureDestructure(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("StructureDestructure");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Destructure::ArrayDestructure(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("ArrayDestructure");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Destructure {
            #[inline]
            fn clone(&self) -> Destructure {
                match (&*self,) {
                    (&Destructure::TupleDestructure(ref __self_0),) =>
                    Destructure::TupleDestructure(::core::clone::Clone::clone(&(*__self_0))),
                    (&Destructure::StructureDestructure(ref __self_0),) =>
                    Destructure::StructureDestructure(::core::clone::Clone::clone(&(*__self_0))),
                    (&Destructure::ArrayDestructure(ref __self_0),) =>
                    Destructure::ArrayDestructure(::core::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        pub fn binding(pair: Pair<Rule>) -> ParseResult<Binding> {
            let mut binding = pair.into_inner();
            let first = binding.next_token()?;
            Ok(match first.as_rule() {
                   Rule::mut_keyword =>
                   Binding::Named{mutable: true,
                                  ident: ident(binding.next_token()?)?,},
                   Rule::ident =>
                   Binding::Named{mutable: false, ident: ident(first)?,},
                   Rule::destructure =>
                   Binding::Destructure(destructure(first)?),
                   _ => return Err(ParseError::UnexpectedToken(first)),
               })
        }
        pub fn destructure(pair: Pair<Rule>) -> ParseResult<Destructure> {
            let pair = pair.into_inner().next_token()?;
            Ok(match pair.as_rule() {
                   Rule::tuple_destructure =>
                   Destructure::TupleDestructure(tuple_array_destructure(pair)?),
                   Rule::array_destructure =>
                   Destructure::ArrayDestructure(tuple_array_destructure(pair)?),
                   Rule::struct_destructure =>
                   Destructure::StructureDestructure(struct_destructure(pair)?),
                   _ => return Err(ParseError::UnexpectedToken(pair)),
               })
        }
        pub fn tuple_array_destructure(pair: Pair<Rule>)
         -> ParseResult<Vec<Binding>> {
            let mut bindings = Vec::new();
            for pair in
                pair.into_inner().filter(|pair|
                                             pair.as_rule() == Rule::binding)
                {
                bindings.push(binding(pair)?);
            }
            Ok(bindings)
        }
    }
    mod code_block {
        use super::*;
        pub struct CodeBlock {
            pub is_unsafe: bool,
            pub stmts: Vec<Statement>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for CodeBlock {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    CodeBlock {
                    is_unsafe: ref __self_0_0, stmts: ref __self_0_1 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("CodeBlock");
                        let _ =
                            debug_trait_builder.field("is_unsafe",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("stmts",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for CodeBlock {
            #[inline]
            fn clone(&self) -> CodeBlock {
                match *self {
                    CodeBlock {
                    is_unsafe: ref __self_0_0, stmts: ref __self_0_1 } =>
                    CodeBlock{is_unsafe:
                                  ::core::clone::Clone::clone(&(*__self_0_0)),
                              stmts:
                                  ::core::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        pub fn code_block(pair: Pair<Rule>) -> ParseResult<CodeBlock> {
            let mut stmts = Vec::new();
            for stmt in pair.into_inner() { stmts.push(statement(stmt)?); }
            Ok(CodeBlock{is_unsafe: false, stmts,})
        }
        pub fn unsafe_code_block(pair: Pair<Rule>) -> ParseResult<CodeBlock> {
            let mut block = pair.into_inner();
            let unsafe_keyword_or_code_block = block.next_token()?;
            Ok(match unsafe_keyword_or_code_block.as_rule() {
                   Rule::unsafe_keyword => {
                       let mut code_block = code_block(block.next_token()?)?;
                       code_block.is_unsafe = true;
                       code_block
                   }
                   Rule::code_block =>
                   code_block(unsafe_keyword_or_code_block)?,
                   _ =>
                   return Err(ParseError::UnexpectedToken(unsafe_keyword_or_code_block)),
               })
        }
    }
    mod expr {
        use super::*;
        pub enum Expression {
            Ident(Ident),
            Literal(Lit),
            Unary(UnaryOp, Box<Expression>),
            Binary {
                lhs: Box<Expression>,
                op: BinaryOp,
                rhs: Box<Expression>,
            },
            Dereference(Box<Expression>),
            Reference {
                mutable: bool,
                value: Box<Expression>,
            },
            Cast(Box<Expression>, Vec<Type>),
            FunctionCall(Box<Expression>, Vec<Expression>),
            FieldAccess(Box<Expression>, Ident),
            TupleAccess(Box<Expression>, usize),
            ArrayAccess(Box<Expression>, Box<Expression>),
            CodeBlock(CodeBlock),
            Tuple(Vec<Expression>),
            Array(ArrayExpr),
            Struct {
                name: Ident,
                fields: Vec<StructureConstructField>,
            },
            IfExpr {
                condition: Box<Expression>,
                body: CodeBlock,
                fallback: Option<Box<Expression>>,
            },
            WhileLoop {
                label: Option<Ident>,
                condition: Box<Expression>,
                body: CodeBlock,
            },
            ForLoop {
                label: Option<Ident>,
                binding: Binding,
                iterator: Box<Expression>,
                body: CodeBlock,
            },
            InfiniteLoop {
                label: Option<Ident>,
                body: CodeBlock,
            },
            Break {
                label: Option<Ident>,
                value: Option<Box<Expression>>,
            },
            Return(Option<Box<Expression>>),
            Assign(Box<Expression>, AssignOp, Box<Expression>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Expression {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&Expression::Ident(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Ident");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Literal(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Literal");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Unary(ref __self_0, ref __self_1),) => {
                        let mut debug_trait_builder = f.debug_tuple("Unary");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Binary {
                     lhs: ref __self_0, op: ref __self_1, rhs: ref __self_2
                     },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("Binary");
                        let _ =
                            debug_trait_builder.field("lhs", &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("op", &&(*__self_1));
                        let _ =
                            debug_trait_builder.field("rhs", &&(*__self_2));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Dereference(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Dereference");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Reference {
                     mutable: ref __self_0, value: ref __self_1 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("Reference");
                        let _ =
                            debug_trait_builder.field("mutable",
                                                      &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("value", &&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Cast(ref __self_0, ref __self_1),) => {
                        let mut debug_trait_builder = f.debug_tuple("Cast");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Expression::FunctionCall(ref __self_0, ref __self_1),)
                    => {
                        let mut debug_trait_builder =
                            f.debug_tuple("FunctionCall");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Expression::FieldAccess(ref __self_0, ref __self_1),) =>
                    {
                        let mut debug_trait_builder =
                            f.debug_tuple("FieldAccess");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Expression::TupleAccess(ref __self_0, ref __self_1),) =>
                    {
                        let mut debug_trait_builder =
                            f.debug_tuple("TupleAccess");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Expression::ArrayAccess(ref __self_0, ref __self_1),) =>
                    {
                        let mut debug_trait_builder =
                            f.debug_tuple("ArrayAccess");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Expression::CodeBlock(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("CodeBlock");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Tuple(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Tuple");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Array(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Array");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Struct {
                     name: ref __self_0, fields: ref __self_1 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("Struct");
                        let _ =
                            debug_trait_builder.field("name", &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("fields",
                                                      &&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Expression::IfExpr {
                     condition: ref __self_0,
                     body: ref __self_1,
                     fallback: ref __self_2 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("IfExpr");
                        let _ =
                            debug_trait_builder.field("condition",
                                                      &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("body", &&(*__self_1));
                        let _ =
                            debug_trait_builder.field("fallback",
                                                      &&(*__self_2));
                        debug_trait_builder.finish()
                    }
                    (&Expression::WhileLoop {
                     label: ref __self_0,
                     condition: ref __self_1,
                     body: ref __self_2 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("WhileLoop");
                        let _ =
                            debug_trait_builder.field("label", &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("condition",
                                                      &&(*__self_1));
                        let _ =
                            debug_trait_builder.field("body", &&(*__self_2));
                        debug_trait_builder.finish()
                    }
                    (&Expression::ForLoop {
                     label: ref __self_0,
                     binding: ref __self_1,
                     iterator: ref __self_2,
                     body: ref __self_3 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("ForLoop");
                        let _ =
                            debug_trait_builder.field("label", &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("binding",
                                                      &&(*__self_1));
                        let _ =
                            debug_trait_builder.field("iterator",
                                                      &&(*__self_2));
                        let _ =
                            debug_trait_builder.field("body", &&(*__self_3));
                        debug_trait_builder.finish()
                    }
                    (&Expression::InfiniteLoop {
                     label: ref __self_0, body: ref __self_1 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("InfiniteLoop");
                        let _ =
                            debug_trait_builder.field("label", &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("body", &&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Break {
                     label: ref __self_0, value: ref __self_1 },) => {
                        let mut debug_trait_builder = f.debug_struct("Break");
                        let _ =
                            debug_trait_builder.field("label", &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("value", &&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Return(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Return");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Expression::Assign(ref __self_0, ref __self_1,
                                         ref __self_2),) => {
                        let mut debug_trait_builder = f.debug_tuple("Assign");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        let _ = debug_trait_builder.field(&&(*__self_2));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Expression {
            #[inline]
            fn clone(&self) -> Expression {
                match (&*self,) {
                    (&Expression::Ident(ref __self_0),) =>
                    Expression::Ident(::core::clone::Clone::clone(&(*__self_0))),
                    (&Expression::Literal(ref __self_0),) =>
                    Expression::Literal(::core::clone::Clone::clone(&(*__self_0))),
                    (&Expression::Unary(ref __self_0, ref __self_1),) =>
                    Expression::Unary(::core::clone::Clone::clone(&(*__self_0)),
                                      ::core::clone::Clone::clone(&(*__self_1))),
                    (&Expression::Binary {
                     lhs: ref __self_0, op: ref __self_1, rhs: ref __self_2
                     },) =>
                    Expression::Binary{lhs:
                                           ::core::clone::Clone::clone(&(*__self_0)),
                                       op:
                                           ::core::clone::Clone::clone(&(*__self_1)),
                                       rhs:
                                           ::core::clone::Clone::clone(&(*__self_2)),},
                    (&Expression::Dereference(ref __self_0),) =>
                    Expression::Dereference(::core::clone::Clone::clone(&(*__self_0))),
                    (&Expression::Reference {
                     mutable: ref __self_0, value: ref __self_1 },) =>
                    Expression::Reference{mutable:
                                              ::core::clone::Clone::clone(&(*__self_0)),
                                          value:
                                              ::core::clone::Clone::clone(&(*__self_1)),},
                    (&Expression::Cast(ref __self_0, ref __self_1),) =>
                    Expression::Cast(::core::clone::Clone::clone(&(*__self_0)),
                                     ::core::clone::Clone::clone(&(*__self_1))),
                    (&Expression::FunctionCall(ref __self_0, ref __self_1),)
                    =>
                    Expression::FunctionCall(::core::clone::Clone::clone(&(*__self_0)),
                                             ::core::clone::Clone::clone(&(*__self_1))),
                    (&Expression::FieldAccess(ref __self_0, ref __self_1),) =>
                    Expression::FieldAccess(::core::clone::Clone::clone(&(*__self_0)),
                                            ::core::clone::Clone::clone(&(*__self_1))),
                    (&Expression::TupleAccess(ref __self_0, ref __self_1),) =>
                    Expression::TupleAccess(::core::clone::Clone::clone(&(*__self_0)),
                                            ::core::clone::Clone::clone(&(*__self_1))),
                    (&Expression::ArrayAccess(ref __self_0, ref __self_1),) =>
                    Expression::ArrayAccess(::core::clone::Clone::clone(&(*__self_0)),
                                            ::core::clone::Clone::clone(&(*__self_1))),
                    (&Expression::CodeBlock(ref __self_0),) =>
                    Expression::CodeBlock(::core::clone::Clone::clone(&(*__self_0))),
                    (&Expression::Tuple(ref __self_0),) =>
                    Expression::Tuple(::core::clone::Clone::clone(&(*__self_0))),
                    (&Expression::Array(ref __self_0),) =>
                    Expression::Array(::core::clone::Clone::clone(&(*__self_0))),
                    (&Expression::Struct {
                     name: ref __self_0, fields: ref __self_1 },) =>
                    Expression::Struct{name:
                                           ::core::clone::Clone::clone(&(*__self_0)),
                                       fields:
                                           ::core::clone::Clone::clone(&(*__self_1)),},
                    (&Expression::IfExpr {
                     condition: ref __self_0,
                     body: ref __self_1,
                     fallback: ref __self_2 },) =>
                    Expression::IfExpr{condition:
                                           ::core::clone::Clone::clone(&(*__self_0)),
                                       body:
                                           ::core::clone::Clone::clone(&(*__self_1)),
                                       fallback:
                                           ::core::clone::Clone::clone(&(*__self_2)),},
                    (&Expression::WhileLoop {
                     label: ref __self_0,
                     condition: ref __self_1,
                     body: ref __self_2 },) =>
                    Expression::WhileLoop{label:
                                              ::core::clone::Clone::clone(&(*__self_0)),
                                          condition:
                                              ::core::clone::Clone::clone(&(*__self_1)),
                                          body:
                                              ::core::clone::Clone::clone(&(*__self_2)),},
                    (&Expression::ForLoop {
                     label: ref __self_0,
                     binding: ref __self_1,
                     iterator: ref __self_2,
                     body: ref __self_3 },) =>
                    Expression::ForLoop{label:
                                            ::core::clone::Clone::clone(&(*__self_0)),
                                        binding:
                                            ::core::clone::Clone::clone(&(*__self_1)),
                                        iterator:
                                            ::core::clone::Clone::clone(&(*__self_2)),
                                        body:
                                            ::core::clone::Clone::clone(&(*__self_3)),},
                    (&Expression::InfiniteLoop {
                     label: ref __self_0, body: ref __self_1 },) =>
                    Expression::InfiniteLoop{label:
                                                 ::core::clone::Clone::clone(&(*__self_0)),
                                             body:
                                                 ::core::clone::Clone::clone(&(*__self_1)),},
                    (&Expression::Break {
                     label: ref __self_0, value: ref __self_1 },) =>
                    Expression::Break{label:
                                          ::core::clone::Clone::clone(&(*__self_0)),
                                      value:
                                          ::core::clone::Clone::clone(&(*__self_1)),},
                    (&Expression::Return(ref __self_0),) =>
                    Expression::Return(::core::clone::Clone::clone(&(*__self_0))),
                    (&Expression::Assign(ref __self_0, ref __self_1,
                                         ref __self_2),) =>
                    Expression::Assign(::core::clone::Clone::clone(&(*__self_0)),
                                       ::core::clone::Clone::clone(&(*__self_1)),
                                       ::core::clone::Clone::clone(&(*__self_2))),
                }
            }
        }
        impl Expression {
            pub fn boxed(self) -> Box<Expression> { Box::new(self) }
        }
        pub enum ArrayExpr {
            Array(Vec<Expression>),
            Splat {
                value: Box<Expression>,
                len: Box<Expression>,
            },
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ArrayExpr {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&ArrayExpr::Array(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Array");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&ArrayExpr::Splat {
                     value: ref __self_0, len: ref __self_1 },) => {
                        let mut debug_trait_builder = f.debug_struct("Splat");
                        let _ =
                            debug_trait_builder.field("value", &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("len", &&(*__self_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ArrayExpr {
            #[inline]
            fn clone(&self) -> ArrayExpr {
                match (&*self,) {
                    (&ArrayExpr::Array(ref __self_0),) =>
                    ArrayExpr::Array(::core::clone::Clone::clone(&(*__self_0))),
                    (&ArrayExpr::Splat {
                     value: ref __self_0, len: ref __self_1 },) =>
                    ArrayExpr::Splat{value:
                                         ::core::clone::Clone::clone(&(*__self_0)),
                                     len:
                                         ::core::clone::Clone::clone(&(*__self_1)),},
                }
            }
        }
        pub fn expr(pair: Pair<Rule>) -> ParseResult<Expression> {
            Ok(match pair.as_rule() {
                   Rule::expr => expr(pair.into_inner().next_token()?)?,
                   Rule::ident => Expression::Ident(ident(pair)?),
                   Rule::literal => Expression::Literal(literal(pair)?),
                   Rule::infix => infix(pair)?,
                   Rule::code_block =>
                   Expression::CodeBlock(code_block(pair)?),
                   Rule::unsafe_code_block =>
                   Expression::CodeBlock(unsafe_code_block(pair)?),
                   Rule::assign_expr => assign(pair)?,
                   Rule::deref_expr => deref_expr(pair)?,
                   Rule::struct_construct_expr => struct_construct(pair)?,
                   Rule::reference_expr => {
                       let mut ref_expr = pair.into_inner();
                       let mut_keyword_or_expr = ref_expr.next_token()?;
                       match mut_keyword_or_expr.as_rule() {
                           Rule::mut_keyword =>
                           Expression::Reference{mutable: true,
                                                 value:
                                                     expr(ref_expr.next_token()?)?.boxed(),},
                           Rule::expr =>
                           Expression::Reference{mutable: false,
                                                 value:
                                                     expr(mut_keyword_or_expr)?.boxed(),},
                           _ =>
                           return Err(ParseError::UnexpectedToken(mut_keyword_or_expr)),
                       }
                   }
                   Rule::prefix => {
                       let mut prefix = pair.into_inner();
                       let op = unary_op(prefix.next_token()?)?;
                       let expr = expr(prefix.next_token()?)?.boxed();
                       Expression::Unary(op, expr)
                   }
                   Rule::array_lit => {
                       let mut elements = Vec::new();
                       for pair in pair.into_inner() {
                           elements.push(expr(pair)?);
                       }
                       Expression::Array(ArrayExpr::Array(elements))
                   }
                   Rule::array_splat => {
                       let mut splat = pair.into_inner();
                       let value = expr(splat.next_token()?)?.boxed();
                       let len = expr(splat.next_token()?)?.boxed();
                       Expression::Array(ArrayExpr::Splat{value, len,})
                   }
                   Rule::tuple => {
                       let mut values = Vec::new();
                       for pair in pair.into_inner() {
                           values.push(expr(pair)?);
                       }
                       Expression::Tuple(values)
                   }
                   Rule::cast_expr => {
                       let mut cast_expr = pair.into_inner();
                       let expr = expr(cast_expr.next_token()?)?;
                       let mut tys = Vec::new();
                       for ty in cast_expr { tys.push(typespec(ty)?); }
                       Expression::Cast(expr.boxed(), tys)
                   }
                   Rule::return_expr =>
                   Expression::Return(match pair.into_inner().next() {
                                          Some(pair) =>
                                          Some(expr(pair)?.boxed()),
                                          None => None,
                                      }),
                   Rule::if_expr => {
                       let mut if_expr = pair.into_inner();
                       let condition = expr(if_expr.next_token()?)?.boxed();
                       let body = code_block(if_expr.next_token()?)?;
                       let fallback =
                           match if_expr.next() {
                               Some(pair) => Some(expr(pair)?.boxed()),
                               None => None,
                           };
                       Expression::IfExpr{condition, body, fallback,}
                   }
                   Rule::break_expr => {
                       let mut label = None;
                       let mut value = None;
                       for pair in pair.into_inner() {
                           match pair.as_rule() {
                               Rule::ident => label = Some(ident(pair)?),
                               Rule::expr =>
                               value = Some(expr(pair)?.boxed()),
                               _ =>
                               return Err(ParseError::UnexpectedToken(pair)),
                           }
                       }
                       Expression::Break{label, value,}
                   }
                   Rule::inf_loop => {
                       let mut label = None;
                       let mut body = None;
                       for pair in pair.into_inner() {
                           match pair.as_rule() {
                               Rule::ident => label = Some(ident(pair)?),
                               Rule::code_block =>
                               body = Some(code_block(pair)?),
                               _ =>
                               return Err(ParseError::UnexpectedToken(pair)),
                           }
                       }
                       Expression::InfiniteLoop{label,
                                                body:
                                                    body.ok_or(ParseError::MissingToken)?,}
                   }
                   Rule::while_loop => {
                       let mut label = None;
                       let mut condition = None;
                       let mut body = None;
                       for pair in pair.into_inner() {
                           match pair.as_rule() {
                               Rule::ident => label = Some(ident(pair)?),
                               Rule::expr =>
                               condition = Some(expr(pair)?.boxed()),
                               Rule::code_block =>
                               body = Some(code_block(pair)?),
                               _ =>
                               return Err(ParseError::UnexpectedToken(pair)),
                           }
                       }
                       Expression::WhileLoop{label,
                                             condition:
                                                 condition.ok_or(ParseError::MissingToken)?,
                                             body:
                                                 body.ok_or(ParseError::MissingToken)?,}
                   }
                   Rule::for_loop => {
                       let mut label = None;
                       let mut loop_binding = None;
                       let mut iterator = None;
                       let mut body = None;
                       for pair in pair.into_inner() {
                           match pair.as_rule() {
                               Rule::ident => label = Some(ident(pair)?),
                               Rule::binding =>
                               loop_binding = Some(binding(pair)?),
                               Rule::expr =>
                               iterator = Some(expr(pair)?.boxed()),
                               Rule::code_block =>
                               body = Some(code_block(pair)?),
                               _ =>
                               return Err(ParseError::UnexpectedToken(pair)),
                           }
                       }
                       Expression::ForLoop{label,
                                           binding:
                                               loop_binding.ok_or(ParseError::MissingToken)?,
                                           iterator:
                                               iterator.ok_or(ParseError::MissingToken)?,
                                           body:
                                               body.ok_or(ParseError::MissingToken)?,}
                   }
                   _ => return Err(ParseError::UnexpectedToken(pair)),
               })
        }
        pub fn deref_expr(pair: Pair<Rule>) -> ParseResult<Expression> {
            let mut accessor = pair.into_inner();
            let explicit_or_implicit = accessor.next_token()?;
            let mut access =
                match explicit_or_implicit.as_rule() {
                    Rule::dereference =>
                    return Ok(Expression::Dereference(expr(accessor.next_token()?)?.boxed())),
                    _ => expr(explicit_or_implicit)?,
                };
            while let Some(next_access) = accessor.next() {
                access =
                    match next_access.as_rule() {
                        Rule::field_access => {
                            let ident_or_index =
                                next_access.into_inner().next_token()?;
                            match ident_or_index.as_rule() {
                                Rule::ident =>
                                Expression::FieldAccess(access.boxed(),
                                                        ident(ident_or_index)?),
                                Rule::decinteger =>
                                Expression::TupleAccess(access.boxed(),
                                                        ident_or_index.as_str().parse::<usize>()?),
                                _ =>
                                return Err(ParseError::UnexpectedToken(ident_or_index)),
                            }
                        }
                        Rule::array_access =>
                        Expression::ArrayAccess(access.boxed(),
                                                expr(next_access.into_inner().next_token()?)?.boxed()),
                        Rule::function_call => {
                            let mut params = Vec::new();
                            for param in next_access.into_inner() {
                                params.push(expr(param)?);
                            }
                            Expression::FunctionCall(access.boxed(), params)
                        }
                        _ =>
                        return Err(ParseError::UnexpectedToken(next_access)),
                    };
            }
            Ok(access)
        }
    }
    mod function {
        use super::*;
        pub struct Function {
            pub name: Ident,
            pub args: Vec<FunctionArgs>,
            pub ret: Option<Type>,
            pub code: CodeBlock,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Function {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    Function {
                    name: ref __self_0_0,
                    args: ref __self_0_1,
                    ret: ref __self_0_2,
                    code: ref __self_0_3 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("Function");
                        let _ =
                            debug_trait_builder.field("name",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("args",
                                                      &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("ret", &&(*__self_0_2));
                        let _ =
                            debug_trait_builder.field("code",
                                                      &&(*__self_0_3));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Function {
            #[inline]
            fn clone(&self) -> Function {
                match *self {
                    Function {
                    name: ref __self_0_0,
                    args: ref __self_0_1,
                    ret: ref __self_0_2,
                    code: ref __self_0_3 } =>
                    Function{name:
                                 ::core::clone::Clone::clone(&(*__self_0_0)),
                             args:
                                 ::core::clone::Clone::clone(&(*__self_0_1)),
                             ret: ::core::clone::Clone::clone(&(*__self_0_2)),
                             code:
                                 ::core::clone::Clone::clone(&(*__self_0_3)),},
                }
            }
        }
        pub struct FunctionArgs {
            pub binding: Binding,
            pub ty: Type,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for FunctionArgs {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    FunctionArgs { binding: ref __self_0_0, ty: ref __self_0_1
                    } => {
                        let mut debug_trait_builder =
                            f.debug_struct("FunctionArgs");
                        let _ =
                            debug_trait_builder.field("binding",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("ty", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for FunctionArgs {
            #[inline]
            fn clone(&self) -> FunctionArgs {
                match *self {
                    FunctionArgs { binding: ref __self_0_0, ty: ref __self_0_1
                    } =>
                    FunctionArgs{binding:
                                     ::core::clone::Clone::clone(&(*__self_0_0)),
                                 ty:
                                     ::core::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        pub fn function_item(item: Pair<Rule>) -> ParseResult<Function> {
            let mut code = Err(ParseError::MissingToken);
            let mut is_unsafe = false;
            let mut name = Err(ParseError::MissingToken);
            let mut args = Vec::new();
            let mut ret = None;
            for pair in item.into_inner() {
                match pair.as_rule() {
                    Rule::unsafe_keyword => is_unsafe = true,
                    Rule::ident => name = ident(pair),
                    Rule::function_arg => {
                        let mut arg = pair.into_inner();
                        let binding = binding(arg.next_token()?)?;
                        let ty = typespec(arg.next_token()?)?;
                        args.push(FunctionArgs{binding, ty,});
                    }
                    Rule::function_return => {
                        ret =
                            Some(typespec(pair.into_inner().next_token()?)?);
                    }
                    Rule::code_block => code = code_block(pair),
                    _ => return Err(ParseError::UnexpectedToken(pair)),
                }
            }
            let mut code: CodeBlock = code?;
            code.is_unsafe = is_unsafe;
            Ok(Function{name: name?, args, ret, code,})
        }
    }
    mod ident {
        use super::*;
        pub type Ident = String;
        pub fn ident(pair: Pair<Rule>) -> ParseResult<Ident> {
            {
                match (&pair.as_rule(), &Rule::ident) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                             "`,\n right: `",
                                                                                             "`"],
                                                                                           &match (&&*left_val,
                                                                                                   &&*right_val)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1)
                                                                                                =>
                                                                                                [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                              ::core::fmt::Debug::fmt),
                                                                                                 ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                              ::core::fmt::Debug::fmt)],
                                                                                            }))
                            }
                        }
                    }
                }
            };
            Ok(Ident::from(pair.as_str()))
        }
    }
    mod impls {
        use super::*;
        pub struct Impl {
            pub tys: Vec<BoundedType>,
            pub items: Vec<Item>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Impl {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    Impl { tys: ref __self_0_0, items: ref __self_0_1 } => {
                        let mut debug_trait_builder = f.debug_struct("Impl");
                        let _ =
                            debug_trait_builder.field("tys", &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("items",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Impl {
            #[inline]
            fn clone(&self) -> Impl {
                match *self {
                    Impl { tys: ref __self_0_0, items: ref __self_0_1 } =>
                    Impl{tys: ::core::clone::Clone::clone(&(*__self_0_0)),
                         items: ::core::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
    }
    mod item {
        use super::*;
        pub enum Item {
            Struct(Structure),
            Function(Function),
            Const(ConstItem),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Item {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&Item::Struct(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Struct");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Item::Function(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Function");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Item::Const(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Const");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Item {
            #[inline]
            fn clone(&self) -> Item {
                match (&*self,) {
                    (&Item::Struct(ref __self_0),) =>
                    Item::Struct(::core::clone::Clone::clone(&(*__self_0))),
                    (&Item::Function(ref __self_0),) =>
                    Item::Function(::core::clone::Clone::clone(&(*__self_0))),
                    (&Item::Const(ref __self_0),) =>
                    Item::Const(::core::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        pub struct ConstItem {
            pub ident: Ident,
            pub ty: Type,
            pub value: Expression,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ConstItem {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    ConstItem {
                    ident: ref __self_0_0,
                    ty: ref __self_0_1,
                    value: ref __self_0_2 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("ConstItem");
                        let _ =
                            debug_trait_builder.field("ident",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("ty", &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("value",
                                                      &&(*__self_0_2));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ConstItem {
            #[inline]
            fn clone(&self) -> ConstItem {
                match *self {
                    ConstItem {
                    ident: ref __self_0_0,
                    ty: ref __self_0_1,
                    value: ref __self_0_2 } =>
                    ConstItem{ident:
                                  ::core::clone::Clone::clone(&(*__self_0_0)),
                              ty: ::core::clone::Clone::clone(&(*__self_0_1)),
                              value:
                                  ::core::clone::Clone::clone(&(*__self_0_2)),},
                }
            }
        }
        pub fn item(item: Pair<Rule>) -> ParseResult<Item> {
            let item = item.into_inner().next_token()?;
            Ok(match item.as_rule() {
                   Rule::struct_decl => Item::Struct(struct_decl(item)?),
                   Rule::const_item => Item::Const(const_item(item)?),
                   Rule::function => Item::Function(function_item(item)?),
                   _ => return Err(ParseError::UnexpectedToken(item)),
               })
        }
        pub fn const_item(item: Pair<Rule>) -> ParseResult<ConstItem> {
            let mut item = item.into_inner();
            {
                match (&item.next_token()?.as_rule(), &Rule::const_keyword) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                             "`,\n right: `",
                                                                                             "`"],
                                                                                           &match (&&*left_val,
                                                                                                   &&*right_val)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1)
                                                                                                =>
                                                                                                [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                              ::core::fmt::Debug::fmt),
                                                                                                 ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                              ::core::fmt::Debug::fmt)],
                                                                                            }))
                            }
                        }
                    }
                }
            };
            let ident = ident(item.next_token()?)?;
            let ty = typespec(item.next_token()?)?;
            {
                match (&item.next_token()?.as_rule(), &Rule::assign) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                             "`,\n right: `",
                                                                                             "`"],
                                                                                           &match (&&*left_val,
                                                                                                   &&*right_val)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1)
                                                                                                =>
                                                                                                [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                              ::core::fmt::Debug::fmt),
                                                                                                 ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                              ::core::fmt::Debug::fmt)],
                                                                                            }))
                            }
                        }
                    }
                }
            };
            let value = expr(item.next_token()?)?;
            Ok(ConstItem{ident, ty, value,})
        }
    }
    mod literal {
        use super::*;
        pub enum Lit {
            String(String),
            RawString(String),
            Integer(String, IntegerBase),
            Float(String),
            Boolean(bool),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Lit {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&Lit::String(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("String");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Lit::RawString(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("RawString");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Lit::Integer(ref __self_0, ref __self_1),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Integer");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Lit::Float(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Float");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Lit::Boolean(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Boolean");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Lit {
            #[inline]
            fn clone(&self) -> Lit {
                match (&*self,) {
                    (&Lit::String(ref __self_0),) =>
                    Lit::String(::core::clone::Clone::clone(&(*__self_0))),
                    (&Lit::RawString(ref __self_0),) =>
                    Lit::RawString(::core::clone::Clone::clone(&(*__self_0))),
                    (&Lit::Integer(ref __self_0, ref __self_1),) =>
                    Lit::Integer(::core::clone::Clone::clone(&(*__self_0)),
                                 ::core::clone::Clone::clone(&(*__self_1))),
                    (&Lit::Float(ref __self_0),) =>
                    Lit::Float(::core::clone::Clone::clone(&(*__self_0))),
                    (&Lit::Boolean(ref __self_0),) =>
                    Lit::Boolean(::core::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        pub enum IntegerBase { Dec, Hex, Bin, Oct, }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for IntegerBase {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&IntegerBase::Dec,) => {
                        let mut debug_trait_builder = f.debug_tuple("Dec");
                        debug_trait_builder.finish()
                    }
                    (&IntegerBase::Hex,) => {
                        let mut debug_trait_builder = f.debug_tuple("Hex");
                        debug_trait_builder.finish()
                    }
                    (&IntegerBase::Bin,) => {
                        let mut debug_trait_builder = f.debug_tuple("Bin");
                        debug_trait_builder.finish()
                    }
                    (&IntegerBase::Oct,) => {
                        let mut debug_trait_builder = f.debug_tuple("Oct");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for IntegerBase {
            #[inline]
            fn clone(&self) -> IntegerBase { { *self } }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for IntegerBase { }
        pub fn literal(pair: Pair<Rule>) -> ParseResult<Lit> {
            let pair = pair.into_inner().next_token()?;
            Ok(match pair.as_rule() {
                   Rule::decinteger =>
                   Lit::Integer(pair.as_str().to_owned(), IntegerBase::Dec),
                   Rule::hexinteger =>
                   Lit::Integer(pair.as_str().to_owned(), IntegerBase::Hex),
                   Rule::octinteger =>
                   Lit::Integer(pair.as_str().to_owned(), IntegerBase::Oct),
                   Rule::bininteger =>
                   Lit::Integer(pair.as_str().to_owned(), IntegerBase::Bin),
                   Rule::float => Lit::Float(pair.as_str().to_owned()),
                   Rule::string_content =>
                   Lit::String(pair.as_str().to_owned()),
                   Rule::raw_string =>
                   Lit::RawString(pair.into_inner().next_token()?.as_str().to_owned()),
                   Rule::boolean_true => Lit::Boolean(true),
                   Rule::boolean_false => Lit::Boolean(false),
                   _ => return Err(ParseError::UnexpectedToken(pair)),
               })
        }
    }
    mod statement {
        use super::*;
        pub enum Statement {
            Local(Binding, Type, Option<Expression>),
            Expr(Expression),
            Item(Item),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Statement {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&Statement::Local(ref __self_0, ref __self_1,
                                       ref __self_2),) => {
                        let mut debug_trait_builder = f.debug_tuple("Local");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        let _ = debug_trait_builder.field(&&(*__self_2));
                        debug_trait_builder.finish()
                    }
                    (&Statement::Expr(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Expr");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Statement::Item(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Item");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Statement {
            #[inline]
            fn clone(&self) -> Statement {
                match (&*self,) {
                    (&Statement::Local(ref __self_0, ref __self_1,
                                       ref __self_2),) =>
                    Statement::Local(::core::clone::Clone::clone(&(*__self_0)),
                                     ::core::clone::Clone::clone(&(*__self_1)),
                                     ::core::clone::Clone::clone(&(*__self_2))),
                    (&Statement::Expr(ref __self_0),) =>
                    Statement::Expr(::core::clone::Clone::clone(&(*__self_0))),
                    (&Statement::Item(ref __self_0),) =>
                    Statement::Item(::core::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        pub fn statement(pair: Pair<Rule>) -> ParseResult<Statement> {
            Ok(match pair.as_rule() {
                   Rule::local => {
                       let mut local = pair.into_inner();
                       let binding = binding(local.next_token()?)?;
                       let mut ty = Type::Inferred;
                       let mut assignment = None;
                       loop  {
                           if let Some(typespec_or_assignment) = local.next()
                              {
                               match typespec_or_assignment.as_rule() {
                                   Rule::typespec => {
                                       ty = typespec(typespec_or_assignment)?;
                                       continue ;
                                   }
                                   Rule::assign => {
                                       assignment =
                                           Some(expr(local.next_token()?)?);
                                       break ;
                                   }
                                   _ =>
                                   return Err(ParseError::UnexpectedToken(typespec_or_assignment)),
                               }
                           }
                           break ;
                       }
                       Statement::Local(binding, ty, assignment)
                   }
                   Rule::expr => Statement::Expr(expr(pair)?),
                   Rule::item => Statement::Item(item(pair)?),
                   _ => return Err(ParseError::UnexpectedToken(pair)),
               })
        }
    }
    mod structure {
        use super::*;
        pub struct Structure {
            pub fields: Vec<StructureField>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Structure {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    Structure { fields: ref __self_0_0 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("Structure");
                        let _ =
                            debug_trait_builder.field("fields",
                                                      &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Structure {
            #[inline]
            fn clone(&self) -> Structure {
                match *self {
                    Structure { fields: ref __self_0_0 } =>
                    Structure{fields:
                                  ::core::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        pub struct StructureField {
            pub name: Ident,
            pub ty: Type,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for StructureField {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    StructureField { name: ref __self_0_0, ty: ref __self_0_1
                    } => {
                        let mut debug_trait_builder =
                            f.debug_struct("StructureField");
                        let _ =
                            debug_trait_builder.field("name",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("ty", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for StructureField {
            #[inline]
            fn clone(&self) -> StructureField {
                match *self {
                    StructureField { name: ref __self_0_0, ty: ref __self_0_1
                    } =>
                    StructureField{name:
                                       ::core::clone::Clone::clone(&(*__self_0_0)),
                                   ty:
                                       ::core::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        pub enum StructureConstructField {
            Captured(Ident),
            Explicit {
                name: Ident,
                value: Box<Expression>,
            },
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for StructureConstructField {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&StructureConstructField::Captured(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Captured");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&StructureConstructField::Explicit {
                     name: ref __self_0, value: ref __self_1 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("Explicit");
                        let _ =
                            debug_trait_builder.field("name", &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("value", &&(*__self_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for StructureConstructField {
            #[inline]
            fn clone(&self) -> StructureConstructField {
                match (&*self,) {
                    (&StructureConstructField::Captured(ref __self_0),) =>
                    StructureConstructField::Captured(::core::clone::Clone::clone(&(*__self_0))),
                    (&StructureConstructField::Explicit {
                     name: ref __self_0, value: ref __self_1 },) =>
                    StructureConstructField::Explicit{name:
                                                          ::core::clone::Clone::clone(&(*__self_0)),
                                                      value:
                                                          ::core::clone::Clone::clone(&(*__self_1)),},
                }
            }
        }
        pub struct StructureDestructureField {
            pub ident: Ident,
            pub binding: Binding,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for StructureDestructureField {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    StructureDestructureField {
                    ident: ref __self_0_0, binding: ref __self_0_1 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("StructureDestructureField");
                        let _ =
                            debug_trait_builder.field("ident",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("binding",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for StructureDestructureField {
            #[inline]
            fn clone(&self) -> StructureDestructureField {
                match *self {
                    StructureDestructureField {
                    ident: ref __self_0_0, binding: ref __self_0_1 } =>
                    StructureDestructureField{ident:
                                                  ::core::clone::Clone::clone(&(*__self_0_0)),
                                              binding:
                                                  ::core::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        pub struct StructureDestructure {
            pub ident: Ident,
            pub fields: Vec<StructureDestructureField>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for StructureDestructure {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    StructureDestructure {
                    ident: ref __self_0_0, fields: ref __self_0_1 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("StructureDestructure");
                        let _ =
                            debug_trait_builder.field("ident",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("fields",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for StructureDestructure {
            #[inline]
            fn clone(&self) -> StructureDestructure {
                match *self {
                    StructureDestructure {
                    ident: ref __self_0_0, fields: ref __self_0_1 } =>
                    StructureDestructure{ident:
                                             ::core::clone::Clone::clone(&(*__self_0_0)),
                                         fields:
                                             ::core::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        pub fn struct_decl(item: Pair<Rule>) -> ParseResult<Structure> {
            let mut fields = Vec::new();
            for field in
                item.into_inner().filter(|pair|
                                             pair.as_rule() ==
                                                 Rule::struct_field) {
                let mut field = field.into_inner();
                fields.push(StructureField{name: ident(field.next_token()?)?,
                                           ty:
                                               typespec(field.next_token()?)?,});
            }
            Ok(Structure{fields,})
        }
        pub fn struct_construct(pair: Pair<Rule>) -> ParseResult<Expression> {
            let mut construct = pair.into_inner();
            let name = ident(construct.next_token()?)?;
            let mut fields = Vec::new();
            for pair in construct {
                match pair.as_rule() {
                    Rule::struct_construct_field => {
                        let mut construct_field = pair.into_inner();
                        let name = ident(construct_field.next_token()?)?;
                        fields.push(match construct_field.next() {
                                        None =>
                                        StructureConstructField::Captured(name),
                                        Some(pair) =>
                                        StructureConstructField::Explicit{name,
                                                                          value:
                                                                              expr(pair)?.boxed(),},
                                    });
                    }
                    _ => return Err(ParseError::UnexpectedToken(pair)),
                }
            }
            Ok(Expression::Struct{name, fields,})
        }
        pub fn struct_destructure(pair: Pair<Rule>)
         -> ParseResult<StructureDestructure> {
            let mut fields = Vec::new();
            let mut struct_destructure = pair.into_inner();
            let name = ident(struct_destructure.next_token()?)?;
            for pair in struct_destructure {
                match pair.as_rule() {
                    Rule::struct_destructure_field => {
                        let mut struct_destructure_field = pair.into_inner();
                        let mut_keyword_or_ident =
                            struct_destructure_field.next_token()?;
                        match mut_keyword_or_ident.as_rule() {
                            Rule::mut_keyword => {
                                let field_name =
                                    ident(struct_destructure_field.next_token()?)?;
                                fields.push(StructureDestructureField{ident:
                                                                          field_name.clone(),
                                                                      binding:
                                                                          Binding::Named{mutable:
                                                                                             true,
                                                                                         ident:
                                                                                             field_name,},});
                            }
                            Rule::ident => {
                                let field_name = ident(mut_keyword_or_ident)?;
                                match struct_destructure_field.next() {
                                    Some(pair) => {
                                        fields.push(StructureDestructureField{ident:
                                                                                  field_name,
                                                                              binding:
                                                                                  binding(pair)?,});
                                    }
                                    None =>
                                    fields.push(StructureDestructureField{ident:
                                                                              field_name.clone(),
                                                                          binding:
                                                                              Binding::Named{mutable:
                                                                                                 false,
                                                                                             ident:
                                                                                                 field_name,},}),
                                }
                            }
                            _ =>
                            return Err(ParseError::UnexpectedToken(mut_keyword_or_ident)),
                        }
                    }
                    _ => return Err(ParseError::UnexpectedToken(pair)),
                }
            }
            Ok(StructureDestructure{ident: name, fields,})
        }
    }
    mod types {
        use super::*;
        pub struct BoundedType {
            pub ty: Type,
            pub bounds: Vec<()>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BoundedType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    BoundedType { ty: ref __self_0_0, bounds: ref __self_0_1 }
                    => {
                        let mut debug_trait_builder =
                            f.debug_struct("BoundedType");
                        let _ =
                            debug_trait_builder.field("ty", &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("bounds",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BoundedType {
            #[inline]
            fn clone(&self) -> BoundedType {
                match *self {
                    BoundedType { ty: ref __self_0_0, bounds: ref __self_0_1 }
                    =>
                    BoundedType{ty:
                                    ::core::clone::Clone::clone(&(*__self_0_0)),
                                bounds:
                                    ::core::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        pub enum Type {
            Array(Box<ArrayType>),
            Tuple(Vec<Type>),
            Ptr(Box<PointerType>),
            Named(Ident),
            Inferred,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Type {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&Type::Array(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Array");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Type::Tuple(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Tuple");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Type::Ptr(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Ptr");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Type::Named(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Named");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Type::Inferred,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Inferred");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Type {
            #[inline]
            fn clone(&self) -> Type {
                match (&*self,) {
                    (&Type::Array(ref __self_0),) =>
                    Type::Array(::core::clone::Clone::clone(&(*__self_0))),
                    (&Type::Tuple(ref __self_0),) =>
                    Type::Tuple(::core::clone::Clone::clone(&(*__self_0))),
                    (&Type::Ptr(ref __self_0),) =>
                    Type::Ptr(::core::clone::Clone::clone(&(*__self_0))),
                    (&Type::Named(ref __self_0),) =>
                    Type::Named(::core::clone::Clone::clone(&(*__self_0))),
                    (&Type::Inferred,) => Type::Inferred,
                }
            }
        }
        pub struct ArrayType {
            pub element: Type,
            pub len: Expression,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ArrayType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    ArrayType { element: ref __self_0_0, len: ref __self_0_1 }
                    => {
                        let mut debug_trait_builder =
                            f.debug_struct("ArrayType");
                        let _ =
                            debug_trait_builder.field("element",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("len", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ArrayType {
            #[inline]
            fn clone(&self) -> ArrayType {
                match *self {
                    ArrayType { element: ref __self_0_0, len: ref __self_0_1 }
                    =>
                    ArrayType{element:
                                  ::core::clone::Clone::clone(&(*__self_0_0)),
                              len:
                                  ::core::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        pub struct PointerType {
            pub mutable: bool,
            pub ty: Type,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for PointerType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match *self {
                    PointerType { mutable: ref __self_0_0, ty: ref __self_0_1
                    } => {
                        let mut debug_trait_builder =
                            f.debug_struct("PointerType");
                        let _ =
                            debug_trait_builder.field("mutable",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("ty", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for PointerType {
            #[inline]
            fn clone(&self) -> PointerType {
                match *self {
                    PointerType { mutable: ref __self_0_0, ty: ref __self_0_1
                    } =>
                    PointerType{mutable:
                                    ::core::clone::Clone::clone(&(*__self_0_0)),
                                ty:
                                    ::core::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        pub fn typespec(pair: Pair<Rule>) -> ParseResult<Type> {
            Ok(match pair.as_rule() {
                   Rule::typespec =>
                   return typespec(pair.into_inner().next_token()?),
                   Rule::ident => Type::Named(ident(pair)?),
                   Rule::tuple_ty => {
                       let mut tys = Vec::new();
                       for pair in pair.into_inner() {
                           tys.push(typespec(pair)?);
                       }
                       Type::Tuple(tys)
                   }
                   Rule::array_ty => {
                       let mut array_ty = pair.into_inner();
                       let element = typespec(array_ty.next_token()?)?;
                       let len = expr(array_ty.next_token()?)?;
                       Type::Array(Box::new(ArrayType{element, len,}))
                   }
                   Rule::ptr_ty => {
                       let mut ptr_ty = pair.into_inner();
                       let mutability = ptr_ty.next_token()?;
                       let mutable =
                           match mutability.as_rule() {
                               Rule::mut_keyword => true,
                               Rule::const_keyword => false,
                               _ =>
                               return Err(ParseError::UnexpectedToken(mutability)),
                           };
                       let ty = typespec(ptr_ty.next_token()?)?;
                       Type::Ptr(Box::new(PointerType{mutable, ty,}))
                   }
                   _ => return Err(ParseError::UnexpectedToken(pair)),
               })
        }
    }
    mod unary_op {
        use super::*;
        pub enum UnaryOp { Absolute, Negate, Not, }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for UnaryOp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&UnaryOp::Absolute,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Absolute");
                        debug_trait_builder.finish()
                    }
                    (&UnaryOp::Negate,) => {
                        let mut debug_trait_builder = f.debug_tuple("Negate");
                        debug_trait_builder.finish()
                    }
                    (&UnaryOp::Not,) => {
                        let mut debug_trait_builder = f.debug_tuple("Not");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for UnaryOp {
            #[inline]
            fn clone(&self) -> UnaryOp { { *self } }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for UnaryOp { }
        pub fn unary_op(pair: Pair<Rule>) -> ParseResult<UnaryOp> {
            Ok(match pair.as_rule() {
                   Rule::plus => UnaryOp::Absolute,
                   Rule::minus => UnaryOp::Negate,
                   Rule::logical_not => UnaryOp::Not,
                   _ => return Err(ParseError::UnexpectedToken(pair)),
               })
        }
    }
    pub use self::{assign::*, binary_op::*, binding::*, code_block::*,
                   expr::*, function::*, ident::*, impls::*, item::*,
                   literal::*, statement::*, structure::*, types::*,
                   unary_op::*};
    pub(crate) use pest::{error::Error, iterators::{Pair, Pairs},
                          prec_climber::{Assoc, Operator, PrecClimber},
                          Parser};
    pub(crate) use crate::{error::{ParseError, ParseResult},
                           iterators::PairsExt, Grammar, Rule};
    pub struct File {
        pub items: Vec<Item>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for File {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                File { items: ref __self_0_0 } => {
                    let mut debug_trait_builder = f.debug_struct("File");
                    let _ =
                        debug_trait_builder.field("items", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for File {
        #[inline]
        fn clone(&self) -> File {
            match *self {
                File { items: ref __self_0_0 } =>
                File{items: ::core::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    pub fn parse(file: &str) -> ParseResult<File> {
        let mut items = Vec::new();
        for pair in Grammar::parse(Rule::file, file)? {
            items.push(match pair.as_rule() {
                           Rule::item => item(pair)?,
                           Rule::EOI => break ,
                           _ => return Err(ParseError::UnexpectedToken(pair)),
                       });
        }
        Ok(File{items,})
    }
}
pub mod error {
    use thiserror::Error;
    use pest::error::Error as PestError;
    use super::Rule;
    pub type ParseResult<'i, T> = Result<T, ParseError<'i>>;
    pub enum ParseError<'i> {

        #[error("Grammar Parse Error {0}")]
        GrammarError(
                     #[from]
                     PestError<Rule>),

        #[error("Missing token when converting to AST")]
        MissingToken,

        #[error("Unexpected token when converting to AST: {0:?}")]
        UnexpectedToken(pest::iterators::Pair<'i, Rule>),

        #[error("Could not parse integer")]
        ParseIntError(
                      #[from]
                      std::num::ParseIntError),

        #[error("Could not parse float")]
        ParseFloatError(
                        #[from]
                        std::num::ParseFloatError),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'i> ::core::fmt::Debug for ParseError<'i> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ParseError::GrammarError(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("GrammarError");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ParseError::MissingToken,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("MissingToken");
                    debug_trait_builder.finish()
                }
                (&ParseError::UnexpectedToken(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UnexpectedToken");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ParseError::ParseIntError(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ParseIntError");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ParseError::ParseFloatError(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ParseFloatError");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl <'i> std::error::Error for ParseError<'i> {
        fn source(&self)
         -> std::option::Option<&(dyn std::error::Error + 'static)> {
            use thiserror::private::AsDynError;

            #[allow(deprecated)]
            match self {
                ParseError::GrammarError { 0: source, .. } =>
                std::option::Option::Some(source.as_dyn_error()),
                ParseError::MissingToken { .. } => std::option::Option::None,
                ParseError::UnexpectedToken { .. } =>
                std::option::Option::None,
                ParseError::ParseIntError { 0: source, .. } =>
                std::option::Option::Some(source.as_dyn_error()),
                ParseError::ParseFloatError { 0: source, .. } =>
                std::option::Option::Some(source.as_dyn_error()),
            }
        }
    }
    impl <'i> std::fmt::Display for ParseError<'i> {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter)
         -> std::fmt::Result {
            #[allow(unused_imports)]
            use thiserror::private::{DisplayAsDisplay, PathAsDisplay};

            #[allow(unused_variables, deprecated)]
            match self {
                ParseError::GrammarError(_0) =>
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["Grammar Parse Error "],
                                                                     &match (&_0.as_display(),)
                                                                          {
                                                                          (arg0,)
                                                                          =>
                                                                          [::core::fmt::ArgumentV1::new(arg0,
                                                                                                        ::core::fmt::Display::fmt)],
                                                                      })),
                ParseError::MissingToken {  } =>
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["Missing token when converting to AST"],
                                                                     &match ()
                                                                          {
                                                                          ()
                                                                          =>
                                                                          [],
                                                                      })),
                ParseError::UnexpectedToken(_0) =>
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["Unexpected token when converting to AST: "],
                                                                     &match (&_0,)
                                                                          {
                                                                          (arg0,)
                                                                          =>
                                                                          [::core::fmt::ArgumentV1::new(arg0,
                                                                                                        ::core::fmt::Debug::fmt)],
                                                                      })),
                ParseError::ParseIntError(_0) =>
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["Could not parse integer"],
                                                                     &match ()
                                                                          {
                                                                          ()
                                                                          =>
                                                                          [],
                                                                      })),
                ParseError::ParseFloatError(_0) =>
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["Could not parse float"],
                                                                     &match ()
                                                                          {
                                                                          ()
                                                                          =>
                                                                          [],
                                                                      })),
            }
        }
    }
    impl <'i> std::convert::From<PestError<Rule>> for ParseError<'i> {
        #[allow(deprecated)]
        fn from(source: PestError<Rule>) -> Self {
            ParseError::GrammarError{0: source,}
        }
    }
    impl <'i> std::convert::From<std::num::ParseIntError> for ParseError<'i> {
        #[allow(deprecated)]
        fn from(source: std::num::ParseIntError) -> Self {
            ParseError::ParseIntError{0: source,}
        }
    }
    impl <'i> std::convert::From<std::num::ParseFloatError> for ParseError<'i>
     {
        #[allow(deprecated)]
        fn from(source: std::num::ParseFloatError) -> Self {
            ParseError::ParseFloatError{0: source,}
        }
    }
}
pub mod iterators {
    use pest::iterators::{Pair, Pairs};
    use crate::{error::{ParseError, ParseResult}, Rule};
    pub trait PairsExt<'i> {
        fn next_token(&mut self)
        -> Result<Pair<'i, Rule>, ParseError<'i>>;
    }
    impl <'i> PairsExt<'i> for Pairs<'i, Rule> where
     Pairs<'i, Rule>: Iterator<Item = Pair<'i, Rule>> {
        fn next_token(&mut self) -> Result<Pair<'i, Rule>, ParseError<'i>> {
            self.next().ok_or(ParseError::MissingToken)
        }
    }
}
