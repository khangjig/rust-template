#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};

use proto::profanity::*;
use proto::profanity_grpc::*;

use crate::common::profanity::sensor_profanity;

mod model;
mod api;
mod common;
mod proto;

const LANGUAGE: &'static [&str] = &["vi", "en"];


struct ProfanityServiceImpl;

impl Profanity for ProfanityServiceImpl {
    fn sensor_profanity(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<ProfanityRequest>,
        resp: ServerResponseUnarySink<ProfanityResponse>,
    ) -> grpc::Result<()> {
        resp.finish(ProfanityResponse {
            text: sensor_profanity(&req.message.text),
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        })
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();

    server.http.set_port(8000);
    server.add_service(ProfanityServer::new_service_def(ProfanityServiceImpl));
    let _server = server.build().expect("server");

    rocket::ignite()
        .mount("/sensor", routes![
            api::censor::add,
            api::censor::sensor,
        ])
        .launch();
}