#![feature(generators, generator_trait)]

mod matrix;

fn main() {
    matrix::CliElement::print_column(|| {
        let unit = matrix::CliElement::print_singal(&["sss"]);
        yield matrix::CliElement::print_row(move || {
            let unita = unit.clone();
            yield unita.clone();
            unit
        });
        yield matrix::CliElement::print_singal(&["sss"]);
        matrix::CliElement::print_singal(&["sss"])
    });
}
