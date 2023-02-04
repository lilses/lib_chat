use macros_create_app::{make_app103, make_app104, make_app80};
use macros_make_error::{make_error2, make_error20};
use macros_make_model::make_model22;
use macros_make_scope::make_scope;
use my_state::State;
use serde::*;

make_error20!(ChatError, [actix]);

make_app104!(
    i32,
    [
        message: String,
        room: i32,
        wallet_id: i32,
        created_at: chrono::DateTime<chrono::Utc>
    ],
    "/chat",
    "/chat/{id}",
    "",
    "{id}",
    chat,
    [
        |s: actix_web::web::Data<State>,
         json: actix_web::web::Json<route::IRequest>,
         wallet: lib_wallet::Q,
         http_request: actix_web::HttpRequest| async move { handle(s, json, wallet).await }
    ],
    [post, get, id]
);

async fn handle(
    s: actix_web::web::Data<State>,
    json: actix_web::web::Json<route::IRequest>,
    _: lib_wallet::Q,
) -> Result<Q, Error> {
    chat::postgres_query::insert(&s.sqlx_pool, &json.data)
        .await
        .map_err(Error::from_general)
}
