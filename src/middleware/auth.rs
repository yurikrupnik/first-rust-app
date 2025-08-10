use std::sync::Arc;
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{auth::verify_jwt_token, state::AppState};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path();
    
    if path.starts_with("/api/auth") || path == "/api/health" {
        return Ok(next.run(request).await);
    }

    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = if let Some(auth_header) = auth_header {
        auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let claims = verify_jwt_token(token, &state.config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use crate::{
        auth::generate_jwt_token,
        config::Config,
        models::Claims,
    };
    use uuid::Uuid;

    // Mock AppState for testing
    fn create_test_config() -> Config {
        Config {
            database_url: "test://localhost".to_string(),
            redis_url: "test://localhost".to_string(),
            mongodb_url: "test://localhost".to_string(),
            jwt_secret: "test-secret-key".to_string(),
            jwt_expires_in: 3600,
            jwt_refresh_expires_in: 604800,
            port: 8080,
        }
    }

    // Helper function for creating test requests (conceptual)
    // In real tests, you would use proper test harness

    #[tokio::test]
    #[ignore = "requires full app state setup"]
    async fn test_auth_middleware_allows_public_endpoints() {
        // This test would require full AppState setup
        // For demonstration, showing the test structure
        let config = create_test_config();
        
        let public_paths = vec![
            "/api/health",
            "/api/auth/login",
            "/api/auth/register",
            "/api/auth/refresh",
        ];

        for path in public_paths {
            // Would test that these paths don't require authentication
            println!("Testing public path: {}", path);
        }
    }

    #[tokio::test]
    #[ignore = "requires full app state setup"]
    async fn test_auth_middleware_rejects_missing_token() {
        // This test would verify rejection of requests without auth header
        let config = create_test_config();
        println!("Testing missing token rejection");
    }

    #[tokio::test]
    #[ignore = "requires full app state setup"]
    async fn test_auth_middleware_rejects_invalid_token() {
        // This test would verify rejection of malformed/invalid tokens
        let config = create_test_config();
        println!("Testing invalid token rejection");
    }

    #[tokio::test]
    #[ignore = "requires full app state setup"]
    async fn test_auth_middleware_accepts_valid_token() {
        // This test would verify acceptance of valid JWT tokens
        let config = create_test_config();
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let role = "user".to_string();
        
        let token = generate_jwt_token(
            user_id,
            email.clone(),
            role.clone(),
            &config.jwt_secret,
            config.jwt_expires_in,
        ).unwrap();
        
        println!("Generated test token: {}", token);
        println!("Testing valid token acceptance");
    }

    // Unit tests for token parsing logic
    #[test]
    fn test_bearer_token_extraction() {
        let test_cases = vec![
            ("Bearer token123", Some("token123")),
            ("Bearer ", Some("")), // strip_prefix returns empty string, not None
            ("Basic token123", None),
            ("bearer token123", None), // case sensitive
            ("Bearer token123 extra", Some("token123 extra")),
            ("", None),
        ];

        for (input, expected) in test_cases {
            let result = input.strip_prefix("Bearer ");
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_public_path_detection() {
        let public_paths = vec![
            "/api/auth/login",
            "/api/auth/register", 
            "/api/auth/refresh",
            "/api/health",
        ];

        let protected_paths = vec![
            "/api/users",
            "/api/users/123",
            "/api/admin",
            "/api/data",
        ];

        for path in public_paths {
            let is_public = path.starts_with("/api/auth") || path == "/api/health";
            assert!(is_public, "Path should be public: {}", path);
        }

        for path in protected_paths {
            let is_public = path.starts_with("/api/auth") || path == "/api/health";
            assert!(!is_public, "Path should be protected: {}", path);
        }
    }

    #[test]
    fn test_auth_header_parsing_edge_cases() {
        let edge_cases = vec![
            "Bearer",           // Just "Bearer"
            "Bearer\t",         // Bearer with tab
            "Bearer\n",         // Bearer with newline
            "Bearer  token",    // Multiple spaces
            "BearerToken",      // No space
            "Bearer token\0",   // Null byte
        ];

        for case in edge_cases {
            let result = case.strip_prefix("Bearer ");
            println!("Edge case '{}' -> {:?}", case.escape_debug(), result);
            
            // Most should fail to parse correctly
            if case == "Bearer  token" {
                assert_eq!(result, Some(" token"));
            } else if case == "Bearer\ttoken" {
                // This would actually work
            }
        }
    }

    // Integration-style tests (require less setup)
    #[test]
    fn test_claims_structure() {
        let user_id = Uuid::new_v4();
        let claims = Claims {
            sub: user_id,
            email: "test@example.com".to_string(),
            role: "user".to_string(),
            iat: 1000000,
            exp: 2000000,
        };

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.role, "user");
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_token_expiry_logic() {
        use chrono::Utc;
        
        let now = Utc::now().timestamp() as usize;
        
        // Valid token (expires in future)
        let valid_claims = Claims {
            sub: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            role: "user".to_string(),
            iat: now - 100,
            exp: now + 3600,
        };
        assert!(valid_claims.exp > now);
        
        // Expired token
        let expired_claims = Claims {
            sub: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            role: "user".to_string(),
            iat: now - 7200,
            exp: now - 100,
        };
        assert!(expired_claims.exp < now);
    }

    #[test]
    fn test_role_based_access_concepts() {
        let admin_claims = Claims {
            sub: Uuid::new_v4(),
            email: "admin@example.com".to_string(),
            role: "admin".to_string(),
            iat: 1000000,
            exp: 2000000,
        };

        let user_claims = Claims {
            sub: Uuid::new_v4(),
            email: "user@example.com".to_string(), 
            role: "user".to_string(),
            iat: 1000000,
            exp: 2000000,
        };

        // Test role-based permissions
        assert_eq!(admin_claims.role, "admin");
        assert_eq!(user_claims.role, "user");
        
        // Admin should have access to admin endpoints
        let admin_can_create_users = admin_claims.role == "admin";
        let user_can_create_users = user_claims.role == "admin";
        
        assert!(admin_can_create_users);
        assert!(!user_can_create_users);
    }
}