use command_line::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    //第一个项为本程序的名称，之后的项为命令行传给其的参数

    let config = Config::new(&args).unwrap_or_else(|err| {
        //闭包
        eprintln!("Problem parsing arguments:{}", err);
        //错误输出eprintln!
        process::exit(1);
        //程序终止
    });
    //模式匹配

    if let Err(e)=command_line::run(config){
        eprintln!("Application error:{}",e);
        process::exit(1);
    }
}

