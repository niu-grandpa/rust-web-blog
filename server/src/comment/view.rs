use std::sync::Arc;

use ntex::web::types::{Json, Path, State};

use crate::{
    errors::CustomError,
    models::{comment::Comment, user::UserInfo},
    AppState,
};

pub async fn get_comments_for_article(
    article_id: Path<(u32,)>,
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<Comment>>, CustomError> {
    let db_pool = &state.db_pool;
    let article_id = article_id.0;

    let comments = sqlx::query!(
        "SELECT \
            c.id, c.user_id, c.content, c.date, u.name, u.avatar_url \
         FROM \
            comments c \
         JOIN users u ON c.user_id = u.id \
         WHERE \
            c.article = $1",
        article_id as i32
    )
    .fetch_all(db_pool)
    .await?
    .iter()
    .map(|i| Comment {
        id: Some(i.id as u32),
        user: Some(UserInfo {
            id: i.user_id as u32,
            login: i.name.clone(),
            avatar_url: i.avatar_url.clone(),
            is_admin: i.user_id == 90502461,
        }),
        content: i.content.clone(),
        date: Some(i.date),
        article: None,
    })
    .collect::<Vec<Comment>>();

    Ok(Json(comments))
}
