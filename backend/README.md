## backend
Coded in Rust, manages database manipulation using AJAX requests from frontend.

### Dependencies:
* [Rouille 3.0.0](https://github.com/tomaka/rouille)
* [Diesel 1.3.3](https://github.com/diesel-rs/diesel)
* [Google Sign-In 0.3.0](https://github.com/wyyerd/google-signin-rs)
* [dotenv 0.13.0](https://github.com/sgrif/rust-dotenv)
* [serde 1.0](https://github.com/serde-rs/serde)
* [serde_json 1.0](https://github.com/serde-rs/json)
* [log 0.4](https://github.com/rust-lang-nursery/log)
* [simplelog](https://github.com/drakulix/simplelog.rs)

### Authentication and Authorization

#### Authentication and Authorization Flow
1. On the frontend, a sign-in button that redirects to Google sign-in calls `onSignIn()` afterwards to set a cookie for the `id_token` of the logged in user.
  * That cookie has an expire date, and will delete itself from the browser once that date has passed.
  * If a user on the frontend prompts an action that attempts to access the `id_token` cookie and it is not present, a login is automatically prompted.
2. The frontend generates an xmlHTTP request over HTTPS with `id_token` in the header.
  * Sending the ID token over HTTP exposes the user's token to packet sniffing vulnerabilities, allowing sniffers to impersonate the user by submitting unauthentic requests with the unencrypted token.
3. The backend attempts authentication before processing the requests (some requests might need authorization, some might not).
  * The token is sent back to Google's servers with our services information, Google does their own verification and sends back valid user data.
4. The email is taken from the Google user's data and cross checked with the `users` database.
  * NOTE: AN EMAIL IS NECESSARY FOR AUTHORIZED REQUESTS
5. The request is processed and just before execution of the request, the backend checks for authorization on the found user if needed.
  * A `user_access` request is made to verify authorization

#### Making Authorized API Calls

To make an authorized API call, a valid ID token from Google's sign-in services must be present in the HTTPS request.

Add the following script imports to HTML pages that make requests. 

`<script src="https://apis.google.com/js/platform.js" async defer></script>`

`<script type="text/javascript" src="/access/google_signin.js"></script>`


And add the following to JavaScript functions that make xmlHTTP requests

`mlhttp.setRequestHeader("id_token", getID_Token());`

### API Calls

`GET /users`
Gets information about every user in the system. Returns a List of Users.

`GET /users/{id: u64}`
Gets information about the user with the given id. Returns a single User.

`POST /users`
Creates a new user. The body of POST should be a valid User. Returns the id of the created user.

`POST /users/{id: u64}`
Updates a given user.

### Data Models

Many of the API calls share a common set of data models, represented in JSON format.

#### User
| Property Name | Type   | Optional | Description |
|---------------|--------|----------|-------------|
| id            | u64    | No       | The internal id of the user |
| first_name    | String | No       | The first name of the user |
| last_name     | String | no       | The last name of the user |
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

#### Partial User
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
