#![feature(generators, generator_trait)]
use std::ffi::OsString;

use zbus::{blocking::Connection, dbus_proxy, Result};

use cliprint::elements;
use cliprint::layout;
use cliprint::layout::RowSettings;
use elements::CliElement;
use layout::Alignment;

use nu_ansi_term::Color::Cyan;

const ARCHLINUX: &str = include_str!("../../assert/archlinux.txt");

#[dbus_proxy(
    interface = "org.freedesktop.hostname1",
    default_service = "org.freedesktop.hostname1",
    default_path = "/org/freedesktop/hostname1"
)]
trait Hostname1 {
    fn describe(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn static_hostname(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn icon_name(&self) -> Result<String>;
}

fn get_hostname() -> String {
    let connection = Connection::system().unwrap();
    let proxy = Hostname1ProxyBlocking::new(&connection).unwrap();
    proxy.static_hostname().unwrap()
}

#[inline]
fn get_username() -> String {
    users::get_current_username()
        .unwrap_or(OsString::new())
        .to_string_lossy()
        .to_string()
}

#[inline]
fn os_name_element() -> CliElement {
    let username = Cyan.bold().paint(get_username());
    let hostname = Cyan.bold().paint(get_hostname());
    let os_name = format!("{}@{}", username, hostname);

    CliElement::print_singal(&[&os_name], Alignment::Left)
}

#[inline]
fn os_icon() -> CliElement {
    CliElement::print_singal_from_str_with_color(ARCHLINUX, Alignment::Left, Cyan, true)
}

#[inline]
fn wm_name() -> String {
    std::env::var("XDG_CURRENT_DESKTOP").unwrap_or("nofind".to_string())
}

#[inline]
fn wm_name_element() -> CliElement {
    let wm_promote = Cyan.bold().paint("WM");
    let wm_element = format!("{}: {}", wm_promote, wm_name());
    CliElement::print_singal(&[&wm_element], Alignment::Left)
}

fn os_description() -> CliElement {
    CliElement::print_column(move || {
        yield os_name_element();
        yield CliElement::print_singal(&["----------"], Alignment::Left);
        yield wm_name_element();
    })
}

fn main() {
    CliElement::print_row(|| {
        yield os_icon();
        yield os_description();
        Some(RowSettings { spacing: 1 })
    })
    .draw();
}
