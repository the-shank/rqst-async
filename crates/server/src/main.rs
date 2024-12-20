use miniserve::{Content, Request, Response};
use serde::{Deserialize, Serialize};

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(req: Request) -> Response {
    dbg!(&req);

    match &req {
        Request::Get => unreachable!(),
        Request::Post(s) => {
            let mut data: PostData = serde_json::from_str(s).expect("parsing request failed");
            data.messages
                .push("And how does that make you feel?".to_string());
            let data_json = serde_json::to_string(&data).unwrap();
            Ok(Content::Json(data_json))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostData {
    messages: Vec<String>,
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}