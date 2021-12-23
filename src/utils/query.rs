use crate::ScyllaConnectionManager;
use crate::AppError;

use scylla::QueryResult;
use scylla::IntoTypedRows;
use r2d2::PooledConnection;
use scylla::transport::errors::QueryError;
use scylla::frame::response::cql_to_rust::FromRow;

pub trait ConnectionResult {
	fn conn_result(&self) -> Result<PooledConnection<ScyllaConnectionManager>, actix_web::Error>;
}

pub trait GetQueryResult<T> {
	type Request;
	fn get_query_result(self) -> Result<Option<Vec<Self::Request>>, actix_web::Error>;
}

impl<T: FromRow> GetQueryResult<T> for Result<QueryResult, QueryError> {
    type Request = T;
	fn get_query_result(self) -> Result<Option<Vec<Self::Request>>, actix_web::Error> {
		self
		.map_err(|err| AppError::from(err).into())
		.map(|res| {
			res.rows.map(|rows| {
				rows.into_typed::<Self::Request>()
					.map(|a| a.unwrap())
					.collect::<Vec<Self::Request>>()
			})
		})
    }
}
