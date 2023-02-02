use std::fs::File;
use std::io::Write;
use actix_form_data::{Field, Form};
use actix_web::{web, get, App, HttpServer, Responder, HttpRequest, post};
use actix_web::web::{BufMut, BytesMut};
use anyhow::anyhow;
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::json;
use futures::StreamExt;

use ai_address_backend::error::MyError;


#[get("/hello")]
async fn greet() -> impl Responder {
    "Hello world"
}

// regex matcher: r"^.*(?=((海边)|(沙滩)|(阳光))?).*$"
#[get("/list")]
async fn list() -> impl Responder {
    lazy_static! { static ref LIST: serde_json::Value = json! {[
            { "value": "海边沙滩", "tags": ["海边", "沙滩", "阳光"] },
            { "value": "纯色风格", "tags": ["纯色", "静谧", "灵动", "压抑"] },
            { "value": "古典建筑", "tags": ["古典", "建筑", "都市", "乡村"] },
            { "value": "繁华都市", "tags": ["都市", "现代", "繁华"] },
            { "value": "夜晚路边", "tags": ["马路", "路灯", "夜晚"] },
            { "value": "傍晚夕阳", "tags": ["傍晚", "夕阳", "山川", "天际线"] },
            { "value": "富士山下", "tags": ["山川", "白色", "山川", "静谧"] },
            { "value": "码头", "tags": ["船舶", "海边", "阳光"] },
            { "value": "机场", "tags": ["飞机", "现代", "都市"] },
            { "value": "火车站", "tags": ["火车", "现代", "都市"] },
            { "value": "汽车站", "tags": ["汽车", "现代", "都市"] },
            { "value": "日出海边", "tags": ["海边", "日出"] },
            { "value": "日落海边", "tags": ["海边", "日落"] },
            { "value": "日出山川", "tags": ["山川", "日出"] },
            { "value": "日落山川", "tags": ["山川", "日落"] },
        ]};
    }
    web::Json(LIST.clone())
}

#[get("/background")]
async fn background(req: HttpRequest) -> impl Responder {
    #[derive(Serialize, Clone, Debug)]
    struct Data {
        url: &'static str,
        tags: Vec<&'static str>,
    }

    impl From<(&'static str, Vec<&'static str>)> for Data {
        fn from(value: (&'static str, Vec<&'static str>)) -> Self {
            Self { url: value.0, tags: value.1 }
        }
    }
    lazy_static! { static ref DATA_LIST: Vec<Data> = Vec::from([
            ("1.jpg", vec!["海边", "山川"]).into(),
            ("2.jpg", vec!["海边", "礁石", "蓝天"]).into(),
            ("3.jpg", vec!["蓝天", "街道", "现代"]).into(),
            ("4.jpg", vec!["日落", "大海", "夕阳"]).into(),
            ("5.jpg", vec!["蓝天", "白云", "马路"]).into(),
            ("6.jpg", vec!["蓝天", "现代", "车"]).into(),
            ("7.jpg", vec!["石头", "山川"]).into(),
            ("8.jpg", vec!["富士山", "花朵", "纯色"]).into(),
            ("9.jpg", vec!["船舶", "阳光", "海边"]).into(),
        ]);
    }

    let keywords = req.query_string();
    if keywords.is_empty() {
        return Ok(web::Json(DATA_LIST.clone()));
    }
    // omit "keyword=" prefix and decode urlencoding
    let keywords = urlencoding::decode(&keywords[8..]);
    let keywords = if let Err(e) = keywords {
        return Err(MyError::from(anyhow!(e)));
    } else {
        keywords.ok().unwrap()
    };
    let keywords: Vec<&str> = keywords.split('-').collect();
    let ret = DATA_LIST.clone().into_iter()
        .filter(|data| {
            data.tags.iter().fold(false, |acc, word| keywords.contains(word) || acc)
        })
        .collect::<Vec<Data>>();
    Ok(web::Json(ret))
}

#[post("/upload", wrap = "construct_file_form()")]
async fn upload() -> impl Responder {
    "Uploaded"
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ip = "127.0.0.1";
    let port = 8080;
    println!("Server running at http://{ip}:{port}"); // TODO: secure link
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(greet)
                    .service(list)
                    .service(background)
                    .service(upload)
            )
    })
        .bind((ip, port))?
        .run()
        .await?;
    Ok(())
}

// TODO: Change file name
fn construct_file_form() -> Form<(), MyError> {
    let file_form: Form<(), MyError> = Form::new()
        // person image
        .field("personImage", Field::file(|filename, _, mut stream| async move {
            let mut buf = BytesMut::new();
            while let Some(res) = stream.next().await {
                buf.put(res?.as_ref());
            }
            let buf = buf.freeze();
            File::create(format!("./test/files/person/{}", filename))?.write_all(&buf[..])?;
            Ok(())
        }))
        // clothes image
        .field("clothesImage", Field::file(|filename, _, mut stream| async move {
            let mut buf = BytesMut::new();
            while let Some(res) = stream.next().await {
                buf.put(res?.as_ref());
            }
            let buf = buf.freeze();
            File::create(format!("./test/files/clothes/{}", filename))?.write_all(&buf[..])?;
            Ok(())
        }))
        // background image
        .field("backgroundImage", Field::file(|filename, _, mut stream| async move {
            let mut buf = BytesMut::new();
            while let Some(res) = stream.next().await {
                buf.put(res?.as_ref());
            }
            let buf = buf.freeze();
            File::create(format!("./test/files/background/{}", filename))?.write_all(&buf[..])?;
            Ok(())
        }));
    file_form
}