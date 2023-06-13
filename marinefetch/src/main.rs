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

const UP_TIME: &str = "/proc/uptime";

use std::sync::OnceLock;

static SESSION: OnceLock<Connection> = OnceLock::new();

#[cfg(target_arch = "x86_64")]
const ARCHTECHER: &str = "x86_64";
#[cfg(target_arch = "x86")]
const ARCHTECHER: &str = "x86";
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const ARCHTECHER: &str = "Unknown";

fn get_connection() -> zbus::Result<Connection> {
    if let Some(cnx) = SESSION.get() {
        Ok(cnx.clone())
    } else {
        let cnx = Connection::system()?;
        SESSION.set(cnx.clone()).expect("Can't reset a OnceCell");
        Ok(cnx)
    }
}

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
    #[dbus_proxy(property)]
    fn kernel_release(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn hardware_vendor(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn operating_system_pretty_name(&self) -> Result<String>;
}

fn get_hostname() -> String {
    let connection = get_connection().unwrap();
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
fn hostname_element() -> CliElement {
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

#[inline]
fn xdg_session_type() -> String {
    std::env::var("XDG_SESSION_TYPE").unwrap_or("Unknown".to_string())
}

#[inline]
fn xdg_session_type_element() -> CliElement {
    let xdg_promote = Cyan.bold().paint("SessionType");
    let xdg_element = format!("{}: {}", xdg_promote, xdg_session_type());
    CliElement::print_singal(&[&xdg_element], Alignment::Left)
}

fn get_uptime() -> String {
    std::fs::read_to_string(UP_TIME)
        .map(|content| {
            let runtime_str = content.trim().split(' ').collect::<Vec<_>>()[0].to_string();
            let runtime: f64 = runtime_str.parse().unwrap_or(0.0);
            let miniute = (runtime / 60.0) as i32;
            let second = (runtime % 60.0) as i32;
            let finaloutput = format!("{}min {}s", miniute, second);
            finaloutput
        })
        .unwrap_or("0".to_string())
}

fn uptime_element() -> CliElement {
    let uptime_promote = Cyan.bold().paint("Uptime");
    let uptime_element = format!("{}: {}", uptime_promote, get_uptime());
    CliElement::print_singal(&[&uptime_element], Alignment::Left)
}

fn get_kernel() -> String {
    let connection = get_connection().unwrap();
    let proxy = Hostname1ProxyBlocking::new(&connection).unwrap();
    proxy.kernel_release().unwrap()
}

fn kernel_element() -> CliElement {
    let kernel_promote = Cyan.bold().paint("Kernel");
    let kernel_element = format!("{}: {}", kernel_promote, get_kernel());
    CliElement::print_singal(&[&kernel_element], Alignment::Left)
}

fn get_os_name() -> String {
    let connection = get_connection().unwrap();
    let proxy = Hostname1ProxyBlocking::new(&connection).unwrap();
    format!(
        "{} {}",
        proxy.operating_system_pretty_name().unwrap(),
        ARCHTECHER
    )
}

fn os_name_element() -> CliElement {
    let os_name_promote = Cyan.bold().paint("OS");
    let os_name_element = format!("{}: {}", os_name_promote, get_os_name());
    CliElement::print_singal(&[&os_name_element], Alignment::Left)
}

fn os_description() -> CliElement {
    CliElement::print_column(move || {
        yield hostname_element();
        yield CliElement::print_singal(&["----------"], Alignment::Left);
        yield os_name_element();
        yield kernel_element();
        yield uptime_element();
        yield wm_name_element();
        yield xdg_session_type_element();
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
