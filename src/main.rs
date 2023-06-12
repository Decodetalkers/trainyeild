#![feature(generators, generator_trait)]

mod elements;

fn main() {
    let test = elements::CliElement::print_column(|| {
        let unit = elements::CliElement::print_singal(&["sss"]);
        yield elements::CliElement::print_row(move || {
            let unita = unit.clone();
            yield elements::CliElement::print_singal(&["beta"]);
            yield elements::CliElement::print_singal(&["  "]);
            yield unita.clone();

            yield elements::CliElement::print_column(|| {
                yield elements::CliElement::print_emptyblock();
                yield elements::CliElement::print_singal(&["gammer", "gammer", "beta"]);
            });
            yield unit
        });
        yield elements::CliElement::print_singal(&["beta"]);
        yield elements::CliElement::print_singal(&["alpha"])
    });
    test.draw();
}
