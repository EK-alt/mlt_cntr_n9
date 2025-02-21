use std::process;

use actix_web::{
    body::BoxBody,
    http::header::ContentType,
    web::{get, scope, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use chrono::Utc;
use mysql::{prelude::Queryable, Pool};
use serde::Serialize;

use std::env;

use gel_tokio::{self, Client};
use std::fs;

// ------------------------------------------------------
#[derive(Serialize)]
struct DateTime {
    now: String,
    api: String,
}

impl Responder for DateTime {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
// ------------------------------------------------------

// #[tokio::main]/* anyhow::Result<()> */
async fn get_date_time_from_edge_db() -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let cert_path = current_dir.join("server-cert.pem");
    // Convert the PathBuf to a String.  Handle the error if needed.
    let cert_path_str = cert_path
        .to_str()
        .expect("Failed to convert path to string");

    //By Service name: edgedb_rust
    env::set_var("EDGEDB_HOST", "edgedb_rust");
    // env::set_var("EDGEDB_PORT", "5657");
    env::set_var("EDGEDB_SERVER_PORT", "5657");
    env::set_var("EDGEDB_USER", "edgedb");
    env::set_var("EDGEDB_PASSWORD", "edgedb");
    env::set_var("EDGEDB_DATABASE", "edgedb");
    env::set_var("EDGEDB_TLS_CA_FILE", cert_path_str);

    let tls_ca_file = env::var("EDGEDB_TLS_CA_FILE").unwrap();
    println!("EDGEDB_TLS_CA_FILE: {}", tls_ca_file);
    println!("EDGEDB_TLS_CA_FILE: {}", cert_path_str);

    let pem_content = fs::read_to_string(cert_path).expect("Failed to read file");
    println!("pem_content {}", pem_content);

    let conn = gel_tokio::create_client()
        .await
        .expect("Client should have initiated");

    // let conn = gel_tokio::create_client(&gel_tokio::Options {
    //     host: "localhost".to_string(), // The Docker host IP (NOT localhost)
    //     port: 5657,                    // The port mapped on the host machine
    //     user: "edgedb",
    //     password: "edgedb",
    //     database: "edgedb",
    //     ..Default::default() // Important: Use Default::default() to avoid setting tls
    // })
    // .await
    // .expect("Client should have initiated");
    let val = conn
        .query_required_single::<i64, _>("SELECT 7*8", &())
        .await
        .expect("Failed to get 7*8");

    println!("7*8 is: {}", val);

    // let timestamp = get_current_timestamp(&conn).await?;
    let result: String = conn
        .query_required_single("SELECT <str>datetime_of_statement()", &())
        .await
        .expect("Failed to get current timestamp");

    println!("Current UTC timestamp: {}", result);
    // println!("EdgeDB Timestamp: {}", timestamp);
    // Ok(())
    // return Ok(result);
    return result;
}

// async fn get_current_timestamp(conn: &gel_tokio::Client) -> Result<String, Box<dyn std::error::Error>> {
//     let result: String = conn
//         .query_required_single("SELECT <str>$<datetime::datetime>now", &()) // EdgeDB query
//         .await?;

//     Ok(result)
// }

async fn getDateTime() -> impl Responder {
    let now_utc = Utc::now();
    println!("Current time in UTC: {}", now_utc.to_string());

    // if let Err(e) = get_date_time_from_edge_db().await {
    //     eprintln!("Error: {:?}", e);
    // }

    // let join_handle = tokio::spawn(async {
    //     if let Err(e) = get_date_time_from_edge_db().await {
    //         eprintln!("Error in task: {:?}", e);
    //     }
    // });
    // let join_handle = tokio::spawn(async {
    //    let tm = get_date_time_from_edge_db().await;
    // });

    let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    // let DATABASE_URL = "postgres://postgres:foobarbaz@db:5432/postgres".to_string();
    // let DATABASE_URL = "mysql://homestead:secret@127.0.0.1:33071/homestead".to_string();
    println!("Database URL: {}", DATABASE_URL);

    let pool = Pool::new(DATABASE_URL.as_str()).expect("Failed to create a pool");
    let mut conn = pool.get_conn().expect("Failed to get connection.");
    let result: Vec<String> = conn
        // .query("SELECT CURRENT_TIMESTAMP() AS now")
        .query("SELECT DATE_FORMAT(UTC_TIMESTAMP(6), '%Y-%m-%dT%H:%i:%s.%fZ') AS now")
        .expect("Query failed");
    println!("aRes {:?} ", result);
    let result = result[0].clone();

    let dateTime = DateTime {
        // now: now_utc.to_string(),
        now: result.to_string(),
        api: "actix-web".to_string(),
    };

    return dateTime;
}

pub fn get_time_factory(app: &mut ServiceConfig) {
    // println!("get_time_factory()-call");
    // getDateTime();
    app.service(scope("").route("/", get().to(getDateTime)));
}
