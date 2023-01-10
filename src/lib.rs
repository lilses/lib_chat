use macros_create_app::make_app65;
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

#[derive(
    utoipa::ToSchema, Debug, PartialEq, serde::Deserialize, serde::Serialize, Clone, Default,
)]
pub struct ChatRequest {
    pub data: IChat,
    pub wallet_request: lib_wallet::WalletAuthId,
}

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
struct IdPathParam {
    pub id: i32,
}

make_app65!(
    [message: String, room: i32, wallet_id: i32],
    chat_route,
    "/chat",
    "/chat/{id}",
    "",
    "{id}",
    OChat,
    QChat,
    chat,
    [
        ChatRequest,
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
    tokio::spawn({
        let s = s.clone();
        let json = json.clone();
        async move {
            println!("hello {:?}", json.data);
            s.req
                .post(&format!("{}/queue/notification", s.env.cwa_2_api))
                .json::<IChat>(&json.data)
                .send()
                .await
                .map_err(ChatError::from_general)
                .map_err(|x| {
                    tracing::error!("{:?}", x);
                    x
                })?
                .error_for_status()
                .map_err(ChatError::from_general)
                .map_err(|x| {
                    tracing::error!("{:?}", x);
                    x
                })?;
            Ok::<(), ChatError>(())
        }
    });
    chat::postgres_query::insert(&s.sqlx_pool, &json.data)
        .await
        .map_err(ChatError::from_general)
}
