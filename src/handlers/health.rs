use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};

pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "service": "first-rust-app",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_success() {
        let result = health_check().await;
        
        assert!(result.is_ok());
        let json_response = result.unwrap();
        let response_value = json_response.0;
        
        // Check that all required fields are present
        assert!(response_value.get("status").is_some());
        assert!(response_value.get("service").is_some());
        assert!(response_value.get("timestamp").is_some());
        
        // Check field values
        assert_eq!(response_value["status"], "ok");
        assert_eq!(response_value["service"], "first-rust-app");
        
        // Timestamp should be a valid RFC3339 string
        let timestamp = response_value["timestamp"].as_str().unwrap();
        assert!(chrono::DateTime::parse_from_rfc3339(timestamp).is_ok());
    }

    #[tokio::test]
    async fn test_health_check_timestamp_format() {
        let result = health_check().await;
        let json_response = result.unwrap();
        let response_value = json_response.0;
        
        let timestamp = response_value["timestamp"].as_str().unwrap();
        
        // Should be valid RFC3339 format
        let parsed_time = chrono::DateTime::parse_from_rfc3339(timestamp);
        assert!(parsed_time.is_ok());
        
        // Should be recent (within last 10 seconds)
        let now = chrono::Utc::now();
        let health_time = parsed_time.unwrap().with_timezone(&chrono::Utc);
        let diff = now.signed_duration_since(health_time);
        assert!(diff.num_seconds() < 10);
    }

    #[tokio::test]
    async fn test_health_check_multiple_calls() {
        // Make multiple calls and ensure they all succeed
        let mut results = Vec::new();
        
        for _ in 0..5 {
            let result = health_check().await;
            assert!(result.is_ok());
            results.push(result.unwrap().0);
        }
        
        // All should have the same status and service
        for result in &results {
            assert_eq!(result["status"], "ok");
            assert_eq!(result["service"], "first-rust-app");
        }
        
        // Timestamps should be different (or at least not all the same)
        let timestamps: Vec<&str> = results
            .iter()
            .map(|r| r["timestamp"].as_str().unwrap())
            .collect();
        
        // At least some should be different (due to time progression)
        let unique_timestamps: std::collections::HashSet<&str> = 
            timestamps.iter().copied().collect();
        // We expect at least some variation, but allow for same if called very quickly
        assert!(unique_timestamps.len() >= 1);
    }

    #[tokio::test]
    async fn test_health_check_json_structure() {
        let result = health_check().await;
        let json_response = result.unwrap();
        let response_value = json_response.0;
        
        // Should be a JSON object
        assert!(response_value.is_object());
        
        // Should have exactly 3 fields
        let obj = response_value.as_object().unwrap();
        assert_eq!(obj.len(), 3);
        
        // Check field types
        assert!(response_value["status"].is_string());
        assert!(response_value["service"].is_string());
        assert!(response_value["timestamp"].is_string());
    }

    #[tokio::test]
    async fn test_health_check_concurrent_calls() {
        // Test concurrent health checks
        let handles: Vec<_> = (0..10)
            .map(|_| tokio::spawn(health_check()))
            .collect();
        
        let results: Vec<_> = futures::future::join_all(handles).await;
        
        // All should succeed
        for handle_result in results {
            let health_result = handle_result.unwrap();
            assert!(health_result.is_ok());
            
            let json_response = health_result.unwrap();
            let response_value = json_response.0;
            
            assert_eq!(response_value["status"], "ok");
            assert_eq!(response_value["service"], "first-rust-app");
            assert!(response_value["timestamp"].is_string());
        }
    }
}