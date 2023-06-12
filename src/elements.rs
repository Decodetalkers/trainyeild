use std::collections::HashMap;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

use strfmt::strfmt;

#[derive(Debug, Clone)]
pub enum CliElement {
    Row { inner: Vec<CliElement> },
    Column { inner: Vec<CliElement> },
    Singal { inner: Vec<String> },
    EmptyBlock,
}

fn init_matrix(heigth: usize) -> Vec<String> {
    let mut output = vec![];
    for _ in 0..heigth {
        output.push(String::new());
    }
    output
}

impl CliElement {
    pub fn print_singal(matrix: &[&str]) -> Self {
        let mut inner = vec![];
        for mat in matrix {
            inner.push(mat.to_string());
        }
        CliElement::Singal { inner }
    }

    #[allow(unused)]
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
        G: Generator<Yield = CliElement, Return = ()> + std::marker::Unpin,
    {
        let mut inner = vec![];
        while let GeneratorState::Yielded(matrix) = Pin::new(&mut generator).resume(()) {
            inner.push(matrix)
        }
        // TODO: complete need with some option
        // loop {
        //     match Pin::new(&mut generator).resume(()) {
        //         GeneratorState::Yielded(matrix) => inner.push(matrix),
        //         GeneratorState::Complete(()) => {
        //             break;
        //         }
        //     }
        // }

        CliElement::Row { inner }
    }

    #[allow(unused)]
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
            CliElement::Singal { inner } => {
                let formatalign = format!("{{content: <{}}}", draw_width);
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
            CliElement::Row { inner } => {
                let height = self.height();
                let mut adjust = init_matrix(height);
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
                    //adjust.push(inn2);
                    for index in 0..height {
                        adjust[index].push_str(&inn2[index]);
                    }
                }
                adjust
            }
        }
    }

    #[allow(unused)]
    pub fn draw(&self) {
        let map = self.get_draw_map(self.width());
        for ma in map {
            println!("{}", ma);
        }
    }

    #[allow(unused)]
    pub fn width(&self) -> usize {
        match self {
            CliElement::Row { inner } => {
                let mut len = 0;
                for inn in inner {
                    len += inn.width();
                }
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
            CliElement::Singal { inner } => {
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

    #[allow(unused)]
    pub fn height(&self) -> usize {
        match self {
            CliElement::Row { inner } => {
                let mut len = 0;
                for inn in inner {
                    if inn.width() > len {
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
            CliElement::Singal { inner } => inner.len(),
        }
    }
}

#[test]
fn tst_len() {
    let test = CliElement::print_column(|| {
        let unit = CliElement::print_singal(&["sss"]);
        yield CliElement::print_row(move || {
            let unita = unit.clone();
            yield unita.clone();
            yield unit;
        });
        yield CliElement::print_singal(&["sss"]);
        yield CliElement::print_singal(&["sss"])
    });
    assert_eq!(test.height(), 3);
    assert_eq!(test.width(), 6);
}
