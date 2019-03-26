use diesel;
use diesel::types;
use diesel::mysql::MysqlConnection;
use diesel::mysql::Mysql;
use diesel::query_builder::AsQuery;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::TextExpressionMethods;
use diesel::NullableExpressionMethods;
use diesel::query_builder::BoxedSelectStatement;

use crate::errors::{WebdevError, WebdevErrorKind};

use crate::search::{Search, NullableSearch};

use super::models::{
    Access, NewAccess, PartialAccess, AccessRequest, AccessResponse,

    UserAccess, NewUserAccess, PartialUserAccess, SearchUserAccess,
    UserAccessRequest, UserAccessResponse,

    JoinedUserAccess, JoinedUserAccessList
};

use crate::users::models::{User, UserList};

use crate::users::schema::users as users_schema;
use super::schema::access as access_schema;
use super::schema::user_access as user_access_schema;

pub fn handle_access(
    request: AccessRequest,
    database_connection: &MysqlConnection
) -> Result<AccessResponse, WebdevError> {
    match request {
        AccessRequest::GetAccess(id) => {
            get_access(id, database_connection).map(|a| AccessResponse::OneAccess(a))
        },
        AccessRequest::CreateAccess(access) => {
            create_access(access, database_connection).map(|a| AccessResponse::OneAccess(a))
        },
        AccessRequest::UpdateAccess(id, access) => {
            update_access(id, access, database_connection).map(|_| AccessResponse::NoResponse)
        },
        AccessRequest::DeleteAccess(id) => {
            delete_access(id, database_connection).map(|_| AccessResponse::NoResponse)
        }
    }
}

fn get_access(id: i64, database_connection: &MysqlConnection) -> Result<Access, WebdevError> {
    let mut found_access = access_schema::table
        .filter(access_schema::id.eq(id))
        .load::<Access>(database_connection)?;

    match found_access.pop() {
        Some(access) => Ok(access),
        None => Err(WebdevError::new(WebdevErrorKind::NotFound)),
    }
}

fn create_access(access: NewAccess, database_connection: &MysqlConnection) -> Result<Access, WebdevError> {
    diesel::insert_into(access_schema::table)
        .values(access)
        .execute(database_connection)?;

    no_arg_sql_function!(last_insert_id, types::Bigint);

    let mut inserted_accesses = access_schema::table
        .filter(access_schema::id.eq(last_insert_id))
        //.filter(diesel::dsl::sql("id = LAST_INSERT_ID()"))
        .load::<Access>(database_connection)?;

    if let Some(inserted_access) = inserted_accesses.pop() {
        Ok(inserted_access)
    } else {
        Err(WebdevError::new(WebdevErrorKind::Database))
    }
}

fn update_access(id: i64, access: PartialAccess, database_connection: &MysqlConnection) -> Result<(), WebdevError> {
    diesel::update(access_schema::table)
        .filter(access_schema::id.eq(id))
        .set(&access)
        .execute(database_connection)?;
    Ok(())
}

fn delete_access(id: i64, database_connection: &MysqlConnection) -> Result<(), WebdevError> {
    diesel::delete(access_schema::table.filter(access_schema::id.eq(id)))
        .execute(database_connection)?;

    Ok(())
}



pub fn handle_user_access(
    request: UserAccessRequest,
    database_connection: &MysqlConnection
) -> Result<UserAccessResponse, WebdevError> {
    match request {
        UserAccessRequest::SearchAccess(user_access) => {
            search_user_access(user_access, database_connection).map(|u| UserAccessResponse::ManyUserAccess(u))
        },
        UserAccessRequest::GetAccess(permission_id) => {
            get_user_access(permission_id, database_connection).map(|a| UserAccessResponse::OneUserAccess(a))
        },
        UserAccessRequest::CheckAccess(user_id, access_id) => {
            check_user_access(user_id, access_id, database_connection).map(|s| UserAccessResponse::AccessState(s))
        },
        UserAccessRequest::CreateAccess(user_access) => {
            create_user_access(user_access, database_connection).map(|a| UserAccessResponse::OneUserAccess(a))
        },
        UserAccessRequest::UpdateAccess(id, user_access) => {
            update_user_access(id, user_access, database_connection).map(|_| UserAccessResponse::NoResponse)
        },
        UserAccessRequest::DeleteAccess(id) => {
            delete_user_access(id, database_connection).map(|_| UserAccessResponse::NoResponse)
        }
    }
}

fn search_user_access(
    user_access_search: SearchUserAccess,
    database_connection: &MysqlConnection
) -> Result<JoinedUserAccessList, WebdevError> {
    let mut user_access_query = user_access_schema::table
        .inner_join(access_schema::table)
        .inner_join(users_schema::table)
        .select((user_access_schema::permission_id,
            users_schema::id,
            access_schema::id,
            users_schema::first_name,
            users_schema::last_name,
            users_schema::banner_id))
        .into_boxed::<Mysql>();

    match user_access_search.access_id {
        Search::Partial(s) => {
            user_access_query = user_access_query
                .filter(user_access_schema::access_id.eq(s))
        }

        Search::Exact(s) => {
            user_access_query = user_access_query
                .filter(user_access_schema::access_id.eq(s))
        }

        Search::NoSearch => {}
    }

    match user_access_search.user_id {
        Search::Partial(s) => {
            user_access_query = user_access_query
                .filter(user_access_schema::user_id.eq(s))
        }

        Search::Exact(s) => {
            user_access_query = user_access_query
                .filter(user_access_schema::user_id.eq(s))
        }

        Search::NoSearch => {}
    }

    match user_access_search.permission_level {
        NullableSearch::Partial(s) => {
            user_access_query = user_access_query
                .filter(user_access_schema::permission_level.like(format!("{}%", s)))
        }

        NullableSearch::Exact(s) => {
            user_access_query = user_access_query
                .filter(user_access_schema::permission_level.eq(s))
        }

        NullableSearch::Some => {
            user_access_query = user_access_query
                .filter(user_access_schema::permission_level.is_not_null());
        }

        NullableSearch::None => {
            user_access_query = user_access_query
                .filter(user_access_schema::permission_level.is_null());
        }

        NullableSearch::NoSearch => {}
    }

    let found_access_entries = user_access_query.load::<JoinedUserAccess>(database_connection)?;
    let joined_list = JoinedUserAccessList { entries: found_access_entries };

    Ok(joined_list)
}

fn get_user_access(
    permission_id: i64,
    database_connection: &MysqlConnection
) -> Result<UserAccess, WebdevError> {
    let mut found_user_accesses = user_access_schema::table
        .filter(user_access_schema::permission_id.eq(permission_id))
        .load::<UserAccess>(database_connection)?;

    match found_user_accesses.pop() {
        Some(found_user_access) => Ok(found_user_access),
        None => Err(WebdevError::new(WebdevErrorKind::NotFound)),
    }
}

fn check_user_access(
    user_id: i64,
    access_id: i64,
    database_connection: &MysqlConnection
) -> Result<bool, WebdevError> {
    let found_user_accesses = user_access_schema::table
        .filter(user_access_schema::user_id.eq(user_id))
        .filter(user_access_schema::access_id.eq(access_id))
        .execute(database_connection)?;

    if found_user_accesses != 0 { Ok(true) } else { Ok(false) }
}

fn create_user_access(
    user_access: NewUserAccess,
    database_connection: &MysqlConnection
) -> Result<UserAccess, WebdevError> {
    //find if permission currently exists, should not duplicate (user_id, access_id) pairs
    let found_user_accesses = user_access_schema::table
        .filter(user_access_schema::user_id.eq(user_access.user_id))
        .filter(user_access_schema::access_id.eq(user_access.access_id))
        .execute(database_connection)?;

    if found_user_accesses != 0 { return Err(WebdevError::new(WebdevErrorKind::Database)) }

    //permission most definitely does not exist at this point

    diesel::insert_into(user_access_schema::table)
        .values(user_access)
        .execute(database_connection)?;

    no_arg_sql_function!(last_insert_id, types::Bigint);

    let mut inserted_accesses = user_access_schema::table
        .filter(user_access_schema::permission_id.eq(last_insert_id))
        //.filter(diesel::dsl::sql("permission_id = LAST_INSERT_ID()"))
        .load::<UserAccess>(database_connection)?;

    if let Some(inserted_access) = inserted_accesses.pop() {
        Ok(inserted_access)
    } else {
        Err(WebdevError::new(WebdevErrorKind::Database))
    }
}

fn update_user_access(
    id: i64,
    user_access: PartialUserAccess,
    database_connection: &MysqlConnection
) -> Result<(), WebdevError> {
    diesel::update(user_access_schema::table)
        .filter(user_access_schema::permission_id.eq(id))
        .set(&user_access)
        .execute(database_connection)?;

    Ok(())
}

fn delete_user_access(id: i64, database_connection: &MysqlConnection) -> Result<(), WebdevError> {
    diesel::delete(user_access_schema::table.filter(user_access_schema::permission_id.eq(id)))
        .execute(database_connection)?;

    Ok(())
}
