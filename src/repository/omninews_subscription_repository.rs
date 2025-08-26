use sqlx::{query, query_as, MySqlPool};

use crate::{
    db_util::get_db,
    model::{omninews_subscription::NewOmniNewsSubscription, user::User},
};

pub async fn verify_subscription(
    pool: &sqlx::MySqlPool,
    user_email: &str,
) -> Result<User, sqlx::Error> {
    let result = query_as!(
        User,
        "SELECT * FROM user WHERE user_email = ? AND user_subscription_plan = 1",
        user_email
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

pub async fn register_subscription(
    pool: &MySqlPool,
    user_email: &str,
    subscription: NewOmniNewsSubscription,
) -> Result<bool, sqlx::Error> {
    let mut conn = get_db(pool).await?;

    let result = query!(
        "UPDATE user 
            SET user_subscription_receipt_data = ?, 
                user_subscription_product_id = ?, 
                user_subscription_platform = ?, 
                user_subscription_plan = ?, 
                user_subscription_auto_renew = ?, 
                user_subscription_is_test = ?,
                user_subscription_start_date = ?, 
                user_subscription_end_date = ?
            WHERE user_email = ?",
        subscription.user_subscription_receipt_data,
        subscription.user_subscription_product_id,
        subscription.user_subscription_platform,
        subscription.user_subscription_plan,
        subscription.user_subscription_auto_renew,
        subscription.user_subscription_is_test,
        subscription.user_subscription_start_date,
        subscription.user_subscription_end_date,
        user_email,
    )
    .execute(&mut *conn)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                Ok(true)
            } else {
                Err(sqlx::Error::RowNotFound)
            }
        }
        Err(e) => Err(e),
    }
}
