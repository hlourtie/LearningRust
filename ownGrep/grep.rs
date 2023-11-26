use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn get_file_content(file_name : &String) -> Result<String, std::io::Error> {

    let file_to_read = File::open(&file_name)?;
    let mut bufferreader = BufReader::new(file_to_read);
    let mut file_contents = String::new();
    bufferreader.read_to_string(&mut file_contents)?;
    Ok(file_contents)
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args.clone());
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

    let file_contents = match get_file_content(&args[2]){
        Err(why)=> panic!("couldn't open and read the file because {}", why),
        Ok(contents) => contents
    };
    println!("{}",file_contents);

    
}