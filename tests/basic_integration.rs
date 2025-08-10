use first_rust_app::handlers::health::health_check;

#[tokio::test]
async fn test_basic_integration_health_check() {
    let result = health_check().await;
    assert!(result.is_ok());
    
    let json_response = result.unwrap();
    let response_value = json_response.0;
    
    assert_eq!(response_value["status"], "ok");
    assert_eq!(response_value["service"], "first-rust-app");
    assert!(response_value["timestamp"].is_string());
}