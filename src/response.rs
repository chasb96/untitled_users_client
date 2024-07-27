use prost::Message;

#[derive(Message)]
pub struct CreateUserResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

#[derive(Message)]
pub struct ListUsersResponse {
    #[prost(message, repeated, tag = "1")]
    users: Vec<UserResponse>,
}

#[derive(Message)]
pub struct UserResponse {
    #[prost(string, tag = "1")]
    id: String,
    #[prost(string, tag = "2")]
    username: String,
}