use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;

use crate::{error::AppError, models::User};

struct EmailPassword {
    email: String,
    password_hash: String,
}

impl User {
    /// find a user by email
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as(
            "SELECT id, fullname, email, ws_id, create_at FROM users where email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    /// create a new User
    pub async fn create(
        email: &str,
        fullname: &str,
        password: &str,
        ws_id: i64,
        pool: &PgPool,
    ) -> Result<User, AppError> {
        let opt_user = Self::find_by_email(email, pool).await?;
        if opt_user.is_some() {
            return Err(AppError::EmailConflictError(format!(
                "this account {email} already exists."
            )));
        }
        let hash_password = hash_password(password)?;
        let user = sqlx::query_as(
            r#"
            INSERT INTO users (email, fullname, password_hash, ws_id) 
            VALUES ($1, $2, $3, $4) 
            RETURNING id, fullname, email, ws_id, create_at
            "#,
        )
        .bind(email)
        .bind(fullname)
        .bind(hash_password)
        .bind(ws_id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Verify an email account
    pub async fn verify_account_by_email(
        email: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Option<User>, AppError> {
        let user_opt: Option<User> = sqlx::query_as(
            "SELECT id, email, fullname, password_hash, ws_id, create_at from users where email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        match user_opt {
            Some(user) => {
                let password_hash = user.password_hash.as_deref().unwrap_or_default();
                let is_valid = verify_password(password, password_hash)?;
                if is_valid {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

/// Hash the password
fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

/// Verify the password
fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use sqlx_db_tester::TestPg;

    use super::*;

    #[tokio::test]
    async fn create_user_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://bobby@localhost/im_web".to_string(),
            std::path::Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;

        // do something with the pool
        let email = "bobby@gmail.com";
        let password = "W3c";
        let fullname = "Bobby Wang";
        let user = User::create(email, fullname, password, 0, &pool).await?;
        let ret = User::verify_account_by_email(email, password, &pool).await?;
        assert_eq!(ret.unwrap().email, user.email);
        assert_eq!(email, user.email);

        // when tdb gets dropped, the database will be dropped
        Ok(())
    }

    #[test]
    fn hash_password_should_work() -> Result<()> {
        let password = "W3c";
        let password_hash = hash_password(password)?;
        let is_valid = verify_password(password, &password_hash)?;
        assert_eq!(is_valid, true);
        Ok(())
    }
}
