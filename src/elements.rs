use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

#[derive(Debug, Clone)]
pub enum CliElement {
    Row { inner: Vec<CliElement> },
    Column { inner: Vec<CliElement> },
    Singal { inner: Vec<String> },
    EmptyBlock,
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
        G: Generator<Yield = CliElement, Return = CliElement> + std::marker::Unpin,
    {
        let mut inner = vec![];
        loop {
            match Pin::new(&mut generator).resume(()) {
                GeneratorState::Yielded(matrix) => inner.push(matrix),
                GeneratorState::Complete(matrix) => {
                    inner.push(matrix);
                    break;
                }
            }
        }

        CliElement::Column { inner }
    }

    #[must_use]
    pub fn print_row<G>(mut generator: G) -> Self
    where
        G: Generator<Yield = CliElement, Return = CliElement> + std::marker::Unpin,
    {
        let mut inner = vec![];
        loop {
            match Pin::new(&mut generator).resume(()) {
                GeneratorState::Yielded(matrix) => inner.push(matrix),
                GeneratorState::Complete(matrix) => {
                    inner.push(matrix);
                    break;
                }
            }
        }

        CliElement::Row { inner }
    }

    #[allow(unused)]
    pub fn draw(&self) {
        todo!()
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
            CliElement::EmptyBlock => 0,
            CliElement::Column { inner } => {
                let mut len = 0;
                for inn in inner {
                    len += inn.height();
                }
                len
            }
            CliElement::Singal { .. } => 1,
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
            unit
        });
        yield CliElement::print_singal(&["sss"]);
        CliElement::print_singal(&["sss"])
    });
    assert_eq!(test.height(), 3);
    assert_eq!(test.width(), 6);
}
