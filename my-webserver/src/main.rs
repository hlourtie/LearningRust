use routes::date::{get_current_date, date_plus_one_month};

// import Rocket
#[macro_use] extern crate rocket;
//import our different modules with routes and services
mod routes;
mod services;
// This is our get route 

#[get("/")]
fn say_hello() -> &'static str{
    "Hello, welcome to the api"
}

//start the webserver and mount or get route at "/hello"
#[launch]
fn rocket() -> _ { 
   rocket::build().mount("/hello", routes![say_hello, get_current_date,date_plus_one_month] )
}