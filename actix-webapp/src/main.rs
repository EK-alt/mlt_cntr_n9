// use mysql::{prelude::Queryable, Pool};
// use std::env;
// fn main() {
//     println!("Hello, world from Rust!");
//     // let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
//     // let pool = Pool::new(DATABASE_URL.as_str()).expect("Failed to create a pool");
//     // let mut conn = pool.get_conn().expect("Failed to get connection.");
//     // let result: Vec<String> = conn
//     //     // .query("SELECT CURRENT_TIMESTAMP() AS now")
//     //     .query("SELECT DATE_FORMAT(UTC_TIMESTAMP(6), '%Y-%m-%dT%H:%i:%s.%fZ') AS now")
//     //     .expect("Query failed");

//     // println!("aRes {:?} ", result);
// }

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use actix_cors::Cors;
use gel_tokio::Builder;
use gel_tokio::Client;
use gel_tokio::Config;
use mysql::{prelude::Queryable, Pool};
use serde::Serialize;
use std::env;
use std::fs;

async fn get_date_time_from_edge_db() -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let cert_path = current_dir.join("server-cert.pem");
    // Convert the PathBuf to a String.  Handle the error if needed.
    let cert_path_str = cert_path
        .to_str()
        .expect("Failed to convert path to string");

    // //By Service name: edgedb_rust
    // env::set_var("EDGEDB_HOST", "edgedb_rust");
    // // env::set_var("EDGEDB_PORT", "5657");
    // env::set_var("EDGEDB_SERVER_PORT", "5657");
    // env::set_var("EDGEDB_USER", "edgedb");
    // env::set_var("EDGEDB_PASSWORD", "edgedb");
    // env::set_var("EDGEDB_DATABASE", "edgedb");
    // // env::set_var("EDGEDB_SERVER_SECURITY", "insecure_dev_mode");
    // env::set_var("EDGEDB_TLS_CA_FILE", cert_path_str);

    // let tls_ca_file = env::var("EDGEDB_TLS_CA_FILE").unwrap();
    // println!("EDGEDB_TLS_CA_FILE: {}", tls_ca_file);
    // println!("EDGEDB_TLS_CA_FILE: {}", cert_path_str);

    // let pem_content = fs::read_to_string(cert_path).expect("Failed to read file");
    // println!("pem_content {}", pem_content);

    // let conn = gel_tokio::create_client()
    //     .await
    //     .expect("Client should have initiated");

    // // let conn = gel_tokio::create_client(&gel_tokio::Options {
    // //     host: "localhost".to_string(), // The Docker host IP (NOT localhost)
    // //     port: 5657,                    // The port mapped on the host machine
    // //     user: "edgedb",
    // //     password: "edgedb",
    // //     database: "edgedb",
    // //     ..Default::default() // Important: Use Default::default() to avoid setting tls
    // // })
    // // .await
    // // .expect("Client should have initiated");
    // let val = conn
    //     .query_required_single::<i64, _>("SELECT 7*8", &())
    //     .await
    //     .expect("Failed to get 7*8");

    // println!("7*8 is: {}", val);
    // Ensure environment variables are loaded (if not using dotenv or similar tools)
    let host = env::var("EDGEDB_HOST").unwrap_or_else(|_| "edgedb_rust".to_string());
    let port = env::var("EDGEDB_PORT").unwrap_or_else(|_| "5657".to_string());
    let user = env::var("EDGEDB_USER").unwrap_or_else(|_| "edgedb".to_string());
    let password = env::var("EDGEDB_PASSWORD").unwrap_or_else(|_| "edgedb".to_string());
    let database = env::var("EDGEDB_DATABASE").unwrap_or_else(|_| "edgedb".to_string());
    // let instance = env::var("EDGEDB_INSTANCE").unwrap_or_else(|_| "edgedb_inst_n2".to_string());

    // Print values for debugging
    // println!("Connecting to EdgeDB instance: {}", instance);
    println!("Using host: {}, port: {}", host, port);
    println!("Database: {}, User: {}", database, user);

    // Create a configuration object
    // let config = Config::default()
    //     .host(&host)
    //     .port(port.parse().unwrap_or(5656))
    //     .user(&user)
    //     .password(&password)
    //     .database(&database)
    //     .tls_security("insecure");

    let cert_path/* : &Path */ = std::path::Path::new("server-cert.pem");
    // Create a new EdgeDB client
    // let config = gel_tokio::Builder::new()
    //     .host("edgedb_rust")
    //     .unwrap()
    //     .port(5657)
    //     .unwrap()
    //     .user("edgedb")
    //     .unwrap()
    //     .password("edgedb")
    //     .database("edgedb")
    //     .unwrap()
    //     // .tls_security(gel_tokio::TlsSecurity::Insecure)
    //     // .tls_ca_file(cert_path)
    //     .build_env()
    //     .await
    //     .unwrap();

    // let client = Client::new(&config)
    //     .ensure_connected()
    //     .await
    //     .expect("Client should have initiated");

    let edgedb_url = "edgedb://edgedb:edgedb@edgedb_rust:5656/edgedb?tls_security=insecure";
    let config = Builder::new()
        .dsn(&edgedb_url)
        .unwrap()
        .build_env()
        .await
        .unwrap();
    let client: Client = Client::new(&config);

    match client.ensure_connected().await {
        Ok(_) => println!("Successfully connected to EdgeDB!"),
        Err(e) => eprintln!("Failed to connect to EdgeDB: {:?}", e),
    }

    let result: String = client
        .query_required_single("SELECT <str>datetime_of_statement()", &())
        .await
        .expect("Failed to get current timestamp");

    println!("Current UTC timestamp: {}", result);

    // "Hello form EdgeDB".to_string();
    result
}

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

async fn hello() -> impl Responder {
    let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let pool = Pool::new(DATABASE_URL.as_str()).expect("Failed to create a pool");
    let mut conn = pool.get_conn().expect("Failed to get connection.");
    let result: Vec<String> = conn
        // .query("SELECT CURRENT_TIMESTAMP() AS now")
        .query("SELECT DATE_FORMAT(UTC_TIMESTAMP(6), '%Y-%m-%dT%H:%i:%s.%fZ') AS now")
        .expect("Query failed");

    println!("aRes {:?} ", result);

    // let tm_lcl;
    let join_handle = tokio::spawn(async {
        let tm = get_date_time_from_edge_db().await;
        tm
    })
    .await
    .expect("Tokio spawn failed");

    println!("tm_lcl: {}", join_handle);

    let result = result[0].clone();
    // format!(
    //     "Hello, world! and response from MySQL {} && from EdgeDB {}",
    //     result, join_handle
    // );

    let date_time = DateTime {
        // now: now_utc.to_string(),
        now: format!("{} && {}", result, join_handle),
        api: "actix-web: from mysql & edgedb".to_string(),
    };

    return date_time;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world from Rust!");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .route("/hello", web::get().to(hello)) // Route for /hello
            .route("/actix-webapp", web::get().to(hello)) // Route for / (root path) - optional
            .wrap(cors)
    })
    .bind(("0.0.0.0", 9093))? // Bind to all interfaces (0.0.0.0) and port 9090
    .run()
    .await
}
