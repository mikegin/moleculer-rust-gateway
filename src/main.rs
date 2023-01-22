use std::{error::Error, env, convert::Infallible};

use moleculer::{
    Error as MoleculerError,
    config::{ConfigBuilder, Transporter},
    service::{ActionBuilder, Service},
    ActionContext, ServiceBroker,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map, Number};
use tokio::{net::TcpListener, task};
use warp::{Filter, hyper::StatusCode, Rejection, Reply, reject, reply::{json, self}};

mod encryption;
mod temporal;
#[derive(Debug)]
struct MyError;

impl warp::reject::Reject for MyError {}

// #[tokio::main]
// async fn main() -> eyre::Result<()> {
//     env_logger::init();
//     color_eyre::install()?;

//     let config = ConfigBuilder::default()
//     .transporter(Transporter::nats("nats://localhost:4222"))
//     .namespace("buildable-services")
//     .build();

//     let math_action = ActionBuilder::new("mathAdd").add_callback(math_add).build();
//     let greeter_service = Service::new("rustMath").add_action(math_action);

//     let service_broker = ServiceBroker::new(config).add_service(greeter_service);

//     let local = task::LocalSet::new();
//     local.run_until(async move {
//         task::spawn_local(async move {
//             service_broker.start().await;
//         }).await.unwrap();
//     }).await;   

//     let hello = warp::path!("hello" / String)
//         .map(|name| format!("Hello, {}!", name));

//     warp::serve(hello)
//         .run(([127, 0, 0, 1], 3030))
//         .await;

//     Ok(())
// }
// fn math_add(ctx: ActionContext) -> Result<(), Box<dyn Error>> {
//     // get message decode using serde
//     let msg: ActionMessage = serde_json::from_value(ctx.params.clone())?;
//     let answer = msg.a + msg.b;

//     // serialize reply using serde and send
//     let _ = ctx.reply(answer.into());

//     Ok(())
// }

// #[derive(Deserialize)]
// struct ActionMessage {
//     a: i32,
//     b: i32,
// }

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    

    
    let config = ConfigBuilder::default()
    .transporter(Transporter::nats("nats://localhost:4222"))
    .namespace("buildable-services")
    .build();


    let math_action = ActionBuilder::new("mathAdd").add_callback(math_add).build();
    let greeter_service = Service::new("rustMath").add_action(math_action);

    let service_broker = ServiceBroker::new(config).add_service(greeter_service);

    let hello = warp::path("hello")
        .and(warp::get())
        .and(warp::query())
        .and(with_broker(service_broker.clone()))
        .and_then(list_todos_handler);
        // .map( |name| {
        //     struct N {
        //         a: i32,
        //         b: i32,
        //     }
        //     let mut m = serde_json::Map::new();
        //     m.insert("a".to_string(), Value::Number(Number::from(1)));
        //     m.insert("b".to_string(), Value::Number(Number::from(1)));

        //     service_broker.call("mathAdd", Value::Object(m)).await.unwrap();
        //     format!("Hello, {}!", name)
        // });

    

    let routes = hello
    // .or(todo_routes)
    .with(warp::cors().allow_any_origin())
    .recover(handle_rejection);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    service_broker.start().await;
}

fn math_add(ctx: ActionContext) -> Result<(), Box<dyn Error>> {
    // get message decode using serde
    let msg: ActionMessage = serde_json::from_value(ctx.params.clone())?;
    let answer = msg.a + msg.b;

    // serialize reply using serde and send
    let _ = ctx.reply(answer.into());

    Ok(())
}

#[derive(Deserialize)]
struct ActionMessage {
    a: i32,
    b: i32,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}


pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}

fn with_broker(broker: ServiceBroker) -> impl Filter<Extract = (ServiceBroker,), Error = Infallible> + Clone {
    warp::any().map(move || broker.clone())
}

pub async fn list_todos_handler(query: SearchQuery, broker: ServiceBroker) -> std::result::Result<impl Reply, Rejection> {
    let mut m = serde_json::Map::new();
    m.insert("a".to_string(), Value::Number(Number::from(1)));
    m.insert("b".to_string(), Value::Number(Number::from(1)));

    match broker.call("v2.external-events.list", Value::Object(m)).await {
        Ok(v) => {
            Ok(warp::reply::json(
                &v,
            ))
        },
        Err(e) => {
            println!("error: {:?}", e);
            Err(warp::reject())
        }
    }
}