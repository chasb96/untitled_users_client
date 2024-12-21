use prost::Message;
use std::collections::HashMap;

#[derive(Message)]
pub struct CreateUserResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

#[derive(Message)]
pub struct ListUsersResponse {
    #[prost(message, repeated, tag = "1")]
    pub users: Vec<UserResponse>,
}

#[derive(Message)]
pub struct UserResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub username: String,
    #[prost(optional, string, tag = "3")]
    pub profile_picture: Option<String>,
}

#[derive(Message)]
pub struct MapUsersResponse {
    #[prost(map = "string, message", tag = "1")]
    pub users: HashMap<String, MapUserResponse>,
}

#[derive(Message, PartialEq)]
pub struct MapUserResponse {
    #[prost(string, tag = "1")]
    pub username: String,
}