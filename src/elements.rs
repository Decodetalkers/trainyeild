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
        todo!()
    }

    #[allow(unused)]
    pub fn height(&self) -> usize {
        todo!()
    }
}
