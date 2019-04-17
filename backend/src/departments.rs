pub mod models;
pub mod requests;
mod schema;

use self::schema::departments as departments_schema;
use self::schema::user_departments as user_departments_schema;
use diesel::expression::AsExpression;
use diesel::expression::Expression;
use diesel::mysql::Mysql;
use diesel::query_builder::InsertStatement;
use diesel::query_builder::ValuesClause;
use rouille;
