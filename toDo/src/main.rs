use console::{Term, style};
use rusqlite::{params, Connection,Result};
use std::path::Path;
use std::env;

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
            let  item_re = item.unwrap_or(Item{id:0,name:"".to_string(),status:"".to_string()});
            if item_re.id!=0{
                let term = Term::stdout();
                match item_re.status.as_str() {
                    "created" => term.write_line(&format!(" {} {} ", style(item_re.id), style(item_re.name).blink())).unwrap(),
                    "progress"=> term.write_line(&format!(" {} {} ", style(item_re.id), style(item_re.name).green())).unwrap(),
                    "done"=>term.write_line(&format!(" {} {} ", style(item_re.id), style(item_re.name).red().strikethrough())).unwrap(),
                    _=>println!("nothing")
                }
            }
        }
    }else if args.len() > 1 {
        let action = args[1].as_str();
        let _res = match action {
            "add"               => add_todo(&conn, &args),
            "done"|"progress"   => change_status(&conn, &args, action),
            "delete"            => delete_todo(&conn, &args),
            _=> Ok(())
        };
    }
    Ok(())
}

fn add_todo(conn:&Connection, arg:&Vec<String>)->Result<()>{
    for s in &arg[2..]{
        println!("{:?}", s);
        conn.execute("INSERT INTO todo (name, status) VALUES(?1, ?2)", params![s, "created"])?;
    }
    Ok(())
}

fn change_status(conn:&Connection, arg:&Vec<String>, res:&str ) -> Result<()>{
    for s in &arg[2..]{
        
        let final_id = s.parse::<i32>().unwrap_or(0);
        if final_id >0{ 
        conn.execute("UPDATE todo SET status=?1 WHERE id=?2", params![res.to_string(), final_id ])?;
        }
    }
    Ok(())
}

fn delete_todo(conn:&Connection, arg:&Vec<String>) -> Result<()>{
    for s in &arg[2..]{
        
        let final_id = s.parse::<i32>().unwrap_or(0);
        if final_id >0{ 
        conn.execute("DELETE FROM todo WHERE id=?1", params![final_id ])?;
        }
    }
    Ok(())
}