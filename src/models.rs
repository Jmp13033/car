use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Serialize, Deserialize}; 
use diesel::prelude::*;
use super::schema::car;

// query the car
#[derive(Queryable, Serialize,  Deserialize)]
pub struct Car {
    pub id: i32, 
    pub make: String, 
    pub model: String, 
}



// insert into car
#[derive(Insertable, Queryable, Deserialize)]
#[table_name = "car"]
pub struct NewCar<'a> {
    pub make: &'a str,
    pub model: &'a str,
    
}



// update car
#[derive(Deserialize, AsChangeset)]
#[table_name = "car"]
pub struct UpdateCar {
    pub make: Option<String>,
    pub model: Option<String>,  
}





















