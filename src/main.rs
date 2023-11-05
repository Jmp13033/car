#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]  
extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use rocket::{get, post, put, delete, routes}; 
mod schema;
mod models; 
use models::{NewCar, Car, UpdateCar}; 

use rocket_contrib::json::{Json, JsonValue};
use serde_json::json;
use crate::schema::car::dsl::car;

// connection to postgres 
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}



// create car
#[post("/car", format = "json", data = "<new_car>")]
pub fn create_car(new_car: Json<NewCar>) -> Json<JsonValue> {
    // connection; 
    let connection = establish_connection();
    

    // model  
    let new_car = NewCar {
        make: new_car.make,
        model: new_car.model,
        
    };


    // added
    diesel::insert_into(crate::schema::car::dsl::car)
        .values(&new_car)
        .execute(&connection)
        .expect("Error saving new car");

    // put json here  
    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Car has been created",

    })))
}


// read 
#[get("/cars")]
pub fn get_cars() -> Json<JsonValue> {
    let connection = establish_connection();

    let cars = car.load::<Car>(&connection)
    .expect("Error loading cars");

    Json(JsonValue::from(json!({
        "cars": cars,
    })))
}

// update car
#[put("/car/<id>", data = "<update_data>")]
pub fn update_car(id: i32, update_data: Json<UpdateCar>) ->Json<JsonValue> 
{
    let connection = establish_connection();

    // Use the `update` method of the Diesel ORM to update 
    // the student's record
    let _updated_car = diesel::update(car.find(id))
        .set(&update_data.into_inner())
        .execute(&connection)
        .expect("Failed to update car");

    // Return a JSON response indicating success
    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("car {} has been updated", id ),
    })))
}


// delete car 
#[delete("/car/<id>")]
pub fn delete_cars(id: i32) -> Json<JsonValue> {
    let connection = establish_connection();

    diesel::delete(car.find(id)).execute(&connection).
    expect(&format!("Unable to find car {}", id));

    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("car with ID {} has been deleted", id),
    })))
}



// mounts
fn main() {
    rocket::ignite().mount("/", routes![
    create_car, get_cars, delete_cars, update_car 
]).launch();
}
