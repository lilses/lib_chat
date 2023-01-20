use macros_create_app::make_app80;
use macros_make_error::make_error2;
use macros_make_model::make_model22;
use macros_make_scope::make_scope;
use my_state::State;
use serde::*;

make_error2!(ChatError);

make_app80!(
    [
        message: String,
        room: i32,
        wallet_id: i32,
        created_at: chrono::DateTime<chrono::Utc>
    ],
    route,
    "/chat",
    "/chat/{id}",
    "",
    "{id}",
    chat,
    [
        |s: actix_web::web::Data<State>,
         json: actix_web::web::Json<route::IRequest>,
         wallet: lib_wallet::QWallet,
         http_request: actix_web::HttpRequest| async move { handle(s, json, wallet).await }
    ],
    ChatError
);

make_scope!("chat", [post, route]);

async fn handle(
    s: actix_web::web::Data<State>,
    json: actix_web::web::Json<route::IRequest>,
    _: lib_wallet::QWallet,
) -> Result<Q, ChatError> {
    chat::postgres_query::insert(&s.sqlx_pool, &json.data)
        .await
        .map_err(ChatError::from_general)
}
