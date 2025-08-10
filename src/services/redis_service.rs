use redis::{AsyncCommands, Client};

#[allow(dead_code)]
pub async fn set_session(
    redis_client: &Client,
    session_id: &str,
    data: &str,
    ttl_seconds: u64,
) -> anyhow::Result<()> {
    let mut conn = redis_client.get_multiplexed_async_connection().await?;
    conn.set_ex::<_, _, ()>(session_id, data, ttl_seconds).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn get_session(
    redis_client: &Client,
    session_id: &str,
) -> anyhow::Result<Option<String>> {
    let mut conn = redis_client.get_multiplexed_async_connection().await?;
    let result: Option<String> = conn.get(session_id).await?;
    Ok(result)
}

#[allow(dead_code)]
pub async fn delete_session(redis_client: &Client, session_id: &str) -> anyhow::Result<()> {
    let mut conn = redis_client.get_multiplexed_async_connection().await?;
    let _: () = conn.del(session_id).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn cache_set<T: serde::Serialize>(
    redis_client: &Client,
    key: &str,
    value: &T,
    ttl_seconds: u64,
) -> anyhow::Result<()> {
    let mut conn = redis_client.get_multiplexed_async_connection().await?;
    let serialized = serde_json::to_string(value)?;
    conn.set_ex::<_, _, ()>(key, serialized, ttl_seconds).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn cache_get<T: serde::de::DeserializeOwned>(
    redis_client: &Client,
    key: &str,
) -> anyhow::Result<Option<T>> {
    let mut conn = redis_client.get_multiplexed_async_connection().await?;
    let result: Option<String> = conn.get(key).await?;
    
    match result {
        Some(data) => {
            let deserialized = serde_json::from_str(&data)?;
            Ok(Some(deserialized))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    // Test data structures
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestData {
        id: String,
        value: i32,
        name: String,
    }

    impl TestData {
        fn new(id: String, value: i32, name: String) -> Self {
            Self { id, value, name }
        }
    }

    // Mock Redis client for testing
    // In real tests, you would use redis-test or similar
    fn create_mock_redis_client() -> Result<Client, redis::RedisError> {
        // This would fail in actual test environment without Redis
        // For demonstration, we'll show the test structure
        Client::open("redis://localhost:6379")
    }

    #[tokio::test]
    #[ignore = "requires Redis connection"]
    async fn test_session_operations() {
        let client = create_mock_redis_client().unwrap();
        let session_id = "test_session_123";
        let session_data = "user:456:active";
        let ttl = 3600;

        // Test setting session
        let set_result = set_session(&client, session_id, session_data, ttl).await;
        assert!(set_result.is_ok());

        // Test getting session
        let get_result = get_session(&client, session_id).await.unwrap();
        assert_eq!(get_result, Some(session_data.to_string()));

        // Test deleting session
        let delete_result = delete_session(&client, session_id).await;
        assert!(delete_result.is_ok());

        // Verify session is deleted
        let get_after_delete = get_session(&client, session_id).await.unwrap();
        assert_eq!(get_after_delete, None);
    }

    #[tokio::test]
    #[ignore = "requires Redis connection"]
    async fn test_cache_operations() {
        let client = create_mock_redis_client().unwrap();
        let key = "test_cache_key";
        let test_data = TestData::new("123".to_string(), 42, "Test Item".to_string());
        let ttl = 1800;

        // Test setting cache
        let set_result = cache_set(&client, key, &test_data, ttl).await;
        assert!(set_result.is_ok());

        // Test getting cache
        let get_result: Result<Option<TestData>, _> = cache_get(&client, key).await;
        assert!(get_result.is_ok());
        
        let retrieved_data = get_result.unwrap();
        assert_eq!(retrieved_data, Some(test_data));
    }

    // Unit tests for data serialization/deserialization (without Redis)
    #[test]
    fn test_test_data_serialization() {
        let test_data = TestData::new("uuid-123".to_string(), 100, "Sample".to_string());
        
        let serialized = serde_json::to_string(&test_data).unwrap();
        let deserialized: TestData = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(test_data, deserialized);
    }

    #[test]
    fn test_test_data_with_special_characters() {
        let test_data = TestData::new(
            "special-chars-{}[]".to_string(),
            -42,
            "Name with spaces and 🔒".to_string(),
        );
        
        let serialized = serde_json::to_string(&test_data).unwrap();
        let deserialized: TestData = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(test_data, deserialized);
    }

    #[test]
    fn test_empty_strings_serialization() {
        let test_data = TestData::new("".to_string(), 0, "".to_string());
        
        let serialized = serde_json::to_string(&test_data).unwrap();
        let deserialized: TestData = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(test_data, deserialized);
    }

    // Edge case tests
    #[tokio::test] 
    #[ignore = "requires Redis connection"]
    async fn test_get_nonexistent_session() {
        let client = create_mock_redis_client().unwrap();
        let nonexistent_session = "nonexistent_session_456";

        let result = get_session(&client, nonexistent_session).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    #[ignore = "requires Redis connection"] 
    async fn test_get_nonexistent_cache() {
        let client = create_mock_redis_client().unwrap();
        let nonexistent_key = "nonexistent_cache_key";

        let result: Result<Option<TestData>, _> = cache_get(&client, nonexistent_key).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[tokio::test]
    #[ignore = "requires Redis connection"]
    async fn test_session_expiry() {
        let client = create_mock_redis_client().unwrap();
        let session_id = "expiry_test_session";
        let session_data = "temporary_data";
        let ttl = 1; // 1 second

        // Set session with short TTL
        let set_result = set_session(&client, session_id, session_data, ttl).await;
        assert!(set_result.is_ok());

        // Should exist immediately
        let get_immediate = get_session(&client, session_id).await.unwrap();
        assert_eq!(get_immediate, Some(session_data.to_string()));

        // Wait for expiry
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Should be expired
        let get_after_expiry = get_session(&client, session_id).await.unwrap();
        assert_eq!(get_after_expiry, None);
    }

    #[tokio::test]
    #[ignore = "requires Redis connection"]
    async fn test_delete_nonexistent_session() {
        let client = create_mock_redis_client().unwrap();
        let nonexistent_session = "nonexistent_for_delete";

        // Deleting non-existent session should not error
        let delete_result = delete_session(&client, nonexistent_session).await;
        assert!(delete_result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires Redis connection"]
    async fn test_concurrent_session_operations() {
        let client = create_mock_redis_client().unwrap();
        
        // Create multiple concurrent operations
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let client = client.clone();
                tokio::spawn(async move {
                    let session_id = format!("concurrent_session_{}", i);
                    let session_data = format!("data_{}", i);
                    
                    // Set session
                    set_session(&client, &session_id, &session_data, 3600).await.unwrap();
                    
                    // Get session
                    let result = get_session(&client, &session_id).await.unwrap();
                    assert_eq!(result, Some(session_data.clone()));
                    
                    // Delete session
                    delete_session(&client, &session_id).await.unwrap();
                    
                    session_id
                })
            })
            .collect();

        let results = futures::future::join_all(handles).await;
        
        // All operations should succeed
        for handle_result in results {
            let session_id = handle_result.unwrap();
            assert!(session_id.starts_with("concurrent_session_"));
        }
    }

    #[tokio::test]
    #[ignore = "requires Redis connection"]
    async fn test_large_session_data() {
        let client = create_mock_redis_client().unwrap();
        let session_id = "large_data_session";
        let large_data = "x".repeat(10000); // 10KB of data
        let ttl = 3600;

        let set_result = set_session(&client, session_id, &large_data, ttl).await;
        assert!(set_result.is_ok());

        let get_result = get_session(&client, session_id).await.unwrap();
        assert_eq!(get_result, Some(large_data));

        delete_session(&client, session_id).await.unwrap();
    }

    // Test various data types with cache
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ComplexTestData {
        uuid: Uuid,
        numbers: Vec<i32>,
        nested: Option<Box<ComplexTestData>>,
    }

    #[tokio::test]
    #[ignore = "requires Redis connection"]
    async fn test_complex_cache_data() {
        let client = create_mock_redis_client().unwrap();
        let key = "complex_data";
        
        let complex_data = ComplexTestData {
            uuid: Uuid::new_v4(),
            numbers: vec![1, 2, 3, 4, 5],
            nested: Some(Box::new(ComplexTestData {
                uuid: Uuid::new_v4(),
                numbers: vec![10, 20],
                nested: None,
            })),
        };

        let set_result = cache_set(&client, key, &complex_data, 3600).await;
        assert!(set_result.is_ok());

        let get_result: Result<Option<ComplexTestData>, _> = cache_get(&client, key).await;
        assert!(get_result.is_ok());
        
        let retrieved = get_result.unwrap();
        assert_eq!(retrieved, Some(complex_data));
    }
}