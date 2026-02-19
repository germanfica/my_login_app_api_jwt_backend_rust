// use actix_web::{App, HttpServer};
//use actix_identity::{IdentityService, CookieIdentityPolicy};
use crate::config::Config;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_web::middleware::Logger;
use diesel::{ExpressionMethods, SelectableHelper};
use env_logger::Env;

use actix_web::{dev::Service as _};
use futures_util::future::FutureExt;


// use crate::auth::{login, authenticate_jwt};

use actix_web::{cookie::Key, get, post, web, App, HttpResponse, HttpServer, Responder};
use login_orm::{add, establish_connection, get_user_roles, models::Post, schema::posts::{self, published}};
// use login_orm::{add, establish_connection, models::Post};

use self::models::*;
use diesel::prelude::*;


// mod auth;
mod config;
mod constants;
mod middlewares; // Agrega esta línea al inicio de main.rs
mod utils;
mod models;
// mod schema;

// mod services;

#[get("/")]
async fn hello() -> impl Responder {
    let config = Config::from_env();

    let result = add(2, 2);

    let posts = &posts_table_example(); // & is more efficient than moving the entire data

    //let formatted_posts: Vec<String> = posts.iter().map(|post| format!("Title: {}, Body: {}", post.title, post.body)).collect();
    let posts_json = serde_json::to_string(&posts).expect("Failed to convert posts to JSON");

    let response_body = format!(
        "Hello world! JWT: {}; DB_HOST: {}; DB_PORT: {}, DB_USERNAME: {}; DB_PASSWORD: {}; DB_NAME: {}; ADD LIB EXAMPLE: {}; POSTS: {:?};",
        config.jwt_secret,
        config.db_host,
        config.db_port,
        config.db_username,
        config.db_password,
        config.db_name,
        result,
        posts_json
    );

    HttpResponse::Ok().body(response_body)
}

#[get("/hello_world")]
async fn hello_world() -> impl Responder {
    let config = Config::from_env();

    let response_body = format!(
        "Hello world! JWT: {}; DB_HOST: {}; DB_PORT: {}, DB_USERNAME: {}; DB_PASSWORD: {}; DB_NAME: {}",
        config.jwt_secret,
        config.db_host,
        config.db_port,
        config.db_username,
        config.db_password,
        config.db_name
    );

    roles_example();

    HttpResponse::Ok().body(response_body)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn posts_table_example() -> Vec<Post> {
    // use self::login_orm::schema::posts::dsl::*;
    // use self::schema::posts::dsl::*;

    //use crate::posts::dsl::posts;
    
    use login_orm::schema::posts::dsl::*;

    // let example = posts;

    let connection = &mut establish_connection();
    let results: Vec<Post> = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in &results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

    results
}

fn roles_example() {
    let connection = &mut establish_connection();

    //let mut connection = establish_connection();  // La conexión ahora es mutable


    match get_user_roles(connection, 1) {     // Pasar la conexión mutable
        Ok(roles) => {
            println!("Roles for user: {:?}", roles);
        }
        Err(err) => {
            println!("Error fetching user roles: {}", err);
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    // let secret_key: Key = Key::from(config.jwt_secret.as_bytes()); // Usa la clave JWT como clave secreta para SessionMiddleware
    use actix_web::{App, HttpServer};
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allow_any_header()
            .allow_any_method()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(config.clone())) // Use app_data here
            .wrap(cors)
            .wrap(IdentityMiddleware::default()) // Use IdentityMiddleware
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(crate::middlewares::auth_middleware::SayHi)
            // .wrap_fn(|req, srv| {
            //     println!("Hi from start. You requested: {}", req.path());
            //     srv.call(req).map(|res| {
            //         println!("Hi from response");
            //         res
            //     })
            // })
            // .wrap(crate::middlewares::auth_middleware::Authentication)
            .service(hello)
            .service(echo)
            .service(hello_world)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
