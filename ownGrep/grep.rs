use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let argslen = args.clone().len();
    if argslen < 3 {
        println!("Lacking a file to search ");
        println!("Usage ./owngrep  STRING_TO_SEARCH FILE_TO_BE_SEARCHED");
        return;
    }else if argslen > 3 {
        println!("Too many argumments");
        println!("Usage ./owngrep  STRING_TO_SEARCH FILE_TO_BE_SEARCHED");
        return;
    }

    let file = match File::open(&Path::new(&args[2])){
        Err(why)=> panic!("couldn't open and read the file because {}", why),
        Ok(contents) => contents
    };
    
    for (line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line{
            if line.contains(&args[1]){
                println!("{} : {}", line_num+1, line);
            }
        }
    }
}