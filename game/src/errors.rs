use wasm_bindgen::JsValue;

#[allow(dead_code)]
#[derive(Debug)]
pub struct GameError {
    message: String,
    payload: Option<String>,
}

impl GameError {
    pub fn new(message: &'static str, payload: Option<String>) -> GameError {
        GameError {
            message: message.to_string(),
            payload,
        }
    }
    pub fn msg(message: &'static str) -> GameError {
        GameError::new(message, None)
    }
    pub fn js(message: &'static str) -> impl Fn(JsValue) -> GameError {
        move |payload: JsValue| -> GameError {
            GameError {
                message: message.to_string(),
                payload: payload.as_string(),
            }
        }
    }
}

pub type GameResult<T> = Result<T, GameError>;
