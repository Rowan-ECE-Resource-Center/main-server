# backend
Coded in Rust, manages database manipulation using AJAX requests from frontend.

## Dependencies:
* [Rouille 3.0.0](https://github.com/tomaka/rouille)
* [Diesel 1.3.3](https://github.com/diesel-rs/diesel)
* [dotenv 0.13.0](https://github.com/sgrif/rust-dotenv)
* [serde 1.0](https://github.com/serde-rs/serde)
* [serde_json 1.0](https://github.com/serde-rs/json)
* [log 0.4](https://github.com/rust-lang-nursery/log)
* [simplelog](https://github.com/drakulix/simplelog.rs)

## API Calls
#### Users
`GET /users/`
Gets information about every user in the system. Returns a List of Users.

`GET /users/?searchterm=searchtype,parameter`
Searches users
  + Available search terms: first_name, last_name, email, and banner_id
  + Available search types: partial, exact
  + Parameter is what needs to be searched
  + Multiple searches can be done by seperating them with an &

`GET /users/{id: u64}`
Gets information about the user with the given id. Returns a single User.

`POST /users`
Creates a new user. The body of POST should be a valid User. Returns the id of the created user.

`POST /users/{id: u64}`
Updates a given user.

`DELETE /users/{id: u64}`
Deletes a given user
#### Departments
`GET /departments/`
Gets all departments, with their ID, name, and abbrevation

`GET /departments/?searchterm=searchtype,parameter` 
Searches departments
  + Available search terms: name, abbreviation
  + Available search types: partial, exact  
  + Parameter is what needs to be searched  
  + Multiple searches can be done by seperating them with an &
  
`POST /departments`
Creates a new department. The body of POST should be a valid Department. Returns the id of the created department.

`POST /departments/{id: u64}`
Updates a given department.

`DELETE /departments/{id: u64}`
Deletes a given department
#### User Departments
`GET /user_departments/`
Gets all user_departments, with their ID, the user's first and last name, and the department name.


This is done by joining the users and departments tables via the foreign keys in the user_department table
  
`GET /user_departments/?searchterm=searchtype,parameter`
Searches user_departments
  + Available search terms: department_name, department_abbreviation, user_first_name, user_last_name
  + Available search types: partial, exact
  + Parameter is what needs to be searched
  + Multiple searches can be done by seperating them with an &

`POST /user_departments`
Creates a new user_department. The body of POST should be a valid User_Department. Returns the id of the created user_department.

`POST /user_departments/{id: u64}`
Updates a given user_department.

`DELETE /user_departments/{id: u64}`
Deletes a given user_department

  
## Data Models

Many of the API calls share a common set of data models, represented in JSON format.

#### User
| Property Name | Type   | Optional | Description |
|---------------|--------|----------|-------------|
| id            | u64    | No       | The internal id of the user |
| first_name    | String | No       | The first name of the user |
| last_name     | String | No       | The last name of the user |
| banner_id     | u64    | No       | The banner id of the user |
| email         | String | Yes      | The Rowan email of the user. If the user does not have an email, this will be null of non-existent |
```
{
    "id": 11,
    "first_name": "John"
    "last_name": "Smith",
    "banner_id": 9162xxxxx,
    "email": "smithj1@students.rowan.edu"
}
```

#### partial User
| Property Name | Type   | Optional | Description |
|---------------|--------|----------|-------------|
| first_name    | String | Yes      | The first name of the user |
| last_name     | String | Yes      | The last name of the user |
| banner_id     | u64    | Yes      | The banner id of the user |
| email         | String | Yes      | The Rowan email of the user. If the user does not have an email, this will be null of non-existent |
```
{
    "first_name": "John"
    "last_name": "Smith",
    "banner_id": 9162xxxxx,
    "email": "smithj1@students.rowan.edu"
}
```

#### List of Users
| Property Name | Type          | Optional | Description     |
|---------------|---------------|----------|-----------------|
| users         | List of Users | No       | A list of Users |
```
{
    "users": [
    {
    "first_name": "John"
    "last_name": "Smith",
    "banner_id": 9162xxxxx,
    "email": "smithj1@students.rowan.edu"
    },
    {
    "first_name": "Mike"
    "last_name": "Johnson",
    "banner_id": 9162xxxxx,
    "email": "johnsonm1@students.rowan.edu"
    }
    ]
}
```

#### Departments
| Property Name | Type   | Optional | Description                                                          |
|---------------|--------|----------|----------------------------------------------------------------------|
| id            | u64    | No       | The internal id of the department                                    |
| name          | String | No       | The name of the department                                           |
| abbreviation  | String | No       | The abbreviation for the department according to Rowan Section Tally |

```
{
    "departments": [
    {
    "name": "Electrical and Computer Engineering"
    "abbreviation": "ECE",
    },
    {
    "name": Mechanical Engineering"
    "abbreviation": "MECH",
    }
    ]
}
```
#### User_Departments
| Property Name | Type   | Optional | Description                                      |
|---------------|--------|----------|--------------------------------------------------|
| id            | u64    | No       | The internal id of the user_department           |
| user_id       | u64    | No       | The internal id of the user                      |
| department_id | u64    | No       | The internal id of the department the user is in |

```
{
    "user_departments": [
    {
    "department_name": "Electrical and Computer Engineering"
    "user_first_name": "Nicholas",
    "user_last_name": "Kluzynski"
    },
    {
    "department_name": "Mechanical Engineering"
    "user_first_name": "Lizzy",
    "user_last_name": "Amory"
    }
    ]
}
```
For user departments, this is what is returned from the search. To send a POST, it needs the following:
```
{
  "user_id":2,
  "department_id":4
}
```
