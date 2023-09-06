use pyo3::prelude::*;
use pyxel_engine::Color;

use crate::image_wrapper::{wrap_pyxel_image, Image};
use crate::tilemap_wrapper::{wrap_pyxel_tilemap, Tilemap};

#[pyfunction]
fn image(img: u32) -> Image {
    wrap_pyxel_image(pyxel_engine::image(img))
}

#[pyfunction]
fn tilemap(tm: u32) -> Tilemap {
    wrap_pyxel_tilemap(pyxel_engine::tilemap(tm))
}

#[pyfunction]
fn clip(x: Option<f64>, y: Option<f64>, w: Option<f64>, h: Option<f64>) -> PyResult<()> {
    if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
        pyxel_engine::clip(x, y, w, h);
    } else if (x, y, w, h) == (None, None, None, None) {
        pyxel_engine::clip0();
    } else {
        type_error!("clip() takes 0 or 4 arguments");
    }
    Ok(())
}

#[pyfunction]
fn camera(x: Option<f64>, y: Option<f64>) -> PyResult<()> {
    if let (Some(x), Some(y)) = (x, y) {
        pyxel_engine::camera(x, y);
    } else if (x, y) == (None, None) {
        pyxel_engine::camera0();
    } else {
        type_error!("camera() takes 0 or 2 arguments");
    }
    Ok(())
}

#[pyfunction]
fn pal(col1: Option<Color>, col2: Option<Color>) -> PyResult<()> {
    if let (Some(col1), Some(col2)) = (col1, col2) {
        pyxel_engine::pal(col1, col2);
    } else if (col1, col2) == (None, None) {
        pyxel_engine::pal0();
    } else {
        type_error!("pal() takes 0 or 2 arguments");
    }
    Ok(())
}

#[pyfunction]
fn cls(col: Color) {
    pyxel_engine::cls(col);
}

#[pyfunction]
fn pget(x: f64, y: f64) -> Color {
    pyxel_engine::pget(x, y)
}

#[pyfunction]
fn pset(x: f64, y: f64, col: Color) {
    pyxel_engine::pset(x, y, col);
}

#[pyfunction]
fn line(x1: f64, y1: f64, x2: f64, y2: f64, col: Color) {
    pyxel_engine::line(x1, y1, x2, y2, col);
}

#[pyfunction]
fn rect(x: f64, y: f64, w: f64, h: f64, col: Color) {
    pyxel_engine::rect(x, y, w, h, col);
}

#[pyfunction]
fn rectb(x: f64, y: f64, w: f64, h: f64, col: Color) {
    pyxel_engine::rectb(x, y, w, h, col);
}

#[pyfunction]
fn circ(x: f64, y: f64, r: f64, col: Color) {
    pyxel_engine::circ(x, y, r, col);
}

#[pyfunction]
fn circb(x: f64, y: f64, r: f64, col: Color) {
    pyxel_engine::circb(x, y, r, col);
}

#[pyfunction]
fn elli(x: f64, y: f64, w: f64, h: f64, col: Color) {
    pyxel_engine::elli(x, y, w, h, col);
}

#[pyfunction]
fn ellib(x: f64, y: f64, w: f64, h: f64, col: Color) {
    pyxel_engine::ellib(x, y, w, h, col);
}

#[pyfunction]
fn tri(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: Color) {
    pyxel_engine::tri(x1, y1, x2, y2, x3, y3, col);
}

#[pyfunction]
fn trib(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: Color) {
    pyxel_engine::trib(x1, y1, x2, y2, x3, y3, col);
}

#[pyfunction]
fn fill(x: f64, y: f64, col: Color) {
    pyxel_engine::fill(x, y, col);
}

#[pyfunction]
fn blt(
    x: f64,
    y: f64,
    img: &PyAny,
    u: f64,
    v: f64,
    w: f64,
    h: f64,
    colkey: Option<Color>,
) -> PyResult<()> {
    type_switch! {
        img,
        u32, {
            pyxel_engine::blt(x, y, img, u, v, w, h, colkey);
        },
        Image, {
            pyxel_engine::screen().lock().blt(x, y, img.pyxel_image, u, v, w, h, colkey);
        }
    }
    Ok(())
}

#[pyfunction]
fn bltm(
    x: f64,
    y: f64,
    tm: &PyAny,
    u: f64,
    v: f64,
    w: f64,
    h: f64,
    colkey: Option<Color>,
) -> PyResult<()> {
    type_switch! {
        tm,
        u32, {
            pyxel_engine::bltm(x, y, tm, u, v, w, h, colkey);
        },
        Tilemap, {
            pyxel_engine::screen().lock().bltm(x, y, tm.pyxel_tilemap, u, v, w, h, colkey);
        }
    }
    Ok(())
}

#[pyfunction]
fn text(x: f64, y: f64, s: &str, col: Color) {
    pyxel_engine::text(x, y, s, col);
}

pub fn add_graphics_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(image, m)?)?;
    m.add_function(wrap_pyfunction!(tilemap, m)?)?;
    m.add_function(wrap_pyfunction!(clip, m)?)?;
    m.add_function(wrap_pyfunction!(camera, m)?)?;
    m.add_function(wrap_pyfunction!(pal, m)?)?;
    m.add_function(wrap_pyfunction!(cls, m)?)?;
    m.add_function(wrap_pyfunction!(pget, m)?)?;
    m.add_function(wrap_pyfunction!(pset, m)?)?;
    m.add_function(wrap_pyfunction!(line, m)?)?;
    m.add_function(wrap_pyfunction!(rect, m)?)?;
    m.add_function(wrap_pyfunction!(rectb, m)?)?;
    m.add_function(wrap_pyfunction!(circ, m)?)?;
    m.add_function(wrap_pyfunction!(circb, m)?)?;
    m.add_function(wrap_pyfunction!(elli, m)?)?;
    m.add_function(wrap_pyfunction!(ellib, m)?)?;
    m.add_function(wrap_pyfunction!(tri, m)?)?;
    m.add_function(wrap_pyfunction!(trib, m)?)?;
    m.add_function(wrap_pyfunction!(fill, m)?)?;
    m.add_function(wrap_pyfunction!(blt, m)?)?;
    m.add_function(wrap_pyfunction!(bltm, m)?)?;
    m.add_function(wrap_pyfunction!(text, m)?)?;
    Ok(())
}