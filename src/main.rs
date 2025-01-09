use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use std::io::{self};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {

    // URL to request
    println!("Enter the discussion URL::");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read input");
    let url = url.trim();

    // For host header
    // let parsed_url = Url::parse(url).expect("Invalid URL");
    // let host = parsed_url.host_str().expect("Failed to extract host from URL");


    // Ask for cookie values from the user
    println!("Enter the value for d2lSessionVal:");
    let mut d2l_session_val = String::new();
    io::stdin().read_line(&mut d2l_session_val).expect("Failed to read input");
    let d2l_session_val = d2l_session_val.trim();

    println!("Enter the value for d2lSecureSessionVal:");
    let mut d2l_secure_session_val = String::new();
    io::stdin().read_line(&mut d2l_secure_session_val).expect("Failed to read input");
    let d2l_secure_session_val = d2l_secure_session_val.trim();

    println!("Enter the delay (in milliseconds):");
    let mut delay = String::new();
    io::stdin().read_line(&mut delay).expect("Failed to read input");

    // Convert the input to an integer
    let delay: u64 = delay.trim().parse().unwrap_or_else(|_| {
        eprintln!("Invalid input. Using default delay of 400ms.");
        400 // Default delay value if parsing fails
    });

    println!("Enter the requests:");
    let mut request_count = String::new();
    io::stdin().read_line(&mut request_count).expect("Failed to read input");

    // Convert the input to an integer
    let request_count: u64 = request_count.trim().parse().unwrap_or_else(|_| {
        eprintln!("Invalid input. default to 1000");
        1000 // Default delay value if parsing fails
    });


    // Create HTTP client
    let client = Client::new();

    // Create custom headers
    let mut headers = HeaderMap::new();

    // You can dynamically add host since if you remove in burp it will show 400 but i think this rust req lib adds one automatically
    //headers.insert("Host", HeaderValue::from_str(host).expect("Invalid host"));
    headers.insert(
        "User-Agent",
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.6778.140 Safari/537.36"),
    );
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
    headers.insert(
        "Accept",
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
        ),
    );
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
    headers.insert("Sec-Fetch-User", HeaderValue::from_static("?1"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
    headers.insert(
        "Cookie",
        HeaderValue::from_str(&format!(
            "d2lSessionVal={}; d2lSecureSessionVal={}",
            d2l_session_val, d2l_secure_session_val
        ))
            .expect("Invalid cookie values"),
    );
    headers.insert("Sec-Ch-Ua", HeaderValue::from_static("\"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\""));
    headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
    headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("\"Windows\""));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
    headers.insert("Priority", HeaderValue::from_static("u=0, i"));


    for i in 0..request_count {
        let response = client.get(url).headers(headers.clone()).send().await;

        match response {
            Ok(res) => {
                let status = res.status();
                match res.bytes().await {
                    Ok(body) => {
                        println!("Request {}: Status: {}", i + 1, status);
                        //println!("Response Body: {:?}", body);
                    }
                    Err(err) => {
                        eprintln!("Request {}: Error reading body: {}", i + 1, err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Request {}: Error: {}", i + 1, err);
            }
        }

        // Wait before sending the next request
        sleep(Duration::from_millis(delay)).await;
    }
}
