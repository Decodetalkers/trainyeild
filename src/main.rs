#![feature(generators, generator_trait)]

mod elements;

fn main() {
    let test = elements::CliElement::print_column(|| {
        let unit = elements::CliElement::print_singal(&["sss"]);
        yield elements::CliElement::print_row(move || {
            let unita = unit.clone();
            yield unita.clone();
            unit
        });
        yield elements::CliElement::print_singal(&["sss"]);
        elements::CliElement::print_singal(&["sss"])
    });
    println!("{test:?}");
}
