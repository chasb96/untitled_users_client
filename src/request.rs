use prost::Message;

#[derive(Message)]
pub struct ProjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: String,
}

#[derive(Message)]
pub struct CreateUserRequest {
    #[prost(string, tag = "1")]
    pub username: String,
}

#[derive(Message)]
pub struct UpdateUserRequest {
    #[prost(string, tag = "1")]
    pub username: String,
    #[prost(optional, string, tag = "2")]
    pub profile_picture: Option<String>,
}