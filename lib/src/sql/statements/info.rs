use crate::dbs::Level;
use crate::dbs::Options;
use crate::dbs::Runtime;
use crate::dbs::Transaction;
use crate::err::Error;
use crate::sql::comment::shouldbespace;
use crate::sql::error::IResult;
use crate::sql::ident::ident_raw;
use crate::sql::value::Value;
use derive::Store;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Store)]
pub enum InfoStatement {
	Namespace,
	Database,
	Scope(String),
	Table(String),
}

impl InfoStatement {
	pub async fn compute(
		&self,
		ctx: &Runtime,
		opt: &Options,
		txn: &Transaction,
		_doc: Option<&Value>,
	) -> Result<Value, Error> {
		// Allowed to run?
		match self {
			InfoStatement::Namespace => opt.check(Level::Ns)?,
			InfoStatement::Database => opt.check(Level::Db)?,
			InfoStatement::Scope(_) => opt.check(Level::Db)?,
			InfoStatement::Table(_) => opt.check(Level::Db)?,
		}
		// Continue
		todo!()
	}
}

impl fmt::Display for InfoStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InfoStatement::Namespace => write!(f, "INFO FOR NAMESPACE"),
			InfoStatement::Database => write!(f, "INFO FOR DATABASE"),
			InfoStatement::Scope(ref s) => write!(f, "INFO FOR SCOPE {}", s),
			InfoStatement::Table(ref t) => write!(f, "INFO FOR TABLE {}", t),
		}
	}
}

pub fn info(i: &str) -> IResult<&str, InfoStatement> {
	let (i, _) = tag_no_case("INFO")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("FOR")(i)?;
	let (i, _) = shouldbespace(i)?;
	alt((namespace, database, scope, table))(i)
}

fn namespace(i: &str) -> IResult<&str, InfoStatement> {
	let (i, _) = alt((tag_no_case("NAMESPACE"), tag_no_case("NS")))(i)?;
	Ok((i, InfoStatement::Namespace))
}

fn database(i: &str) -> IResult<&str, InfoStatement> {
	let (i, _) = alt((tag_no_case("DATABASE"), tag_no_case("DB")))(i)?;
	Ok((i, InfoStatement::Database))
}

fn scope(i: &str) -> IResult<&str, InfoStatement> {
	let (i, _) = alt((tag_no_case("SCOPE"), tag_no_case("SC")))(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, scope) = ident_raw(i)?;
	Ok((i, InfoStatement::Scope(scope)))
}

fn table(i: &str) -> IResult<&str, InfoStatement> {
	let (i, _) = alt((tag_no_case("TABLE"), tag_no_case("TB")))(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, table) = ident_raw(i)?;
	Ok((i, InfoStatement::Table(table)))
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn info_query_ns() {
		let sql = "INFO FOR NAMESPACE";
		let res = info(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!(out, InfoStatement::Namespace);
		assert_eq!("INFO FOR NAMESPACE", format!("{}", out));
	}

	#[test]
	fn info_query_db() {
		let sql = "INFO FOR DATABASE";
		let res = info(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!(out, InfoStatement::Database);
		assert_eq!("INFO FOR DATABASE", format!("{}", out));
	}

	#[test]
	fn info_query_sc() {
		let sql = "INFO FOR SCOPE test";
		let res = info(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!(out, InfoStatement::Scope(String::from("test")));
		assert_eq!("INFO FOR SCOPE test", format!("{}", out));
	}

	#[test]
	fn info_query_tb() {
		let sql = "INFO FOR TABLE test";
		let res = info(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!(out, InfoStatement::Table(String::from("test")));
		assert_eq!("INFO FOR TABLE test", format!("{}", out));
	}
}
