//! Operator `auth_user` helpers. Self-hosted = one staff operator.

use anyhow::Result;
use chrono::Utc;
use sqlx::{Row, SqlitePool};

use crate::models::User;

const USER_COLUMNS: &str = "id, username, email, password, first_name, last_name, \
     is_staff, is_active, is_superuser, last_login, date_joined";

impl User {
    pub async fn get(pool: &SqlitePool, id: i64) -> Result<Option<User>> {
        let sql = format!("SELECT {USER_COLUMNS} FROM auth_user WHERE id = ?1");
        Ok(sqlx::query_as::<_, User>(&sql).bind(id).fetch_optional(pool).await?)
    }

    pub async fn by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>> {
        let sql = format!("SELECT {USER_COLUMNS} FROM auth_user WHERE username = ?1");
        Ok(sqlx::query_as::<_, User>(&sql).bind(username).fetch_optional(pool).await?)
    }

    pub async fn any_staff_active(pool: &SqlitePool) -> Result<bool> {
        let row = sqlx::query("SELECT 1 FROM auth_user WHERE is_staff = 1 AND is_active = 1 LIMIT 1")
            .fetch_optional(pool)
            .await?;
        Ok(row.is_some())
    }

    /// Create (or update) the staff operator. (`account` onboarding step)
    pub async fn upsert_operator(
        pool: &SqlitePool,
        username: &str,
        email: &str,
        first_name: &str,
        is_superuser: bool,
    ) -> Result<User> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO auth_user (username, email, first_name, is_staff, is_active, is_superuser, date_joined)
             VALUES (?1, ?2, ?3, 1, 1, ?4, ?5)
             ON CONFLICT(username) DO UPDATE SET email=?2, first_name=?3, is_staff=1, is_active=1,
                is_superuser=MAX(is_superuser, ?4)",
        )
        .bind(username)
        .bind(email)
        .bind(first_name)
        .bind(is_superuser as i64)
        .bind(&now)
        .execute(pool)
        .await?;
        Ok(User::by_username(pool, username).await?.expect("just upserted"))
    }

    pub async fn set_password_hash(pool: &SqlitePool, id: i64, hash: &str) -> Result<()> {
        sqlx::query("UPDATE auth_user SET password = ?2 WHERE id = ?1")
            .bind(id)
            .bind(hash)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// True when the active staff operator has a non-blank email. (`account.is_done`)
    pub async fn operator_has_email(pool: &SqlitePool) -> Result<bool> {
        let row = sqlx::query(
            "SELECT 1 FROM auth_user WHERE is_staff = 1 AND is_active = 1 AND email != '' LIMIT 1",
        )
        .fetch_optional(pool)
        .await?;
        Ok(row.is_some())
    }
}

/// Link every campaign to the operator (so `session.campaigns` finds them).
pub async fn link_all_campaigns(pool: &SqlitePool, user_id: i64) -> Result<()> {
    let ids = sqlx::query("SELECT id FROM core_campaign")
        .fetch_all(pool)
        .await?;
    for r in ids {
        let cid: i64 = r.get("id");
        crate::models::Campaign::add_user(pool, cid, user_id).await?;
    }
    Ok(())
}
