#![feature(generators, generator_trait)]

mod elements;

mod layout;

use elements::CliElement;
use layout::RowSettings;

fn main() {
    CliElement::print_column(|| {
        let unit = CliElement::print_singal(&["sss"], layout::Alignment::Left);
        yield CliElement::print_row(move || {
            let unita = unit.clone();
            yield CliElement::print_singal(&["beta"], layout::Alignment::Left);
            yield unita.clone();

            yield CliElement::print_column(|| {
                yield CliElement::print_emptyblock();
                yield CliElement::print_singal(
                    &["gammer", "gammer", "beta"],
                    layout::Alignment::Right,
                );
            });
            yield unit;
            Some(RowSettings { spacing: 2 })
        });
        yield CliElement::print_singal(&["beta"], layout::Alignment::Right);
        yield CliElement::print_singal(&["alpha"], layout::Alignment::Right)
    })
    .draw();
}
