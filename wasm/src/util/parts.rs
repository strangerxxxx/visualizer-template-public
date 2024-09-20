#![allow(non_snake_case, unused_macros)]

use svg::node::element::{Circle, Group, Line, Rectangle, Text, Title};

//https://zenn.dev/tipstar0125/articles/d2cf0ef63bceb7
// 0.0(透明) <= opacity(透明度) <= 1.0(不透明)
pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str, opacity: f64) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
        .set("fill-opacity", opacity)
}

pub fn rect_with_stroke(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    fill: &str,
    opacity: f64,
    stroke_color: &str,
    stroke_width: usize,
) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
        .set("fill-opacity", opacity)
        .set("stroke", stroke_color)
        .set("stroke-width", stroke_width)
}

pub fn cir(x: usize, y: usize, r: usize, fill: &str) -> Circle {
    Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", r)
        .set("fill", fill)
}

pub fn line(x1: usize, y1: usize, x2: usize, y2: usize, color: &str, width: usize) -> Line {
    Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", color)
        .set("stroke-width", width)
        .set("stroke-linecap", "round")
}

pub fn dashline(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: &str,
    width: usize,
    stroke: usize,
) -> Line {
    Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", color)
        .set("stroke-width", width)
        .set("stroke-linecap", "round")
        .set("stroke-dasharray", stroke)
}

pub fn txt(x: usize, y: usize, text: &str, font_size: usize) -> Text {
    Text::new(text)
        .set("x", x)
        .set("y", y)
        .set("fill", "black")
        .set("font-size", font_size)
        .set("dominant-baseline", "central") // 上下中央揃え
        .set("text-anchor", "middle") // 左右中央揃え
}

// 0.0 <= val <= 1.0
pub fn color(mut val: f64) -> String {
    val = val.min(1.0);
    val = val.max(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        r.round() as i32,
        g.round() as i32,
        b.round() as i32
    )
}

pub fn group(title: String) -> Group {
    Group::new().add(Title::new(title))
}
