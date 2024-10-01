use console::Term;
use rusqlite::{params, Connection,Result};
use std::path::Path;
use std::env;
use std::fs;
use clap::Parser;

#[derive(Debug)]
struct Item {
    id:i32,
    name: String,
    status:String
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let db_file = "./src/db/to_do_list.db";
    if !Path::new(db_file).exists(){
        println!("did not find db");
        let conn = Connection::open(db_file)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todo(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            status TEXT NOT NULL)", 
            params![],
        )?;
    }
    let conn = Connection::open(db_file)?;
    let args: Vec<String>= env::args().collect();
    println!("{:?}", args.len());
    if args.len() == 1 {
        let mut stmt  = conn.prepare("SELECT id, name, status FROM todo")?;
        let todo_iter = stmt.query_map([],|row|{
            Ok(Item{
                id: row.get(0)?,
                name: row.get(1)?,
                status: row.get(2)?
            })
        })?;
        for item in todo_iter{
                println!("{:?}", item);
        }
    }else if args.len() > 1 {
        let action = args[1].as_str();
       let res = match action {
            "add" => add_todo(&conn, &args),
            "done" => mark_as_done(&conn, &args),
            _=> Ok(())
        };
        println!("{:?}", res);
    }
    Ok(())
}

fn add_todo(conn:&Connection, arg:&Vec<String>)->Result<()>{
    for s in &arg[2..]{
        println!("{:?}", s);
        conn.execute("INSERT INTO todo (name, status) VALUES(?1, ?2)", params![s, "inprogress"])?;
    }
    Ok(())
}
fn mark_as_done(conn:&Connection, arg:&Vec<String> ) -> Result<()>{
    for s in &arg[2..]{
        let potentialId = s.parse::<i32>();
        let finalId = match potentialId{
            Ok(value) =>value,
            Err(e)=> 0,
        };
        if finalId >0{ 
        conn.execute("UPDATE todo SET status=?1 WHERE id=?2", params!["done", finalId ])?;
        }
    }
    Ok(())
}
