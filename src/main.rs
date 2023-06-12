#![feature(generators, generator_trait)]

mod elements;

use elements::CliElement;

fn main() {
    CliElement::print_column(|| {
        let unit = CliElement::print_singal(&["sss"]);
        yield CliElement::print_row(move || {
            let unita = unit.clone();
            yield CliElement::print_singal(&["beta"]);
            yield CliElement::print_singal(&["  "]);
            yield unita.clone();

            yield CliElement::print_column(|| {
                yield CliElement::print_emptyblock();
                yield CliElement::print_singal(&["gammer", "gammer", "beta"]);
            });
            yield unit
        });
        yield CliElement::print_singal(&["beta"]);
        yield CliElement::print_singal(&["alpha"])
    })
    .draw();
}
