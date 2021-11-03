use bytes::Bytes;
use clokwerk::{Scheduler, TimeUnits};
use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use route_recognizer::Params;
use router::Router;

use std::fs;

use std::fs::OpenOptions;
use std::io::prelude::*;

use std::sync::Arc;

use std::time::Duration;
// Import week days and WeekDay

mod handler;
mod router;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub state_thing: String,
}

#[tokio::main]
async fn main() {
    let some_state = "state".to_string();

    let mut router: Router = Router::new();
    router.get("/test", Box::new(handler::test_handler));
    router.post("/send", Box::new(handler::send_handler));
    router.get("/params/:some_param", Box::new(handler::param_handler));
    let mut scheduler = Scheduler::new();
    scheduler.every(24.hours()).run(|| seek_and_changefl_all());
    /*let thread_handle = */
    scheduler.watch_thread(Duration::from_millis(100));

    let shared_router = Arc::new(router);
    let new_service = make_service_fn(move |_| {
        let app_state = AppState {
            state_thing: some_state.clone(),
        };

        let router_capture = shared_router.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                route(router_capture.clone(), req, app_state.clone())
            }))
        }
    });

    let addr = "0.0.0.0:8000".parse().expect("address creation works");
    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);

    let _ = server.await;
}

async fn route(
    router: Arc<Router>,
    req: Request<hyper::Body>,
    app_state: AppState,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler
        .handler
        .invoke(Context::new(app_state, req, found_handler.params))
        .await;
    Ok(resp)
}

#[derive(Debug)]
pub struct Context {
    pub state: AppState,
    pub req: Request<Body>,
    pub params: Params,
    body_bytes: Option<Bytes>,
}

impl Context {
    pub fn new(state: AppState, req: Request<Body>, params: Params) -> Context {
        Context {
            state,
            req,
            params,
            body_bytes: None,
        }
    }

    pub async fn body_json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Error> {
        let body_bytes = match self.body_bytes {
            Some(ref v) => v,
            _ => {
                let body = to_bytes(self.req.body_mut()).await?;
                self.body_bytes = Some(body);
                self.body_bytes.as_ref().expect("body_bytes was set above")
            }
        };
        Ok(serde_json::from_slice(&body_bytes)?)
    }
}

fn seek_and_changefl_all() {
    let contents = fs::read_to_string("foo.txt").expect("err");
    let mut new: String = String::new();
    let mut buff: String = String::new();

    for line in contents.lines() {
        // println!(" {} ",line);
        /* if line.contains(searched_item){*/
        let mut days_remaining: String = line.chars().skip(line.len() - 2).take(2).collect();
        let mut my_int;
        let mut substring: String;
        if !days_remaining.contains(">") {
            my_int = days_remaining.parse::<i32>().unwrap();
            substring = line.chars().take(line.len() - 2).collect();
        } else {
            days_remaining = line.chars().skip(line.len() - 1).take(1).collect();
            //println!(" {} ",days_remaining);
            my_int = days_remaining.parse::<i32>().unwrap();
            substring = line.chars().take(line.len() - 1).collect();
        }
        my_int -= 1;
        println!("this is the shit {} ", days_remaining);
        substring.push_str(&my_int.to_string());

        buff.push_str(substring.as_str());
        buff.push_str("\n");
    }
    new.push_str(&buff);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("foo.txt")
        .expect("err");
    file.write(new.as_bytes()).expect("err");
}
