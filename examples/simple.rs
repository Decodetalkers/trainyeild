#![feature(generators, generator_trait)]
use cliprint::elements;
use cliprint::layout;
use cliprint::layout::RowSettings;
use elements::CliElement;
use layout::Alignment;

const ARCHLINUX: &str = include_str!("../assert/archlinux.txt");

fn main() {
    CliElement::print_row(|| {
        let archlinux: Vec<&str> = ARCHLINUX.lines().collect();
        yield CliElement::print_singal(&archlinux, Alignment::Left);
        yield CliElement::print_column(|| {
            yield CliElement::print_singal(&["name: marine"], Alignment::Left);
            yield CliElement::print_singal(&["------------"], Alignment::Left);
            yield CliElement::print_singal(&["OS: ArchLinux"], Alignment::Left);
            yield CliElement::print_singal(&["Host: Yoga 15s"], Alignment::Left);
            yield CliElement::print_singal(&["Kernel: 6.3.0-arch1-1"], Alignment::Left);
            yield CliElement::print_singal(&["Wm: sway"], Alignment::Left);
        });
        Some(RowSettings { spacing: 2 })
    })
    .draw();
}
