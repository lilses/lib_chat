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
    wallet_id: i32,
    message: String,
    room: i32,
    wallet_id: i32
);
