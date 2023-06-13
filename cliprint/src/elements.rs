use std::collections::HashMap;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

use strfmt::strfmt;

use crate::layout::{Alignment, RowSettings};

#[cfg(feature = "color")]
use nu_ansi_term::Color;

/// It is the element of cli, a unit
/// privide Row, Column, Singal Singal
/// ```
/// use cliprint::elements::CliElement;
/// let a = CliElement::EmptyBlock;
/// ```
/// Other one should build with impl
#[derive(Debug, Clone)]
pub enum CliElement {
    Row {
        inner: Vec<CliElement>,
        settings: Option<RowSettings>,
    },
    Column {
        inner: Vec<CliElement>,
    },
    Singal {
        inner: Vec<String>,
        layout: Alignment,
    },
    EmptyBlock,
}

fn init_matrix(heigth: usize) -> Vec<String> {
    let mut output = vec![];
    for _ in 0..heigth {
        output.push(String::new());
    }
    output
}

fn init_string_with_width(width: usize) -> String {
    let mut output = String::new();
    for _ in 0..width {
        output.push(' ');
    }
    output
}

impl CliElement {
    /// use a matrix to init a CliElement::Singal
    /// ```
    /// use cliprint::elements::CliElement;
    /// use cliprint::layout::Alignment;
    /// let a = CliElement::print_singal(&["sss","bbb"], Alignment::Left);
    /// ```
    #[must_use]
    pub fn print_singal(matrix: &[&str], layout: Alignment) -> Self {
        let mut inner = vec![];
        for mat in matrix {
            inner.push(mat.to_string());
        }
        CliElement::Singal { inner, layout }
    }

    /// same as print_singal, but with string
    /// ```
    /// use cliprint::elements::CliElement;
    /// use cliprint::layout::Alignment;
    /// let archlinux = include_str!("../../assert/archlinux.txt");
    /// let a = CliElement::print_singal_from_str(archlinux, Alignment::Left);
    /// ```
    #[must_use]
    pub fn print_singal_from_str(matrix: &str, layout: Alignment) -> Self {
        let matrix: Vec<&str> = matrix.lines().collect();
        Self::print_singal(&matrix, layout)
    }

    /// same as print_singal_from_str, but privde color, use nu_ansi_term
    #[cfg(feature = "color")]
    #[must_use]
    pub fn print_singal_from_str_with_color(
        matrix: &str,
        layout: Alignment,
        color: Color,
        is_bold: bool,
    ) -> Self {
        let matrix: Vec<String> = matrix
            .lines()
            .map(|line| {
                if is_bold {
                    color.bold().paint(line).to_string()
                } else {
                    color.paint(line).to_string()
                }
            })
            .collect();
        let matrix: Vec<&str> = matrix.iter().map(|s| s.as_str()).collect();
        Self::print_singal(&matrix, layout)
    }

    #[must_use]
    pub fn print_emptyblock() -> Self {
        CliElement::EmptyBlock
    }

    #[must_use]
    pub fn print_column<G>(mut generator: G) -> Self
    where
        G: Generator<Yield = CliElement, Return = ()> + std::marker::Unpin,
    {
        let mut inner = vec![];
        while let GeneratorState::Yielded(matrix) = Pin::new(&mut generator).resume(()) {
            inner.push(matrix)
        }

        CliElement::Column { inner }
    }

    #[must_use]
    pub fn print_row<G>(mut generator: G) -> Self
    where
        G: Generator<Yield = CliElement, Return = Option<RowSettings>> + std::marker::Unpin,
    {
        let mut inner = vec![];
        let settings;
        loop {
            match Pin::new(&mut generator).resume(()) {
                GeneratorState::Yielded(matrix) => inner.push(matrix),
                GeneratorState::Complete(setting) => {
                    settings = setting;
                    break;
                }
            }
        }

        CliElement::Row { inner, settings }
    }

    fn get_draw_map(&self, draw_width: usize) -> Vec<String> {
        match self {
            CliElement::EmptyBlock => {
                let formatalign = format!("{{content: <{}}}", draw_width);
                let format_res = strfmt(
                    &formatalign,
                    &HashMap::from([("content".to_string(), String::new())]),
                )
                .unwrap();
                vec![format_res]
            }
            CliElement::Singal { inner, layout } => {
                let formatalign = match layout {
                    Alignment::Left => format!("{{content: <{}}}", draw_width),
                    Alignment::Right => format!("{{content: >{}}}", draw_width),
                };
                let mut output = vec![];
                for inn in inner {
                    output.push({
                        strfmt(
                            &formatalign,
                            &HashMap::from([("content".to_string(), inn.clone())]),
                        )
                        .unwrap()
                    });
                }
                output
            }
            CliElement::Column { inner } => {
                let mut output = vec![];
                for inn in inner {
                    output.append(&mut inn.get_draw_map(draw_width));
                }
                output
            }
            CliElement::Row { inner, settings } => {
                let spacewidth = settings.and_then(|a| Some(a.spacing)).unwrap_or(0);
                let spacestring = init_string_with_width(spacewidth);
                let height = self.height();
                let mut adjust = init_matrix(height);
                for item in adjust.iter_mut().take(height) {
                    item.push_str(&spacestring);
                }
                for inn in inner {
                    let mut inn2 = inn.get_draw_map(inn.width());
                    let formatalign = format!("{{content: <{}}}", inn.width());
                    for _ in inn.height()..height {
                        inn2.push(
                            strfmt(
                                &formatalign,
                                &HashMap::from([("content".to_string(), String::new())]),
                            )
                            .unwrap(),
                        );
                    }
                    for index in 0..height {
                        adjust[index].push_str(&inn2[index]);
                        adjust[index].push_str(&spacestring);
                    }
                }
                adjust
            }
        }
    }

    pub fn draw(&self) {
        let map = self.get_draw_map(self.width());
        for ma in map {
            println!("{}", ma);
        }
    }

    pub fn width(&self) -> usize {
        match self {
            CliElement::Row { inner, settings } => {
                let mut len = 0;
                for inn in inner {
                    len += inn.width();
                }
                let spacwidth = settings.and_then(|a| Some(a.spacing)).unwrap_or(0);
                len += (inner.len() + 1) * spacwidth;
                len
            }
            CliElement::EmptyBlock => 0,
            CliElement::Column { inner } => {
                let mut len = 0;
                for inn in inner {
                    if inn.width() > len {
                        len = inn.width();
                    }
                }
                len
            }
            CliElement::Singal { inner, .. } => {
                let mut len = 0;
                for inn in inner {
                    if inn.len() > len {
                        len = inn.len();
                    }
                }
                len
            }
        }
    }

    pub fn height(&self) -> usize {
        match self {
            CliElement::Row { inner, .. } => {
                let mut len = 0;
                for inn in inner {
                    if inn.height() > len {
                        len = inn.height();
                    }
                }
                len
            }
            CliElement::EmptyBlock => 1,
            CliElement::Column { inner } => {
                let mut len = 0;
                for inn in inner {
                    len += inn.height();
                }
                len
            }
            CliElement::Singal { inner, .. } => inner.len(),
        }
    }
}

#[test]
fn tst_len() {
    let test = CliElement::print_column(|| {
        let unit = CliElement::print_singal(&["sss"], Alignment::Left);
        yield CliElement::print_row(move || {
            let unita = unit.clone();
            yield unita.clone();
            yield unit;
            None
        });
        yield CliElement::print_singal(&["sss"], Alignment::Left);
        yield CliElement::print_singal(&["sss"], Alignment::Left)
    });
    assert_eq!(test.height(), 3);
    assert_eq!(test.width(), 6);
}
