#![feature(coroutines, coroutine_trait)]
use cliprint::elements;
use cliprint::layout;
use cliprint::layout::RowSettings;
use elements::CliElement;
use layout::Alignment;

use nu_ansi_term::Color::Cyan;

const ARCHLINUX: &str = include_str!("../assert/archlinux.txt");

#[cfg(not(feature = "nightly"))]
fn main() {
    CliElement::print_row(
        vec![
            CliElement::print_singal_from_str_with_color(ARCHLINUX, Alignment::Left, Cyan, true),
            CliElement::print_column(
                vec![
                    CliElement::print_singal(
                        &[&Cyan.bold().paint("name: marine").to_string()],
                        Alignment::Left,
                    ),
                    CliElement::print_singal(&["------------"], Alignment::Left),
                    CliElement::print_singal(&["OS: ArchLinux"], Alignment::Left),
                    CliElement::print_singal(&["Host: Yoga 15s"], Alignment::Left),
                    CliElement::print_singal(&["Kernel: 6.3.0-arch1-1"], Alignment::Left),
                    CliElement::print_singal(&["Wm: sway"], Alignment::Left),
                    CliElement::print_singal(&["terminal: Wezterm"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_singal(&["Theme: Breeze"], Alignment::Left),
                ]
                .into_iter(),
            ),
        ]
        .into_iter(),
        Some(RowSettings { spacing: 1 }),
    )
    .draw();
}

#[cfg(feature = "nightly")]
fn main() {
    CliElement::print_row(|| {
        yield CliElement::print_singal_from_str_with_color(ARCHLINUX, Alignment::Left, Cyan, true);
        yield CliElement::print_column(|| {
            yield CliElement::print_singal(
                &[&Cyan.bold().paint("name: marine").to_string()],
                Alignment::Left,
            );
            yield CliElement::print_singal(&["------------"], Alignment::Left);
            yield CliElement::print_singal(&["OS: ArchLinux"], Alignment::Left);
            yield CliElement::print_singal(&["Host: Yoga 15s"], Alignment::Left);
            yield CliElement::print_singal(&["Kernel: 6.3.0-arch1-1"], Alignment::Left);
            yield CliElement::print_singal(&["Wm: sway"], Alignment::Left);
            yield CliElement::print_singal(&["terminal: Wezterm"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
            yield CliElement::print_singal(&["Theme: Breeze"], Alignment::Left);
        });
        Some(RowSettings { spacing: 1 })
    })
    .draw();
}
