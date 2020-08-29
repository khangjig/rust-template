use rocket::http::RawStr;
use rocket_contrib::json::Json;

use crate::common::profanity::{add_profanity, sensor_profanity};
use crate::LANGUAGE;
use crate::model::response::{ResponseError, ResponseSuccess};
use crate::model::sensor::AddSensor;

#[get("/?<text>")]
pub(crate) fn sensor(text: &RawStr) -> String {
    let resp = sensor_profanity(&String::from(text.url_decode_lossy()));
    format!("{}", resp)
}

#[post("/", format = "json", data = "<words>")]
pub(crate) fn add(words: Json<AddSensor>) -> Result<Json<ResponseSuccess>, Json<ResponseError>> {
    let mut checked: bool = false;
    let lang = words.language.to_lowercase();

    for n in 0..LANGUAGE.len() {
        if lang == LANGUAGE[n] {
            checked = true;
            break;
        }
    }

    if checked {
        for n in 0..words.words.len() {
            add_profanity(words.words[n].as_str(), lang.as_str())
        }

        return Ok(Json(ResponseSuccess {
            code: 200,
            data: Default::default(),
            message: String::from("successfully!"),
        }));
    }

    return Err(Json(ResponseError {
        code: 400,
        message: String::from("failed!"),
    }));
}