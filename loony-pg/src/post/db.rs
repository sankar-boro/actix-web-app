use diesel::{RunQueryDsl, insert_into};
use super::{create::CreatePost, update::UpdatePost};
use crate::post::schema::posts::dsl::{posts, id as post_id};
use loony_service::{LoonyError, PGPooledConnection};
use chrono::NaiveDateTime;
use serde::{Serialize};
use diesel::prelude::*;

#[derive(Queryable, Serialize, Debug)]
pub struct ReadRow {
  id: i32,
  user_id: i32,
  title: String,
  body: String,
  image: Option<String>,
  created_at: NaiveDateTime,
  updated_at: NaiveDateTime,
}

pub fn insert(row: &CreatePost, conn: &PGPooledConnection) -> Result<ReadRow, LoonyError> {
  Ok(insert_into(posts)
    .values(row)
    .get_result::<ReadRow>(
      conn
    )?)
}

pub fn read_rows(conn: &PGPooledConnection) -> Result<Vec<ReadRow>, LoonyError> {
  Ok(posts.load::<ReadRow>(conn)?)
}

pub fn read_row_by_id(p_id: i32, conn: &PGPooledConnection) -> Result<ReadRow, LoonyError> {
  Ok(posts.filter(post_id.eq(p_id)).first(conn)?)
}

pub fn update_row(p_id: i32, post: &UpdatePost, conn: &PGPooledConnection) -> Result<(), LoonyError> {
  diesel::update(
posts.filter(post_id.eq(p_id))
  ).set(post).execute(conn)?;
  Ok(())
}

pub fn delete_row(p_id: i32, conn: &PGPooledConnection) -> Result<(), LoonyError> {
  diesel::delete(
posts.filter(post_id.eq(p_id))
  ).execute(conn)?;
  Ok(())
}