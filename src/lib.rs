use macros_create_app::make_app68;
use macros_make_error::make_error2;
use macros_make_model::make_model22;
use macros_make_scope::make_scope;
use serde::*;

make_error2!(ChatError);

make_model22!(
    QChat,
    IChat,
    OChat,
    chat,
    message: String,
    room: i32,
    wallet_id: i32
);

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
struct IdPathParam {
    pub id: i32,
}

make_app68!(
    [message: String, room: i32, wallet_id: i32],
    chat_route,
    "/chat",
    "/chat/{id}",
    "",
    "{id}",
    OChat,
    QChat,
    IChat,
    ChatRequest,
    chat,
    [
        |s: actix_web::web::Data<my_state::MyState>,
         json: actix_web::web::Json<ChatRequest>,
         wallet: lib_wallet::QWallet,
         http_request: actix_web::HttpRequest| async move { handle(s, json, wallet).await }
    ],
    ChatError
);

make_scope!("chat", [post, chat_route]);

async fn handle(
    s: actix_web::web::Data<my_state::MyState>,
    json: actix_web::web::Json<ChatRequest>,
    _: lib_wallet::QWallet,
) -> Result<QChat, ChatError> {
    chat::postgres_query::insert(&s.sqlx_pool, &json.data)
        .await
        .map_err(ChatError::from_general)
}
