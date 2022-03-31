mod validation;
mod query;

use uuid::Uuid;

pub use validation::{SessionClaims, validate_user_credentials};
pub use query::{GetQueryResult};

pub trait ParseUuid {
    fn to_uuid(self) -> Result<Uuid, crate::AppError>;
}

impl ParseUuid for &String {
    fn to_uuid(self) -> Result<Uuid, crate::AppError> {
        Ok(Uuid::parse_str(self)?)
    }
}