use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use anyhow::Result;
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

pub struct UserService {
    pool: Pool<Postgres>,
}

impl UserService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)?;
        let now = Utc::now();
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO users (id, email, username, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(&id)
        .bind(&request.email)
        .bind(&request.username)
        .bind(&password_hash)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(User {
            id,
            email: request.email,
            username: request.username,
            password_hash,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let row = sqlx::query(
            "SELECT id, email, username, password_hash, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(&id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            email: r.get("email"),
            username: r.get("username"),
            password_hash: r.get("password_hash"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let row = sqlx::query(
            "SELECT id, email, username, password_hash, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            email: r.get("email"),
            username: r.get("username"),
            password_hash: r.get("password_hash"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    pub async fn list_users(&self, limit: i64, offset: i64) -> Result<Vec<User>> {
        let rows = sqlx::query(
            "SELECT id, email, username, password_hash, created_at, updated_at FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| User {
            id: r.get("id"),
            email: r.get("email"),
            username: r.get("username"),
            password_hash: r.get("password_hash"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn update_user(&self, id: Uuid, request: UpdateUserRequest) -> Result<Option<User>> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE users 
            SET email = COALESCE($2, email),
                username = COALESCE($3, username),
                updated_at = $4
            WHERE id = $1
            "#,
        )
        .bind(&id)
        .bind(&request.email)
        .bind(&request.username)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            self.get_user_by_id(id).await
        } else {
            Ok(None)
        }
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(&id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn verify_password(&self, email: &str, password: &str) -> Result<Option<User>> {
        if let Some(user) = self.get_user_by_email(email).await? {
            if bcrypt::verify(password, &user.password_hash)? {
                return Ok(Some(user));
            }
        }
        Ok(None)
    }
}