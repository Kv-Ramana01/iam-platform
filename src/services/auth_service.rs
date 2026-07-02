use crate::{models::{organization::{self, CreateOrganizationRequest, NewOrganization}, role, session::Session, user::{LoginRequest, NewUser, RegisterRequest}}, repositories::{organization_repository, role_repository, user_repository}};
use crate::repositories::session_repository;
use crate::models::role::*;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{
        Error::Password, SaltString, rand_core::OsRng,
    },
};
use chrono::{Duration, Utc};
use uuid::Uuid;
use crate::auth::jwt;
use sqlx::PgPool;

use crate::{
    models::permission::{
        CreatePermissionRequest,
        NewPermission,
    },
    repositories::permission_repository,
};

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

pub async fn create_organization(
    pool: &PgPool,
    user_id: Uuid,
    request: CreateOrganizationRequest,
) {
    if request.name.trim().is_empty() {
        println!("Organization name cannot be empty");
        return;
    }

    let organization_id = Uuid::new_v4();

    let organization = NewOrganization{
        id: organization_id,
        name: request.name.trim().to_string(),
        description: request.description,
    };

    organization_repository::create_organization(pool, organization,).await.unwrap();


}


pub async fn create_role(
    pool: &PgPool,
    request: CreateRoleRequest,
) {
    if request.name.trim().is_empty() {
        println!("Role name cannot be empty");
        return;
    }

    let role_id = Uuid::new_v4();

    let role = NewRole {
        id: role_id,
        organization_id: request.organization_id,
        name: request.name.trim().to_string(),
        description: request.description,
    };

    role_repository::create_role(pool, role).await.unwrap();

}

pub async fn create_permission(
    pool: &PgPool,
    request: CreatePermissionRequest,
) {

    if request.name.trim().is_empty() {
        println!("Permission name cannot be empty");
        return;
    }

    let permission = NewPermission {
        id: Uuid::new_v4(),
        name: request.name.trim().to_string(),
        description: request.description,
    };

    permission_repository::create_permission(
        pool,
        permission,
    )
    .await
    .unwrap();

    println!("Permission created!");
}

use crate::models::role_permission::*;
use crate::repositories::role_permission_repository;

pub async fn assign_permission(
    pool: &PgPool,
    request: AssignPermissionRequest,
) {

    let role_permission = NewRolePermission {
        role_id: request.role_id,
        permission_id: request.permission_id,
    };

    role_permission_repository::assign_permission(
        pool,
        role_permission,
    )
    .await
    .unwrap();

    println!("Permission assigned!");
}

use crate::models::membership::*;
use crate::repositories::membership_repository;

pub async fn create_membership(
    pool: &PgPool,
    request: CreateMembershipRequest,
) {

    let membership = NewMembership {
        id: Uuid::new_v4(),
        user_id: request.user_id,
        organization_id: request.organization_id,
        role_id: request.role_id,
    };

    membership_repository::create_membership(
        pool,
        membership,
    )
    .await
    .unwrap();

    println!("Membership created!");
}

pub async fn has_permission(
    pool: &PgPool,
    user_id: Uuid,
    organization_id: Uuid,
    permission_name: &str,
) -> bool {

    membership_repository::has_permission(
        pool,
        user_id,
        organization_id,
        permission_name,
    )
    .await
    .unwrap()
}