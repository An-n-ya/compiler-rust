use compiler_core::lexer::Lexer;
use compiler_core::parser::Parser;
use compiler_core::visitors::AstPrinter;
use std::{env, io::{self, Write}};

mod compiler_core;

static PROMPT: &str = ">> ";
static HELP_MSG: &str = "
Usage:

	liu <command>

The commands are:

	lexer/lex       show the lexer structure
	parser/ast      show the ast structure
	[default]       evaluate the expression
	
";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // 如果没有带参数，暂时报错
        // FIXME 默认行为是求值
        print_help_msg();
        return;
    }
    
    match args[1].as_str() {
        "lexer" | "lex" => lexer_begin(),
        "parser" | "ast" => parser_begin(),
        _ => print_help_msg()
    };
}

fn lexer_begin() {
    let mut in_buf = "".to_string();
    loop {
        // 进入无限循环，持续解析每个输入
        print!("{PROMPT}");
        // 因为效率问题，rust的stdout默认使用缓存
        // 这会导致stdin完成后才会统一输出(将两次输出合并为一次)
        // 我们需要在每次输入前打印出promt，所以这里需要刷新stdout
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut in_buf) {
            Ok(_) => {
                let lexer = Lexer::new(&in_buf);
                for tok in lexer {
                    println!("{:?}", tok);
                }
                // 因为read_line会在buf后面append，所以每次执行完后需要clear
                in_buf.clear();
            }
            Err(error) => println!("error: {error}")
        }
    }
}

fn parser_begin() {
    let mut in_buf = "".to_string();
    loop {
        // 进入无限循环，持续解析每个输入
        print!("{PROMPT}");
        // 因为效率问题，rust的stdout默认使用缓存
        // 这会导致stdin完成后才会统一输出(将两次输出合并为一次)
        // 我们需要在每次输入前打印出promt，所以这里需要刷新stdout
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut in_buf) {
            Ok(_) => {
                let mut parser = Parser::new(&in_buf);
                let expression = parser.parse();
                let mut printer = AstPrinter::new();
                println!("{}", expression.accept(&mut printer));
                
                // 因为read_line会在buf后面append，所以每次执行完后需要clear
                in_buf.clear();
            }
            Err(error) => println!("error: {error}")
        }
    }
}


fn print_help_msg() {
    print!("{}", HELP_MSG);
}
