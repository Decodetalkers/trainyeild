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
            CliElement::print_single_from_str_with_color(ARCHLINUX, Alignment::Left, Cyan, true),
            CliElement::print_column(
                vec![
                    CliElement::print_single(
                        &[&Cyan.bold().paint("name: marine").to_string()],
                        Alignment::Left,
                    ),
                    CliElement::print_single(&["------------"], Alignment::Left),
                    CliElement::print_single(&["OS: ArchLinux"], Alignment::Left),
                    CliElement::print_single(&["Host: Yoga 15s"], Alignment::Left),
                    CliElement::print_single(&["Kernel: 6.3.0-arch1-1"], Alignment::Left),
                    CliElement::print_single(&["Wm: sway"], Alignment::Left),
                    CliElement::print_single(&["terminal: Wezterm"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
                    CliElement::print_single(&["Theme: Breeze"], Alignment::Left),
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
    CliElement::print_row(
        #[coroutine]
        || {
            yield CliElement::print_single_from_str_with_color(
                ARCHLINUX,
                Alignment::Left,
                Cyan,
                true,
            );
            yield CliElement::print_column(
                #[coroutine]
                || {
                    yield CliElement::print_single(
                        &[&Cyan.bold().paint("name: marine").to_string()],
                        Alignment::Left,
                    );
                    yield CliElement::print_single(&["------------"], Alignment::Left);
                    yield CliElement::print_single(&["OS: ArchLinux"], Alignment::Left);
                    yield CliElement::print_single(&["Host: Yoga 15s"], Alignment::Left);
                    yield CliElement::print_single(&["Kernel: 6.3.0-arch1-1"], Alignment::Left);
                    yield CliElement::print_single(&["Wm: sway"], Alignment::Left);
                    yield CliElement::print_single(&["terminal: Wezterm"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                    yield CliElement::print_single(&["Theme: Breeze"], Alignment::Left);
                },
            );
            Some(RowSettings { spacing: 1 })
        },
    )
    .draw();
}
