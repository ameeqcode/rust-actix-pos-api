use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset, QueryableByName, Selectable};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset, QueryableByName, Selectable)]
#[diesel(table_name = crate::repository::schema::pos_items)]
pub struct POSItems {            
    pub code: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub sub_category: Option<String>,
}
