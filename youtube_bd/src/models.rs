use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_types::Bool;

#[derive(Queryable)]
pub struct Video {
    pub id: i32,
    pub author: String,
    pub title: String,
    pub link: String,
    pub published: Bool,
}

#[derive(Insertable)]
#[table_name = "videos"]
pub struct NewVideo<'a> {
    pub author: &'a str,
    pub title: &'a str,
    pub link: &'a str,
    pub published: bool,
}
