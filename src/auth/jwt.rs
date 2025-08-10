use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::models::Claims;

pub fn generate_jwt_token(
    user_id: Uuid,
    email: String,
    role: String,
    secret: &str,
    expires_in: u64,
) -> anyhow::Result<String> {
    let now = Utc::now();
    let expire = now + Duration::seconds(expires_in as i64);

    let claims = Claims {
        sub: user_id,
        email,
        role,
        iat: now.timestamp() as usize,
        exp: expire.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_jwt_token(token: &str, secret: &str) -> anyhow::Result<Claims> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(claims.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use claims::{assert_err, assert_ok};
    use uuid::Uuid;

    const TEST_SECRET: &str = "test-secret-key";
    const INVALID_SECRET: &str = "invalid-secret";

    fn create_test_user_data() -> (Uuid, String, String) {
        (
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "user".to_string(),
        )
    }

    #[test]
    fn test_generate_jwt_token_success() {
        let (user_id, email, role) = create_test_user_data();
        let expires_in = 3600u64;

        let result = generate_jwt_token(user_id, email.clone(), role.clone(), TEST_SECRET, expires_in);
        
        let token = result.unwrap();
        
        // Token should not be empty
        assert!(!token.is_empty());
        
        // Should be able to verify the token we just created
        let claims = verify_jwt_token(&token, TEST_SECRET).unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
    }

    #[test]
    fn test_verify_jwt_token_success() {
        let (user_id, email, role) = create_test_user_data();
        let expires_in = 3600u64;
        
        let token = generate_jwt_token(user_id, email.clone(), role.clone(), TEST_SECRET, expires_in).unwrap();
        let result = verify_jwt_token(&token, TEST_SECRET);
        
        let claims = result.unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
        
        // Check that timestamps are reasonable
        let now = Utc::now().timestamp() as usize;
        assert!(claims.iat <= now);
        assert!(claims.exp > now);
    }

    #[test]
    fn test_verify_jwt_token_invalid_secret() {
        let (user_id, email, role) = create_test_user_data();
        let expires_in = 3600u64;
        
        let token = generate_jwt_token(user_id, email, role, TEST_SECRET, expires_in).unwrap();
        let result = verify_jwt_token(&token, INVALID_SECRET);
        
        assert_err!(result);
    }

    #[test]
    fn test_verify_jwt_token_malformed() {
        let result = verify_jwt_token("invalid-token", TEST_SECRET);
        assert_err!(result);
    }

    #[test]
    fn test_verify_jwt_token_empty() {
        let result = verify_jwt_token("", TEST_SECRET);
        assert_err!(result);
    }

    #[test]
    fn test_generate_jwt_token_zero_expiration() {
        let (user_id, email, role) = create_test_user_data();
        let expires_in = 0u64;

        let result = generate_jwt_token(user_id, email.clone(), role.clone(), TEST_SECRET, expires_in);
        let token = result.unwrap();
        let claims = verify_jwt_token(&token, TEST_SECRET).unwrap();
        
        // Token should be immediately expired
        let now = Utc::now().timestamp() as usize;
        assert!(claims.exp <= now);
    }

    #[test]
    fn test_generate_jwt_token_very_long_expiration() {
        let (user_id, email, role) = create_test_user_data();
        let expires_in = u64::MAX;

        // This might fail due to overflow, which is expected behavior
        let result = generate_jwt_token(user_id, email, role, TEST_SECRET, expires_in);
        
        // If it succeeds, the token should be valid
        if let Ok(token) = result {
            let claims = verify_jwt_token(&token, TEST_SECRET);
            assert_ok!(claims);
        }
    }

    #[test]
    fn test_generate_jwt_token_empty_secret() {
        let (user_id, email, role) = create_test_user_data();
        let expires_in = 3600u64;

        let result = generate_jwt_token(user_id, email, role, "", expires_in);
        assert!(result.is_ok());
        
        // Should be verifiable with empty secret
        let token = result.unwrap();
        let verify_result = verify_jwt_token(&token, "");
        assert_ok!(verify_result);
    }

    #[test]
    fn test_jwt_claims_fields() {
        let user_id = Uuid::new_v4();
        let email = "special+chars@test-domain.co.uk".to_string();
        let role = "admin".to_string();
        let expires_in = 7200u64;
        
        let token = generate_jwt_token(user_id, email.clone(), role.clone(), TEST_SECRET, expires_in).unwrap();
        let claims = verify_jwt_token(&token, TEST_SECRET).unwrap();
        
        // Verify all fields are correctly preserved
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
        
        // Verify time fields
        let now = Utc::now().timestamp() as usize;
        assert!(claims.iat <= now);
        assert!(claims.exp > claims.iat);
        assert!(claims.exp <= now + expires_in as usize);
    }

    #[test]
    fn test_different_user_ids_produce_different_tokens() {
        let user_id_1 = Uuid::new_v4();
        let user_id_2 = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let role = "user".to_string();
        let expires_in = 3600u64;
        
        let token_1 = generate_jwt_token(user_id_1, email.clone(), role.clone(), TEST_SECRET, expires_in).unwrap();
        let token_2 = generate_jwt_token(user_id_2, email, role, TEST_SECRET, expires_in).unwrap();
        
        assert_ne!(token_1, token_2);
        
        let claims_1 = verify_jwt_token(&token_1, TEST_SECRET).unwrap();
        let claims_2 = verify_jwt_token(&token_2, TEST_SECRET).unwrap();
        
        assert_eq!(claims_1.sub, user_id_1);
        assert_eq!(claims_2.sub, user_id_2);
    }

    #[test]
    fn test_unicode_email_and_role() {
        let user_id = Uuid::new_v4();
        let email = "тест@пример.рф".to_string(); // Cyrillic characters
        let role = "管理员".to_string(); // Chinese characters
        let expires_in = 3600u64;
        
        let result = generate_jwt_token(user_id, email.clone(), role.clone(), TEST_SECRET, expires_in);
        let token = result.unwrap();
        let claims = verify_jwt_token(&token, TEST_SECRET).unwrap();
        
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
    }
}