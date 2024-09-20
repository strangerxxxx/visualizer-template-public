use proconio::input;

pub struct Input {
    n: usize,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.n)?;
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n: usize,
    }
    Input { n }
}

pub struct Output {}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    Ok(Output {})
}

pub fn gen(seed: usize) -> Input {
    Input { n: seed }
}

// 上に公式配布のソースコードをコピーする

mod parts;
use svg::node::element::Style;

pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

pub fn vis(input: &Input, out: &Output, turn: usize) -> (i64, String, String) {
    (0, "".to_string(), "".to_string())
}
