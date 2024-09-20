use wasm_bindgen::prelude::*;
mod util;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    util::gen(seed as usize).to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = util::parse_input(&_input);
    let str: &str = &_output;
    let output = util::parse_output(&input, &str);
    let (score, err, svg) = util::vis(&input, &output.unwrap(), turn);
    Ret {
        score: score as i64,
        err,
        svg,
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let input = util::parse_input(&_input);
    let output = util::parse_output(&input, &_output);
    0
}
