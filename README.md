## Rust Web Requester

Made a rust program to spam views on a specific d2l discussion post :)

#### Installation

1. Clone the repository:
   `git clone https://github.com/0xC-Dev/Discussion-View-Botter`

2. Build the project:
   `cd Discussion-View-Botter`
   `cargo build --release`

3. Run the application:
   ```cargo run```

#### Usage
1. Enter the URL for the requests.
2. Provide session cookie values (d2lSessionVal and d2lSecureSessionVal).
3. Set a delay (in milliseconds) between requests.
4. Specify the number of requests to send.

The program will make the requests, print the status of each, and wait the specified delay before sending the next one.


##### Access d2lSessionVal and d2lSecureSessionVal 
1. Login to d2l account
2. Open Inspect
3. Go to Storage (Firefox) / Application (Chrome), Then cookies
4. Copy values and paste into program
