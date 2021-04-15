use diesel::prelude::*;
use chrono::{NaiveDateTime};
use diesel::{RunQueryDsl};
use serde::Serialize;
use loony_service::{LoonyError, PGPooledConnection};
use super::signup::SignupFormData;
use super::update::UpdateUser;
use super::schema::users::dsl::*;
use super::schema::users::dsl::{users, id as user_id};

#[derive(Queryable, Serialize, Debug)]
pub struct ReadRow {
  pub id: i32,
  pub name: String,
  pub email: String,
  phone: Option<String>,
  password: String,
  uname: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl ReadRow {
  pub fn get_password(&self) -> &str {
    &self.password
  }
}

pub fn insert(row: &SignupFormData, conn: &PGPooledConnection) -> Result<ReadRow, LoonyError> {
  Ok(diesel::insert_into(users)
    .values(row)
    .get_result::<ReadRow>(
      conn
    )?)
}

pub fn read(user_email: &str, conn: &PGPooledConnection) -> Result<ReadRow, LoonyError> {
  Ok(users.filter(
    email.eq(user_email)
  ).first(conn)?)
}

pub fn read_rows(conn: &PGPooledConnection) -> Result<Vec<ReadRow>, LoonyError> {
  Ok(users.load::<ReadRow>(conn)?)
}

pub fn read_row_by_id(u_id: i32, conn: &PGPooledConnection) -> Result<ReadRow, LoonyError> {
  Ok(users.filter(user_id.eq(u_id)).first(conn)?)
}

pub fn update_row(u_id: i32, user: &UpdateUser, conn: &PGPooledConnection) -> Result<(), LoonyError> {
  diesel::update(
users.filter(id.eq(u_id))
  ).set(user).execute(conn)?;
  Ok(())
}