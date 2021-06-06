use diesel::{RunQueryDsl, insert_into};
use super::{create::CreatePost, update::UpdatePost};
use crate::post::schema::posts::dsl::{posts, id as post_id};
use lily_service::{WebResponseError};
use chrono::NaiveDateTime;
use serde::{Serialize};
use diesel::prelude::*;
use crate::connection::PGPooledConnection;

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

pub fn insert_one(row: &CreatePost, conn: &PGPooledConnection) -> Result<ReadRow, WebResponseError> {
  Ok(insert_into(posts)
    .values(row)
    .get_result::<ReadRow>(
      conn
    )?)
}

pub fn get_all(conn: &PGPooledConnection) -> Result<Vec<ReadRow>, WebResponseError> {
  Ok(posts.load::<ReadRow>(conn)?)
}

pub fn get_one(p_id: i32, conn: &PGPooledConnection) -> Result<ReadRow, WebResponseError> {
  Ok(posts.filter(post_id.eq(p_id)).first(conn)?)
}

pub fn update_one(p_id: i32, post: &UpdatePost, conn: &PGPooledConnection) -> Result<(), WebResponseError> {
  diesel::update(
posts.filter(post_id.eq(p_id))
  ).set(post).execute(conn)?;
  Ok(())
}

pub fn delete_one(p_id: i32, conn: &PGPooledConnection) -> Result<(), WebResponseError> {
  diesel::delete(
posts.filter(post_id.eq(p_id))
  ).execute(conn)?;
  Ok(())
}