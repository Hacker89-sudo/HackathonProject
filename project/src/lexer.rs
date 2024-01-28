use santiago::lexer::LexerRules;

#[allow(unused)]
pub fn rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "STRING"            = pattern r#"\"[^"]*\""#;
        "DEFAULT" | ""                  = pattern r#"//[^\n]*"#         =>  |lexer| {lexer.skip()};
        "DEFAULT" | ""                  = pattern r#"/\*(.|\s)*\*/"#    =>  |lexer| {lexer.skip()};
        "DEFAULT" | "INTEGER"           = pattern r#"\d+"#;
        "DEFAULT" | "FLOAT"             = pattern r#"\d+\.\d+"#;
        "DEFAULT" | "FLOAT"             = pattern r#"\.\d+"#;
        "DEFAULT" | "="                 = string  r#"="#;
        "DEFAULT" | "+"                 = string  r#"+"#;
        "DEFAULT" | "-"                 = string  r#"-"#;
        "DEFAULT" | "*"                 = string  r#"*"#;
        "DEFAULT" | "/"                 = string  r#"/"#;
        "DEFAULT" | "^"                 = string  r#"^"#;
        "DEFAULT" | "%"                 = string  r#"%"#;
        "DEFAULT" | "<"                 = string  r#"<"#;
        "DEFAULT" | ">"                 = string  r#">"#;
        "DEFAULT" | "<="                = string  r#"<="#;
        "DEFAULT" | ">="                = string  r#">="#;
        "DEFAULT" | "=="                = string  r#"=="#;
        "DEFAULT" | "!="                = string  r#"!="#;
        "DEFAULT" | "<="                = string  r#"<="#;
        "DEFAULT" | ":"                 = string  r#":"#;
        "DEFAULT" | ","                 = string  r#","#;
        "DEFAULT" | "("                 = string  r#"("#;
        "DEFAULT" | ")"                 = string  r#")"#;
        "DEFAULT" | "{"                 = string  r#"{"#;
        "DEFAULT" | "}"                 = string  r#"}"#;
        "DEFAULT" | "["                 = string  r#"["#;
        "DEFAULT" | "]"                 = string  r#"]"#;
        "DEFAULT" | "FN"                = string  r#"fn"#;
        "DEFAULT" | "$"                 = string  r#"$"#;
        "DEFAULT" | ";"                 = string  r#";"#;
        "DEFAULT" | "WHILE"             = string  r#"while"#;
        "DEFAULT" | "NAME"              = pattern r#"[a-zA-Z_][a-zA-Z0-9_]*"#;
        "DEFAULT" | ""                  = pattern r#"\s"#              =>  |lexer| {lexer.skip()};
    )
}

#[test]
pub fn test() -> () {
    let text = 
"

fn fib(x) {
    x <= 2 : x,
    fib(x-1) + fib(x-2)
}

x = fib(8);

a = 1;
b = 1;

while [1] {
    a = a + b;
    b = a + b;
    disp(a);
    disp(b);
}

";

    let rules = rules();

    let tokens = santiago::lexer::lex(&rules, &text).unwrap();

    println!("{:#?}", tokens);
}