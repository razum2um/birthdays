use actix_web::{http::StatusCode, web, HttpResponse};
use chrono::{NaiveDate, Timelike, Utc};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::app::{BirthdayAwaiter, Person};
use crate::errors::AppError;
use crate::validation::{valid_date, valid_username};
use crate::view;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BirthdayRequest {
    date_of_birth: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BirthdayResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct OnDate {
    on: Option<String>,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "birthdays")]
pub struct Birthday {
    pub username: String,
    pub birthday: NaiveDate,
}

fn http_birthday_response(message: String) -> HttpResponse {
    let now = Utc::now();
    let end_of_today = now
        .with_hour(23)
        .and_then(|t| t.with_minute(59))
        .and_then(|t| t.with_second(59))
        .unwrap_or(now);
    let sec_until_end_of_today = end_of_today.timestamp() - now.timestamp();
    let mut resp = HttpResponse::Ok()
        .insert_header(("Cache-Control", format!("public, max-age={}", sec_until_end_of_today)))
        .insert_header(("Expires", end_of_today.to_rfc2822()))
        .json(BirthdayResponse { message });
    resp.head_mut().set_camel_case_headers(true);
    resp
}

pub async fn save_birthday(
    username: web::Path<String>,
    request: web::Json<BirthdayRequest>,
    pool: web::Data<Pool>,
    schema: web::Data<&str>,
) -> Result<HttpResponse, AppError> {
    let uname = valid_username(username.to_string())?;

    let date = NaiveDate::parse_from_str(&request.date_of_birth, "%Y-%m-%d")
        .map_err(|_| AppError::InvalidDate)
        .and_then(valid_date)?;

    let stmt = include_str!("../sql/upsert_birthday.sql");
    let stmt = stmt.replace("$table_fields", &Birthday::sql_table_fields());
    let stmt = stmt.replace("$schema", &schema);

    pool.get()
        .await?
        .query(&stmt, &[&uname, &date])
        .await?
        .iter()
        .map(|row| Birthday::from_row_ref(row).unwrap())
        .collect::<Vec<Birthday>>()
        .pop()
        .map(|_| HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish())
        .ok_or(AppError::SystemError)
}

pub async fn birthday(
    username: web::Path<String>,
    on_moment: web::Query<OnDate>,
    pool: web::Data<Pool>,
    schema: web::Data<&str>,
) -> Result<HttpResponse, AppError> {
    let on_date = match &on_moment.on {
        Some(d) => NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|_| AppError::InvalidDate)?,
        None => Utc::now().date_naive(),
    };

    let stmt = include_str!("../sql/find_birthday.sql");
    let stmt = stmt.replace("$table_fields", &Birthday::sql_table_fields());
    let stmt = stmt.replace("$schema", &schema);

    pool.get()
        .await?
        .query(&stmt, &[&username.to_string()])
        .await?
        .iter()
        .map(|row| Birthday::from_row_ref(row).unwrap())
        .collect::<Vec<Birthday>>()
        .pop()
        .map(|row| Person::new(row.birthday).days_until_birthday(on_date))
        .map(|days| http_birthday_response(view::birthday_message(&username, days)))
        .ok_or(AppError::NotFound)
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
