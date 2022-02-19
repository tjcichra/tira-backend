use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::{
    models::{create::CreateCategory, Category},
    service, TiraDbConn,
};

pub async fn create_category(conn: TiraDbConn, category: CreateCategory) -> QueryResult<usize> {
    use crate::schema::categories::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(categories)
            .values((&category, created.eq(Utc::now().naive_utc())))
            .execute(c)
    })
    .await
}

pub async fn delete_categories(conn: TiraDbConn) -> QueryResult<usize> {
    use crate::schema::categories::dsl::*;

    conn.run(|c| diesel::delete(categories).execute(c)).await
}

pub async fn delete_category_by_id(conn: TiraDbConn, category_id: i64) -> QueryResult<()> {
    use crate::schema::categories::dsl::*;

    let result = conn
        .run(move |c| diesel::delete(categories.filter(id.eq(category_id))).execute(c))
        .await;

    service::check_only_one_row_changed(result)
}

pub async fn get_categories(conn: TiraDbConn) -> QueryResult<Vec<Category>> {
    use crate::schema::categories::dsl::*;

    conn.run(|c| categories.load::<Category>(c)).await
}

pub async fn get_category_by_id(conn: TiraDbConn, user_id: i64) -> QueryResult<Category> {
    use crate::schema::categories::dsl::*;

    conn.run(move |c| categories.filter(id.eq(user_id)).first::<Category>(c))
        .await
}
