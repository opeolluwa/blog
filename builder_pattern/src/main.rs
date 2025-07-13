use axum::Router;

use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::post;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub struct ApiResponse<T: Serialize> {
    message: String,
    data: Option<T>,
    status_code: StatusCode,
}

#[derive(Debug)]
pub struct ApiResponseBuilder<T: Serialize> {
    status_code: StatusCode,
    message: Option<String>,
    data: Option<T>,
}

// default values of the APiBuilder pattern
impl<T> Default for ApiResponseBuilder<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            status_code: StatusCode::OK,
            message: None,
            data: None,
        }
    }
}

// the builder pattern
impl<T> ApiResponseBuilder<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> ApiResponse<T> {
        ApiResponse {
            message: self.message.unwrap_or_default(),
            data: self.data,
            status_code: self.status_code,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("an internal database error has occurred")]
    DatabaseError,
    #[error("badly formed request")]
    BadRequest,
    #[error("an internal error occurred")]
    OperationFailed,
}

impl ServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::OperationFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = Json(json!({
          "message":self.message,
          "data":self.data
        }));
        (self.status_code, body).into_response()
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/signup", post(sign_up));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct UserProfileRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserProfileResponse {
    first_name: String,
    last_name: String,
    avatar: String,
}

async fn sign_up(
    Json(request): Json<UserProfileRequest>,
) -> Result<ApiResponse<UserProfileResponse>, ServiceError> {
    let UserProfileRequest { email, password } = request;
    if email.is_empty() || password.is_empty() {
        return Err(ServiceError::BadRequest);
    }

    let user_profile = UserProfileResponse {
        first_name: "adeoye".to_string(),
        last_name: "adefemi".to_string(),
        avatar: "https://example.com/example.jpg".to_string(),
    };

    let response = ApiResponseBuilder::new()
        .data(user_profile)
        .status_code(StatusCode::CREATED)
        .build();

    Ok(response)
}
