#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub author: String,
    pub title: String,
    pub isbn: String,
}

use super::schema::books;

#[derive(Insertable)]
#[table_name="books"]
pub struct NewBook<'a>{
    pub author: &'a str,
    pub title: &'a str,
    pub isbn: &'a str,
}
