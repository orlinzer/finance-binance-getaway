use actix_web::{
    App, HttpResponse, HttpServer, Responder, cookie::time::error::Format, get, post, web,
};
use reqwest::{Response, Url};
use std::time::Duration;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

// #[get("/vuln/{vulnerability_id}")]
// pub async fn get_vulnerability(
//     pool: web::Data<DbPool>,
//     vuln_id: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {
//     let vuln_id = vuln_id.into_inner();
//     let conn = pool.get().expect("couldn't get db connection");

//     // Use web::block to offload blocking Diesel code without blocking server thread.
//     // Omitting find_vulnerability_by_id() function that queries the database here, it's trivial.
//     let vulnerability = web::block(move || find_vulnerability_by_id(vuln_id, &conn))
//         .await
//         .map_err(|e| {
//             eprintln!("{}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;

//     if let Some(vulnerability) = vulnerability {
//         Ok(HttpResponse::Ok().json(vulnerability))
//     } else {
//         let res = HttpResponse::NotFound().body(format!("No vuln found with id: {}", vuln_id));
//         Ok(res)
//     }
// }

// pub async fn fetch_data(symbol: &str) -> Result<CompanyQuote, Box<dyn std::error::Error>> {
pub async fn fetch_data(symbol: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Fetching data...");

    let url = format!(
        "https://data-api.binance.vision/api/v3/exchangeInfo?symbol={}",
        symbol
    );
    let url2 = format!(
        "https://data-api.binance.vision/api/v3/trades?symbol={}",
        symbol
    );

    let url = Url::parse(&*url)?;
    let url2 = Url::parse(&*url2)?;
    // let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;
    let res = reqwest::get(url).await?.text().await?;
    let res2 = reqwest::get(url2).await?.text().await?;

    // println!("Data fetched! {:?}", res2);

    let result = format!("{:?}, {:?}", res, res2);

    Ok(result)
}

pub fn init(config: &mut web::ServiceConfig) {
    config
        // .service(web::scope("")
        // .service(index)
        // .service(healthcheck));
        .service(hello)
        .service(echo);
    // .route("/hey", web::get().to(manual_hello))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(1000));
        loop {
            interval.tick().await; // This should go first.

            let res = fetch_data("BTCUSDT").await;
            match res {
                Ok(data) => {
                    // println!("Data: {:?}", data);
                    println!("Data: {:?}", data);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            // tokio::spawn(fetch_data());
        }
    });

    HttpServer::new(|| {
        App::new().configure(init)
        // .service(hello)
        // .service(echo)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
