use axum::{
    routing::get,
    response::Html,
    extract::Multipart,
    Router
};

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../src/public/index.html"))
}

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart
        .next_field().await.expect("Failed to get next field!")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }
        println!("Got file!");
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index).post(upload));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.expect("Failed to start listener!");
    
    axum::serve(listener, app)
        .await.expect("Failed to serve 'app'!");
}

