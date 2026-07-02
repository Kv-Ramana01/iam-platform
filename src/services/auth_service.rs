use crate::{models::{session::Session, user::{LoginRequest, NewUser, RegisterRequest}}, repositories::user_repository};
use crate::repositories::session_repository;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{
        Error::Password, SaltString, rand_core::OsRng,
    },
};
use chrono::{Duration, Utc};
use uuid::Uuid;
use crate::auth::jwt;
use sqlx::PgPool;

pub async fn register_user(
    pool: &PgPool,
    request: RegisterRequest,
) {
     //algorithm
    //email normalisation, pasword hashing, uuid generation, newuser creation, calling repo for the db commmunication , return success

    if request.name.trim().is_empty() {
        println!("Name cannot be empty");
        return;
    }

    if request.email.trim().is_empty() {
        println!("Email cannot be empty");
        return;
    }

    if request.password.trim().is_empty() {
        println!("Password cannot be empty");
        return;
    }

    //normalizing the email format here
    let email = request.email.to_lowercase();

    //salt generation
    let salt = SaltString::generate(&mut OsRng);

    //password hashing
    let password_hash = Argon2::default().hash_password(
        request.password.as_bytes(),
        &salt,
    ).unwrap().to_string();

    //new user
    let new_user = NewUser {
        id: Uuid::new_v4(),
        name: request.name.trim().to_string(),
        email,
        password_hash,
    };

    //contacting the repository
    user_repository::create_user(pool, new_user).await.unwrap();

    println!("User inserted!");
   
}

pub async fn login_user(
    pool: &PgPool,
    request: LoginRequest,
) {
    if request.email.trim().is_empty() {
        println!("Email cannot be empty");
        return;
    }

    if request.password.trim().is_empty() {
        println!("Password cannot be empty");
        return;
    }

    let information = user_repository::find_user_by_email(pool, &request.email).await.unwrap();

    match information {
        Some(user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();

            let valid = Argon2::default()
            .verify_password(request.password.as_bytes(), &parsed_hash).is_ok();

            if !valid {
                println!("Invalid credentials");
                return;
            }
            let session = Session {
                id: Uuid::new_v4(),
                user_id:user.id,
                created_at: Utc::now(),
                expires_at: Utc::now() + Duration::hours(24),
                is_active: true,
            };

            session_repository::create_session(pool, &session).await.unwrap();

            let token = jwt::generate_token(
                user.id.to_string(),
                session.id.to_string(),
            );

            println!("{}",token);
                    
        },
        None => println!("User does not exist!"),
    }
}