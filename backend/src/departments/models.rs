use diesel::Queryable;
use rouille::router;
use rouille::Request;
use serde::Deserialize;
use serde::Serialize;
use url::form_urlencoded;
use std::io::Read;

use log::trace;
use log::warn;
use log::debug;

use super::schema::user_departments;
use super::schema::departments;
use crate::users::schema::users;

use crate::errors::WebdevError;
use crate::errors::WebdevErrorKind;

use crate::search::NullableSearch;
use crate::search::Search;

use crate::departments::requests;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Department {
    pub id: u64,
    pub name: String,
    pub abbreviation: String,
}
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserDepartment {
    pub id: u64,
	pub user_id: u64,
    pub department_id: u64,
}
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserDepartmentJoin {
    pub id: u64,
	pub user_id: u64,
	pub department_id: u64,
	pub department_name: String,
	pub department_abbreviation:String,
	pub user_first_name: String,
	pub user_last_name: String,
	pub user_email: Option<String>,
	pub user_banner_id: u32,
}


#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "departments"]
pub struct NewDepartment {
    pub name: String,
    pub abbreviation: String,
}
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "user_departments"]
pub struct NewUserDepartment {
    pub user_id: u64,
    pub department_id: u64,
}


#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "departments"]
pub struct PartialDepartment {
    pub name: Option<String>,
    pub abbreviation: Option<String>,
}
#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "user_departments"]
pub struct PartialUserDepartment {
	pub user_id: Option<u64>,
    pub department_id: Option<u64>,
}

pub struct SearchDepartments {
    pub name: Search<String>,
    pub abbreviation: Search<String>,
}
pub struct SearchUserDepartments {
	pub user_id: Search<u64>,
    pub department_id: Search<u64>,
	pub user_first_name: Search<String>,
	pub user_last_name: Search<String>,
	pub user_email: NullableSearch<String>,
	pub user_banner: Search<u32>,
	pub department_name: Search<String>,
	pub department_abbreviation: Search<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DepartmentList {
    pub departments: Vec<Department>,
}
#[derive(Serialize, Deserialize)]
pub struct UserDepartmentList {
    pub user_departments: Vec<UserDepartment>,
}
#[derive(Serialize, Deserialize)]
pub struct UserDepartmentJoinList {
    pub user_departments: Vec<UserDepartmentJoin>,
}

pub enum DepartmentRequest {
    SearchDepartments(SearchDepartments),
    GetDepartment(u64),
    CreateDepartment(NewDepartment),
    UpdateDepartment(u64, PartialDepartment),
    DeleteDepartment(u64),
}
pub enum UserDepartmentRequest {
    SearchUserDepartments(SearchUserDepartments),
    GetUserDepartment(u64),
    CreateUserDepartment(NewUserDepartment),
    UpdateUserDepartment(u64, PartialUserDepartment),
    DeleteUserDepartment(u64),
}

impl DepartmentRequest {
    pub fn from_rouille(request: &rouille::Request) -> Result<DepartmentRequest, WebdevError> {
        trace!("Creating DepartmentRequest from {:#?}", request);

        let url_queries = form_urlencoded::parse(request.raw_query_string().as_bytes());

        router!(request,
            (GET) (/) => {
                let mut name_search = Search::NoSearch;
                let mut abbreviation_search = Search::NoSearch;
				

                for (field, query) in url_queries {
                    match field.as_ref() {
                        "name" => name_search = Search::from_query(query.as_ref())?,
                        "abbreviation" => abbreviation_search = Search::from_query(query.as_ref())?,
                        _ => return Err(WebdevError::new(WebdevErrorKind::Format)),
                    }
                }

                Ok(DepartmentRequest::SearchDepartments(SearchDepartments {
                    name: name_search,
                    abbreviation: abbreviation_search,
                }))
            },

            (GET) (/{id: u64}) => {
                Ok(DepartmentRequest::GetDepartment(id))
            },

            (POST) (/) => {

                let request_body = request.data().ok_or(WebdevError::new(WebdevErrorKind::FormatInvalidBody))?;
				let new_department: NewDepartment = serde_json::from_reader(request_body)?;
				debug!("Department created: {:#?}", new_department);
                Ok(DepartmentRequest::CreateDepartment(new_department))
            },

            (POST) (/{id: u64}) => {
                let request_body = request.data().ok_or(WebdevError::new(WebdevErrorKind::Format))?;
                let update_department: PartialDepartment = serde_json::from_reader(request_body)?;

                Ok(DepartmentRequest::UpdateDepartment(id, update_department))
            },

            (DELETE) (/{id: u64}) => {
				trace!("Deleting Department");
                Ok(DepartmentRequest::DeleteDepartment(id))
            },

            _ => {
                warn!("Could not create a department request for the given rouille request");
                Err(WebdevError::new(WebdevErrorKind::NotFound))
            }
        )
    }
}
impl UserDepartmentRequest {
    pub fn from_rouille(request: &rouille::Request) -> Result<UserDepartmentRequest, WebdevError> {
        trace!("Creating UserDepartmentRequest from {:#?}", request);

        let url_queries = form_urlencoded::parse(request.raw_query_string().as_bytes());


        router!(request,
            (GET) (/) => {

                let mut user_id_search = Search::NoSearch;
                let mut department_id_search = Search::NoSearch;
				let mut user_first_name_search = Search::NoSearch;
				let mut user_last_name_search = Search::NoSearch;
				let mut user_email_search = NullableSearch::NoSearch;
				let mut user_banner_search = Search::NoSearch;
				let mut department_name_search = Search::NoSearch;
				let mut department_abbreviation_search = Search::NoSearch;


                for (field, query) in url_queries {
                    match field.as_ref() {
                        "user_id" => user_id_search = Search::from_query(query.as_ref())?,
                        "department_id" => department_id_search = Search::from_query(query.as_ref())?,
						"user_first_name" =>user_first_name_search = Search::from_query(query.as_ref())?,
						"user_last_name" =>user_last_name_search = Search::from_query(query.as_ref())?,
						"user_email" => user_email_search = NullableSearch::from_query(query.as_ref())?,
						"user_banner" => user_banner_search = Search::from_query(query.as_ref())?,
						"department_name" =>department_name_search = Search::from_query(query.as_ref())?,
						"department_abbreviation" =>department_abbreviation_search = Search::from_query(query.as_ref())?,
                        _ => return Err(WebdevError::new(WebdevErrorKind::Format)),
                    }
                }

                Ok(UserDepartmentRequest::SearchUserDepartments(SearchUserDepartments {
                    user_id: user_id_search,
                    department_id: department_id_search,
					user_first_name: user_first_name_search,
					user_last_name:user_last_name_search,
					user_email:user_email_search,
					user_banner:user_banner_search,
					department_name:department_name_search,
					department_abbreviation: department_abbreviation_search,
										
                }))
            },

            (GET) (/{id: u64}) => {
                Ok(UserDepartmentRequest::GetUserDepartment(id))
            },

            (POST) (/) => {
                let request_body = request.data().ok_or(WebdevError::new(WebdevErrorKind::Format))?;
                let new_user_department: NewUserDepartment = serde_json::from_reader(request_body)?;

                Ok(UserDepartmentRequest::CreateUserDepartment(new_user_department))
            },

            (POST) (/{id: u64}) => {
                let request_body = request.data().ok_or(WebdevError::new(WebdevErrorKind::Format))?;
                let update_user_department: PartialUserDepartment = serde_json::from_reader(request_body)?;

                Ok(UserDepartmentRequest::UpdateUserDepartment(id, update_user_department))
            },

            (DELETE) (/{id: u64}) => {
                Ok(UserDepartmentRequest::DeleteUserDepartment(id))
            },

            _ => {
                warn!("Could not create a user department request for the given rouille request");
                Err(WebdevError::new(WebdevErrorKind::NotFound))
            }
        )
    }
}

pub enum UserDepartmentResponse {
    OneUserDepartment(UserDepartment),
    ManyUserDepartments(UserDepartmentJoinList),
    NoResponse,
}
impl UserDepartmentResponse {
    pub fn to_rouille(self) -> rouille::Response {
        match self {
            UserDepartmentResponse::OneUserDepartment(user_department) => rouille::Response::json(&user_department),
            UserDepartmentResponse::ManyUserDepartments(user_departments) => rouille::Response::json(&user_departments),
            UserDepartmentResponse::NoResponse => rouille::Response::empty_204(),
        }
    }
}
pub enum DepartmentResponse {
    OneDepartment(Department),
    ManyDepartments(DepartmentList),
    NoResponse,
}
impl DepartmentResponse {
    pub fn to_rouille(self) -> rouille::Response {
        match self {
            DepartmentResponse::OneDepartment(department) => rouille::Response::json(&department),
            DepartmentResponse::ManyDepartments(departments) => rouille::Response::json(&departments),
            DepartmentResponse::NoResponse => rouille::Response::empty_204(),
        }
    }
}

