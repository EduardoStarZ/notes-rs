use crate::schema::{note, user};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    pub id: &'a i32,
    pub name: &'a str,
    pub active: &'a bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::note)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = note)]
pub struct NewNote<'a> {
    pub id: &'a i32,
    pub name: &'a str,
    pub content: &'a String,
    pub user_id: &'a i32,
}
