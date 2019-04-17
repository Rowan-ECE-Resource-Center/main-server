use diesel;
use diesel::mysql::Mysql;
use diesel::mysql::MysqlConnection;
use diesel::query_builder::AsQuery;
use diesel::query_builder::BoxedSelectStatement;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::TextExpressionMethods;

use log::{trace,info,warn,error,debug};

use crate::errors::WebdevError;
use crate::errors::WebdevErrorKind;

use crate::search::NullableSearch;
use crate::search::Search;

use crate::departments::models::{
    NewUserDepartment, PartialUserDepartment, SearchUserDepartments, UserDepartment, UserDepartmentList, UserDepartmentRequest, UserDepartmentResponse,
    NewDepartment, PartialDepartment, SearchDepartments, Department, DepartmentList, DepartmentRequest, DepartmentResponse,
	UserDepartmentJoin,UserDepartmentJoinList,
};
use crate::users::schema::users as users_schema;
use crate::departments::schema::departments as departments_schema;
use crate::departments::schema::user_departments as user_departments_schema;

pub fn handle_department(
    request: DepartmentRequest,
    database_connection: &MysqlConnection,
) -> Result<DepartmentResponse, WebdevError> {
    match request {
        DepartmentRequest::SearchDepartments(department) => {
            search_departments(department, database_connection).map(|u| DepartmentResponse::ManyDepartments(u))
        }
        DepartmentRequest::GetDepartment(id) => {
            get_department(id, database_connection).map(|u| DepartmentResponse::OneDepartment(u))
        }
        DepartmentRequest::CreateDepartment(department) => {
            create_department(department, database_connection).map(|u| DepartmentResponse::OneDepartment(u))
        }
        DepartmentRequest::UpdateDepartment(id, department) => {
            update_department(id, department, database_connection).map(|_| DepartmentResponse::NoResponse)
        }
        DepartmentRequest::DeleteDepartment(id) => {
            delete_department(id, database_connection).map(|_| DepartmentResponse::NoResponse)
        }
    }
}

pub fn handle_user_department(
    request: UserDepartmentRequest,
    database_connection: &MysqlConnection,
) -> Result<UserDepartmentResponse, WebdevError> {
    match request {
        UserDepartmentRequest::SearchUserDepartments(user_department) => {
            search_user_departments(user_department, database_connection).map(|u| UserDepartmentResponse::ManyUserDepartments(u))
        }
        UserDepartmentRequest::GetUserDepartment(id) => {
            get_user_department(id, database_connection).map(|u| UserDepartmentResponse::OneUserDepartment(u))
        }
        UserDepartmentRequest::CreateUserDepartment(user_department) => {
            create_user_department(user_department, database_connection).map(|u| UserDepartmentResponse::OneUserDepartment(u))
        }
        UserDepartmentRequest::UpdateUserDepartment(id, user_department) => {
            update_user_department(id, user_department, database_connection).map(|_| UserDepartmentResponse::NoResponse)
        }
        UserDepartmentRequest::DeleteUserDepartment(id) => {
            delete_user_department(id, database_connection).map(|_| UserDepartmentResponse::NoResponse)
        }
    }
}

fn search_user_departments(
    user_department: SearchUserDepartments,
    database_connection: &MysqlConnection,
) -> Result<UserDepartmentJoinList, WebdevError> {
    //let mut user_departments_query = user_departments_schema::table.as_query().into_boxed();
		joinable!( user_departments_schema-> users_schema (user_id));
		joinable!( user_departments_schema-> departments_schema (department_id));
		allow_tables_to_appear_in_same_query!(user_departments_schema, users_schema,departments_schema);
	debug!("Creating query");	
	let mut user_departments_query =user_departments_schema::table
		.inner_join(users_schema::table)
		.inner_join(departments_schema::table)
		.select((user_departments_schema::id, users_schema::id,departments_schema::id,
			departments_schema::name,departments_schema::abbreviation, users_schema::first_name,
			users_schema::last_name, users_schema::email, users_schema::banner_id))
		.into_boxed();
    match user_department.user_id {
        Search::Partial(s) => {
			debug!("Searching user_departments for a user of {}",s);
            user_departments_query = user_departments_query.filter(user_departments_schema::user_id.eq(s));
        }
        Search::Exact(s) => {
			debug!("Searching user_departments for a user of {}",s);
            user_departments_query = user_departments_query.filter(user_departments_schema::user_id.eq(s));
        }
        Search::NoSearch => {}
    }

    match user_department.department_id {
        Search::Partial(s) => {
            user_departments_query = user_departments_query.filter(user_departments_schema::department_id.eq(s))
        }
        Search::Exact(s) => {
            user_departments_query = user_departments_query.filter(user_departments_schema::department_id.eq(s))
        }
        Search::NoSearch => {}
    }
	match user_department.user_first_name {
	    Search::Partial(s) => {
            user_departments_query = user_departments_query.filter(users_schema::first_name.like(format!("{}%", s)))
        }
        Search::Exact(s) => {
            user_departments_query = user_departments_query.filter(users_schema::first_name.eq(s))
        }
        Search::NoSearch => {}
	}
	match user_department.user_last_name {
	    Search::Partial(s) => {
            user_departments_query = user_departments_query.filter(users_schema::last_name.like(format!("{}%", s)))
        }
        Search::Exact(s) => {
            user_departments_query = user_departments_query.filter(users_schema::last_name.eq(s))
        }
        Search::NoSearch => {}
	}
	match user_department.user_email {
        NullableSearch::Partial(s) => {
            user_departments_query = user_departments_query.filter(users_schema::email.like(format!("{}%", s)))
        }

        NullableSearch::Exact(s) => {
            user_departments_query = user_departments_query.filter(users_schema::email.eq(s))
        }

        NullableSearch::Some => {
            user_departments_query = user_departments_query.filter(users_schema::email.is_not_null());
        }

        NullableSearch::None => {
            user_departments_query = user_departments_query.filter(users_schema::email.is_null());
        }
		NullableSearch::NoSearch => {}
	}
	match user_department.user_banner {
	    Search::Partial(s) => {
            user_departments_query = user_departments_query.filter(users_schema::banner_id.eq(s))
        }
        Search::Exact(s) => {
            user_departments_query = user_departments_query.filter(users_schema::banner_id.eq(s))
        }
        Search::NoSearch => {}
	}
	match user_department.department_name {
	    Search::Partial(s) => {
            user_departments_query = user_departments_query.filter(departments_schema::name.like(format!("{}%", s)))
        }
        Search::Exact(s) => {
            user_departments_query = user_departments_query.filter(departments_schema::name.eq(s))
        }
        Search::NoSearch => {}
	}
	match user_department.department_abbreviation {
	    Search::Partial(s) => {
            user_departments_query = user_departments_query.filter(departments_schema::abbreviation.like(format!("{}%", s)))
        }
        Search::Exact(s) => {
            user_departments_query = user_departments_query.filter(departments_schema::abbreviation.eq(s))
        }
        Search::NoSearch => {}
	}
	//user_departments_query = user_departments_query.inner_join(users_schema::table).inner_join(departments_schema::table);
    let found_user_departments = user_departments_query.load::<UserDepartmentJoin>(database_connection)?;
    let user_department_list = UserDepartmentJoinList { user_departments: found_user_departments };

    Ok(user_department_list)
}
fn search_departments(
    department: SearchDepartments,
    database_connection: &MysqlConnection,
) -> Result<DepartmentList, WebdevError> {
    let mut departments_query = departments_schema::table.as_query().into_boxed();

    match department.name {
        Search::Partial(s) => {
            departments_query = departments_query.filter(departments_schema::name.like(format!("{}%", s)))
        }
        Search::Exact(s) => {
            departments_query = departments_query.filter(departments_schema::name.eq(s))
        }
        Search::NoSearch => {}
    }

    match department.abbreviation {
        Search::Partial(s) => {
            departments_query = departments_query.filter(departments_schema::abbreviation.like(format!("{}%", s)))
        }
        Search::Exact(s) => {
            departments_query = departments_query.filter(departments_schema::abbreviation.eq(s))
        }
        Search::NoSearch => {}
    }

    let found_departments = departments_query.load::<Department>(database_connection)?;
    let department_list = DepartmentList {departments: found_departments};

    Ok(department_list)
}

fn get_user_department(id: u64, database_connection: &MysqlConnection) -> Result<UserDepartment, WebdevError> {
    let mut found_user_departments = user_departments_schema::table
        .filter(user_departments_schema::id.eq(id))
        .load::<UserDepartment>(database_connection)?;

    match found_user_departments.pop() {
        Some(user_department) => Ok(user_department),
        None => Err(WebdevError::new(WebdevErrorKind::NotFound)),
    }
}
fn get_department(id: u64, database_connection: &MysqlConnection) -> Result<Department, WebdevError> {
    let mut found_departments = departments_schema::table
        .filter(departments_schema::id.eq(id))
        .load::<Department>(database_connection)?;

    match found_departments.pop() {
        Some(department) => Ok(department),
        None => Err(WebdevError::new(WebdevErrorKind::NotFound)),
    }
}


fn create_user_department(user_department: NewUserDepartment, database_connection: &MysqlConnection) -> Result<UserDepartment, WebdevError> {
    diesel::insert_into(user_departments_schema::table)
        .values(user_department)
        .execute(database_connection)?;

    let mut inserted_user_departments = user_departments_schema::table
        .filter(diesel::dsl::sql("id = LAST_INSERT_ID()"))
        .load::<UserDepartment>(database_connection)?;

    if let Some(inserted_user_departments) = inserted_user_departments.pop() {
        trace!("Successfully created user department");
		Ok(inserted_user_departments)
    } else {
        Err(WebdevError::new(WebdevErrorKind::Database))
    }
}
fn create_department(department: NewDepartment, database_connection: &MysqlConnection) -> Result<Department, WebdevError> {
    diesel::insert_into(departments_schema::table)
        .values(department)
        .execute(database_connection)?;

    let mut inserted_department = departments_schema::table
        .filter(diesel::dsl::sql("id = LAST_INSERT_ID()"))
        .load::<Department>(database_connection)?;

    if let Some(inserted_department) = inserted_department.pop() {
		trace!("Successfully created department");
        Ok(inserted_department)
    } else {
        Err(WebdevError::new(WebdevErrorKind::Database))
    }
}

fn update_user_department(
    id: u64,
    user_department: PartialUserDepartment,
    database_connection: &MysqlConnection,
) -> Result<(), WebdevError> {
    diesel::update(user_departments_schema::table)
        .filter(user_departments_schema::id.eq(id))
        .set(&user_department)
        .execute(database_connection)?;
    Ok(())
}
fn update_department(
    id: u64,
    department: PartialDepartment,
    database_connection: &MysqlConnection,
) -> Result<(), WebdevError> {
    diesel::update(departments_schema::table)
        .filter(departments_schema::id.eq(id))
        .set(&department)
        .execute(database_connection)?;
    Ok(())
}

fn delete_user_department(id: u64, database_connection: &MysqlConnection) -> Result<(), WebdevError> {
    diesel::delete(user_departments_schema::table.filter(user_departments_schema::id.eq(id)))
        .execute(database_connection)?;

    Ok(())
}
fn delete_department(id: u64, database_connection: &MysqlConnection) -> Result<(), WebdevError> {
    diesel::delete(departments_schema::table.filter(departments_schema::id.eq(id)))
        .execute(database_connection)?;

    Ok(())
}
