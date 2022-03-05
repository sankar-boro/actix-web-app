use scylla::QueryResult;
use scylla::IntoTypedRows;
use scylla::transport::errors::QueryError;
use scylla::frame::response::cql_to_rust::FromRow;

pub trait GetQueryResult<T> {
	type Request;
	fn get_query_result(self) -> Result<Option<Vec<T>>, crate::AppError>;
}

impl<T: FromRow> GetQueryResult<T> for Result<QueryResult, QueryError> {
    type Request = T;
	fn get_query_result(self) -> Result<Option<Vec<T>>, crate::AppError> {
		let a = self?;
		Ok(a.rows.map(|row| {
			row.into_typed::<Self::Request>()
			.filter(|d| {
				d.is_ok()
			})
			.map(|d| {
				d.unwrap()
			})
			.collect::<Vec<T>>()
		}))
    }
}