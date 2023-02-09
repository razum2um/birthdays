use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use serde::Serialize;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Serialize)]
struct AppErrorBody {
    error: String,
}

impl AppErrorBody {
    fn new(error: String) -> AppErrorBody {
        AppErrorBody { error }
    }
}

#[derive(Display, From, Debug)]
pub enum AppError {
    NotFound,
    InvalidDate,
    InvalidBirthdate,
    InvalidUsername,
    SystemError,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}

#[cfg(not(tarpaulin_include))]
impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::NotFound => HttpResponse::NotFound().json(AppErrorBody::new(
              "no user found".to_string())
            ),
            AppError::InvalidDate => HttpResponse::BadRequest().json(AppErrorBody::new(
              "invalid date".to_string())
            ),
            AppError::InvalidBirthdate => HttpResponse::BadRequest().json(AppErrorBody::new(
                "dateOfBirth must be a date before the today date".to_string()
            )),
            AppError::InvalidUsername => HttpResponse::BadRequest().json(AppErrorBody::new(
                "username must contain only letters".to_string()
            )),
            _ => HttpResponse::InternalServerError().json(AppErrorBody::new(
                "internal error".to_string()
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_system_error_mapping() {
        assert_eq!(ResponseError::error_response(&AppError::NotFound).status().as_u16(), 404);
        assert_eq!(ResponseError::error_response(&AppError::InvalidDate).status().as_u16(), 400);
        assert_eq!(ResponseError::error_response(&AppError::InvalidBirthdate).status().as_u16(), 400);
        assert_eq!(ResponseError::error_response(&AppError::InvalidUsername).status().as_u16(), 400);
        assert_eq!(ResponseError::error_response(&AppError::SystemError).status().as_u16(), 500)
    }
}