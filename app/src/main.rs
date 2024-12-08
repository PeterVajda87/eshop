use app::db_connection;
use app::CONFIG;
use entity::user::{self, Entity as User};
use migration::{Migrator, MigratorTrait};
use ntex::web::{self};
use sea_orm::EntityTrait;

#[web::get("/")]
async fn hello() -> impl web::Responder {
    let db = db_connection();
    let message = format!("{:?}", db);
    web::HttpResponse::Ok().body(message)
}

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl web::Responder {
    let user: Option<user::Model> = User::find().one(db_connection()).await.unwrap();
    web::HttpResponse::Ok().body(format!("{user:?}"))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    Migrator::refresh(db_connection()).await.unwrap();
    web::HttpServer::new(|| {
        web::App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((format!("{}", CONFIG.server.address), CONFIG.server.port))?
    .run()
    .await
}
