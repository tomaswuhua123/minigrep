use std::error::Error;
use std::fs; //处理与文件相关的事务
use std::env;//处理环境变量

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results=if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitives(&config.query, &contents)
    };
    for line in results{
        println!("{}",line)
    };
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive:bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        //返回Result，进行错误处理{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone(); //所要查询的字符串
        let filename = args[2].clone(); //所要查询的文件名
        //这里使用clone()来保持代码整洁性，舍去性能
        let case_sensitive=env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename,case_sensitive})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results=vec![];
    
    for line in contents.lines(){ //返回迭代器
        if line.contains(query){
            results.push(line);
        }
    };

    results
}

pub fn search_case_insensitives<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results=vec![];
    let query=query.to_ascii_lowercase();
    for line in contents.lines(){ //返回迭代器
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    };

    results
}

//以下为测试驱动开发
#[cfg(test)]
mod tests {
    use super::*;
    //引入外部所有代码
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."], search_case_insensitives(query, contents))
    }
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe,fast,productive."], search(query, contents))
    }
}
