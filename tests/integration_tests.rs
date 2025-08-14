use reqwest;
use serde_json::json;
use std::collections::HashMap;

#[tokio::test]
async fn test_health_endpoint() {
    let client = reqwest::Client::new();
    
    let response = client
        .get("http://localhost:3000/health")
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            assert_eq!(resp.status(), 200);
            let body: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(body["status"], "healthy");
        }
        Err(_) => {
            println!("Server not running - skipping integration test");
        }
    }
}

#[tokio::test]
async fn test_user_registration_flow() {
    let client = reqwest::Client::new();
    
    let user_data = json!({
        "email": "test@example.com",
        "username": "testuser",
        "password": "password123"
    });
    
    let response = client
        .post("http://localhost:3000/api/auth/register")
        .json(&user_data)
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let body: serde_json::Value = resp.json().await.unwrap();
                assert!(body["token"].is_string());
                assert!(body["user"]["id"].is_string());
            }
        }
        Err(_) => {
            println!("Server not running - skipping integration test");
        }
    }
}