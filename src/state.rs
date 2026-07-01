//appstate which will be shared among multiple handlers
use sqlx::PgPool;

#[derive(Clone)] //derive clone because axum gives each request a copy of the state it just copies a smart pointer to the pool so its shared among all the participants, avoids expensive resource investment.
pub struct AppState{
    pub pool: PgPool,
}