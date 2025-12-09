use crate::middleware::auth::Auth;
use crate::utils::jwt::create_jwt;
use actix_web::HttpMessage;
use actix_web::{HttpResponse, web};
use bcrypt::{hash, verify};
use db::DbPool;
use db::models::{NewUser, User};
use db::schema::users::dsl::*;
use diesel::prelude::*;
use tracing::{error, info, warn};
use crate::types::auth_types::{LoginForm, RegisterForm};
use crate::types::response_types::{ResponseMessage};


// POST /api/v1/auth/register
async fn register(pool: web::Data<DbPool>, req: web::Json<RegisterForm>) -> HttpResponse {
    info!("Register attempt for username: {}", req.username);
    info!("Register attempt for email: {}", req.email);
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB connection error"),
    };

    let hashed = hash(&req.password, 12).unwrap();

    let new_user = NewUser {
        email: req.email.clone(),
        username: req.username.clone(),
        password_hash: hashed,
    };

    let inserted: Result<User, _> = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&mut conn);

    match inserted {
        Ok(user) => {
            info!("✅ User registered: {}", user.username);
            HttpResponse::Ok().json(ResponseMessage {
                message: "Registration successful".to_string(),
                status_code: 200,
                token: None,
                user_id: Some(user.id),
            })
        }
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => {
            warn!("User registration failed: {}", req.username);
            HttpResponse::Conflict().body("User already exists")
        }
        Err(_) => {
            error!("User registration failed: {}", req.username);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// POST /api/v1/auth/login
async fn login(pool: web::Data<DbPool>, req: web::Json<LoginForm>) -> HttpResponse {
    info!("Login attempt for username: {}", req.email);
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB connection error"),
    };

    let user = users
        .filter(email.eq(&req.email))
        .first::<User>(&mut conn);

    match user {
        Ok(u) => {
            if verify(&req.password, &u.password_hash).unwrap_or(false) {
                info!("✅ User logged in: {}", u.username);
                let token = create_jwt(u.id).unwrap_or_default();
                HttpResponse::Ok().json(ResponseMessage{
                    user_id: Some(u.id),
                    token: Some(token),
                    message: "Login successful".to_string(),
                    status_code: 200,
                })
            } else {
                warn!("User login failed: {}", req.email);
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

// GET /api/v1/auth/me
async fn me(req: actix_web::HttpRequest) -> HttpResponse {
    info!("Me request received");

    if let Some(claims) = req.extensions().get::<crate::utils::jwt::Claims>() {
        HttpResponse::Ok().json(serde_json::json!({
            "user_id": claims.sub
        }))
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .service(web::scope("").wrap(Auth).route("/me", web::get().to(me))),
    );
}
