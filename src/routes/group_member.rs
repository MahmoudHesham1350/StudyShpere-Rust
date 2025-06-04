// src/routes/group_member.rs
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;

use crate::handlers::group_member::{
    list_group_members_handler,
    create_group_member_handler,
    get_group_member_detail_handler,
    update_group_member_handler,
    delete_group_member_handler,
    get_self_group_membership_handler,
    leave_group_membership_handler,
};

pub fn group_member_routes() -> Router<PgPool> {
    Router::new()
        .route("/groups/{group_id}/members", get(list_group_members_handler))
        .route("/groups/{group_id}/members/create", post(create_group_member_handler))
        .route("/groups/{group_id}/members/self", get(get_self_group_membership_handler).delete(leave_group_membership_handler))
        .route("/groups/{group_id}/members/{user_id}", get(get_group_member_detail_handler).put(update_group_member_handler).delete(delete_group_member_handler))
}