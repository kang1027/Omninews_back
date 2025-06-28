use rocket::{http::Status, request::Request, request::FromRequest, serde::json::Json, State};
use sqlx::MySqlPool;

use crate::{
    model::{
        error::OmniNewsError,
        token::{JwtToken, TokenVerificationResponse, TokenInfo, UserInfo, TokenErrorResponse},
        user::{ParamUser, UserEmail},
    },
    service::user_service,
};

pub struct BearerToken(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let authorization = req.headers().get_one("Authorization");
        match authorization {
            Some(auth) => {
                if auth.starts_with("Bearer ") {
                    let token = auth[7..].to_string();
                    rocket::request::Outcome::Success(BearerToken(token))
                } else {
                    rocket::request::Outcome::Failure((Status::BadRequest, "Invalid authorization format"))
                }
            }
            None => rocket::request::Outcome::Failure((Status::Unauthorized, "Missing authorization header")),
        }
    }
}

#[post("/user", data = "<user_data>")]
pub async fn create_user(
    pool: &State<MySqlPool>,
    user_data: Json<ParamUser>,
) -> Result<Json<JwtToken>, Status> {
    match user_service::create_user(pool, user_data.into_inner()).await {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/user/logout", data = "<user_email>")]
pub async fn logout(
    pool: &State<MySqlPool>,
    user_email: Json<UserEmail>,
) -> Result<Status, Status> {
    match user_service::delete_user_token(
        pool,
        user_email.into_inner().user_email.unwrap_or_default(),
    )
    .await
    {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/user/verify-token")]
pub async fn verify_token(
    pool: &State<MySqlPool>,
    bearer_token: BearerToken,
) -> Result<Json<TokenVerificationResponse>, (Status, Json<TokenErrorResponse>)> {
    match user_service::verify_token(pool, bearer_token.0).await {
        Ok((user, jwt_token)) => {
            let response = TokenVerificationResponse {
                user: UserInfo {
                    email: user.user_email,
                    display_name: user.user_display_name,
                    photo_url: user.user_photo_url,
                },
                token: TokenInfo {
                    access_token: jwt_token.access_token,
                    expires_at: jwt_token.access_token_expires_at,
                },
            };
            Ok(Json(response))
        }
        Err(e) => {
            let error_message = match e {
                OmniNewsError::TokenExpired => "Token expired",
                OmniNewsError::InvalidToken => "Invalid token",
                _ => "Invalid token",
            };
            Err((Status::Unauthorized, Json(TokenErrorResponse {
                error: error_message.to_string(),
            })))
        }
    }
}
