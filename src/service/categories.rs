use std::time::SystemTime;

use diesel::{delete, insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    models::{Category, CreateCategory},
    TiraDbConn,
};

pub async fn create_category(conn: TiraDbConn, category: CreateCategory) {
    use crate::schema::categories::dsl::*;

    conn.run(move |c| {
        insert_into(categories)
            .values((&category, created.eq(SystemTime::now())))
            .execute(c)
            .expect("Error with inserting category")
    })
    .await;
}

pub async fn get_categories(conn: TiraDbConn) -> Vec<Category> {
    use crate::schema::categories::dsl::*;

    conn.run(|c| {
        categories
            .load::<Category>(c)
            .expect("Error with retrieving categories")
    })
    .await
}

pub async fn get_category_by_id(conn: TiraDbConn, user_id: i32) -> Category {
    use crate::schema::categories::dsl::*;

    conn.run(move |c| {
        categories
            .filter(id.eq(user_id))
            .first::<Category>(c)
            .expect("Could not find any category.")
    })
    .await
}

pub async fn delete_categories(conn: TiraDbConn) {
    use crate::schema::categories::dsl::*;

    conn.run(|c| {
        delete(categories)
            .execute(c)
            .expect("Failed to delete categories table");
    })
    .await;
}

pub async fn delete_category_by_id(conn: TiraDbConn, category_id: i32) {
    use crate::schema::categories::dsl::*;

    conn.run(move |c| {
        delete(categories.filter(id.eq(category_id)))
            .execute(c)
            .expect("Failed to delete category by id")
    })
    .await;
}
