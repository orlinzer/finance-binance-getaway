use actix_web::{get, post, /* web, */ App, HttpResponse, HttpServer, Responder};

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

// pub fn init(config: &mut web::ServiceConfig) {
  // config.service(
  //     web::scope("")
  //         .service(index)
  //         .service(healthcheck)
  // );
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .configure(init)
            .service(hello)
            .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
