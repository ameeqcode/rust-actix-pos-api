
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::pos_items::POSItems;
use crate::repository::schema::pos_items::dsl::*;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {

    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }


pub fn import_pos_items(&self, data: Vec<POSItems>) -> Result<(), diesel::result::Error> { 
    let mut connection_ = self.pool.get().unwrap();

    for inner_data in data {   
       let existing_row = pos_items  
            .filter(code.eq(&inner_data.code))
            .select(POSItems::as_select())
            .first::<POSItems>(&mut connection_)
            .optional()?;
       if let Some(_) = existing_row{           //if item exists, update the row
        let _ = diesel::update(pos_items.filter(code.eq(&inner_data.code)))
            .set(&inner_data)
            .execute(&mut connection_);
       }   
       else{                                //if item does not exist, insert new row
        let _ = diesel::insert_into(pos_items)
            .values(&inner_data)
            .execute(&mut connection_);
         }
    }
        Ok(())
}


}
