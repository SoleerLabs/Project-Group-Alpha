use anyhow::Result;
use httpc_test::new_client;
use serde_json::json;
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = new_client("http://localhost:8080")?;
    hc.do_get("/hello2/ola")
        .await?
        .print()
        .await?;

        hc.do_get("/nest").await?.print().await?;

        let req_login = hc.do_post("/api/login", json!({"username":"admin","password":"password"}));
        req_login.await?.print().await?;

    Ok(())

    
}
