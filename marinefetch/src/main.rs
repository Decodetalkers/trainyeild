#![cfg_attr(feature = "nightly", feature(coroutines, coroutine_trait))]
mod waylandinfos;

use sctk::output::OutputInfo;
use waylandinfos::get_output_infos;
use zbus::{blocking::Connection, dbus_proxy, Result};

use cliprint::elements;
use cliprint::layout;
use cliprint::layout::RowSettings;
use elements::CliElement;
use layout::Alignment;

use nu_ansi_term::Color::Cyan;

const ARCHLINUX: &str = include_str!("../assert/archlinux.txt");

const UP_TIME: &str = "/proc/uptime";

const MEMINFO: &str = "/proc/meminfo";

static SESSION: OnceLock<Connection> = OnceLock::new();

#[cfg(target_arch = "x86_64")]
const ARCHTECHER: &str = "x86_64";
#[cfg(target_arch = "x86")]
const ARCHTECHER: &str = "x86";
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const ARCHTECHER: &str = "Unknown";

const PRODUCT_NAME: &str = "/sys/devices/virtual/dmi/id/product_name";
const PRODUCT_VERSION: &str = "/sys/devices/virtual/dmi/id/product_version";

const CPU_INFO: &str = "/proc/cpuinfo";

use std::sync::OnceLock;

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
    proxy
        .static_hostname()
        .unwrap_or("UnownHostName".to_string())
}

#[inline]
fn get_username() -> String {
    users::get_current_username()
        .unwrap_or_default()
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

fn get_memory() -> String {
    std::fs::read_to_string(MEMINFO)
        .map(|content| {
            let lines: Vec<&str> = content.lines().collect();
            let mut total_memory = String::new();
            let mut free_memory = String::new();
            for lin in lines {
                if lin.starts_with("MemTotal:") {
                    total_memory = lin
                        .split(' ')
                        .filter(|lin| !lin.is_empty())
                        .collect::<Vec<&str>>()[1]
                        .to_string();
                }
                if lin.starts_with("MemFree:") {
                    free_memory = lin
                        .split(' ')
                        .filter(|line| !line.is_empty())
                        .collect::<Vec<&str>>()[1]
                        .to_string();
                }
            }
            let total_memory: i32 = total_memory.parse::<i32>().unwrap() / 1024;
            let free_memory: i32 = free_memory.parse::<i32>().unwrap() / 1024;
            format!("{} MiB / {} MiB", free_memory, total_memory)
        })
        .unwrap_or("Unknown".to_string())
}

fn memory_element() -> CliElement {
    let memory_promote = Cyan.bold().paint("Memory");
    let memory_element = format!("{}: {}", memory_promote, get_memory());
    CliElement::print_singal(&[&memory_element], Alignment::Left)
}

fn get_shell() -> String {
    std::env::var("SHELL")
        .map(|shell| shell.split('/').last().unwrap_or("Unknown").to_string())
        .unwrap_or("Unknown".to_string())
}

fn shell_element() -> CliElement {
    let shell_promote = Cyan.bold().paint("Shell");
    let shell_element = format!("{}: {}", shell_promote, get_shell());
    CliElement::print_singal(&[&shell_element], Alignment::Left)
}

fn get_terminal() -> String {
    std::env::var("TERM_PROGRAM").unwrap_or_else(|_| {
        std::process::Command::new("tty")
            .output()
            .map(|output| String::from_utf8_lossy(output.stdout.as_ref()).to_string())
            .unwrap_or("Unkown".to_string())
            .trim()
            .to_string()
    })
}

fn terminal_element() -> CliElement {
    let terminal_promote = Cyan.bold().paint("Terminal");
    let terminal_element = format!("{}: {}", terminal_promote, get_terminal());
    CliElement::print_singal(&[&terminal_element], Alignment::Left)
}

fn get_machine_name() -> String {
    let Ok(name) = std::fs::read_to_string(PRODUCT_NAME) else {
        return "Unknown".to_string();
    };
    let name = name.trim();
    let Ok(version) = std::fs::read_to_string(PRODUCT_VERSION) else {
        return "Unknown".to_string();
    };
    let version = version.trim();
    format!("{} {}", name, version)
}

fn machine_element() -> CliElement {
    let machine_promote = Cyan.bold().paint("Host");
    let machine_element = format!("{}: {}", machine_promote, get_machine_name());
    CliElement::print_singal(&[&machine_element], Alignment::Left)
}

fn get_cpu_name() -> String {
    let Ok(cpu) = std::fs::read_to_string(CPU_INFO) else {
        return "Unknown".to_string();
    };
    let cpu = cpu.trim();
    let cpuinfo: Vec<&str> = cpu.split("\n\n").collect();
    let number = cpuinfo.len();
    let onecpu = cpuinfo[0];
    let mut cpuname = String::new();
    for info in onecpu.lines() {
        if info.starts_with("model name") {
            cpuname = info.split(':').last().unwrap_or("").to_string();
            break;
        }
    }
    let cpunameinfos: Vec<&str> = cpuname.split('@').collect();
    format!("{} ({})", cpunameinfos[0].trim(), number)
}

fn cpu_element() -> CliElement {
    let cpu_promote = Cyan.bold().paint("CPU");
    let cpu_element = format!("{}: {}", cpu_promote, get_cpu_name());
    CliElement::print_singal(&[&cpu_element], Alignment::Left)
}

fn get_gpu_names() -> Vec<String> {
    std::process::Command::new("lspci")
        .arg("-mm")
        .output()
        .map(|output| {
            let output = String::from_utf8_lossy(output.stdout.as_ref())
                .trim()
                .to_string();
            let lines: Vec<&str> = output.lines().collect();
            let mut outputs = vec![];
            for line in lines {
                let information: Vec<&str> = line
                    .split('"')
                    .filter(|line| !line.trim().is_empty())
                    .collect();
                let infoname = information[1];
                if infoname.starts_with("DISPLAY")
                    || infoname.starts_with("3D")
                    || infoname.starts_with("VGA")
                {
                    let corporations = information[2].split(' ').collect::<Vec<&str>>();
                    let corporation = corporations[0];
                    outputs.push(format!("{} {}", corporation, information[3]));
                }
            }
            outputs
        })
        .unwrap_or_default()
}

fn gpu_element(gpu: &str) -> CliElement {
    let gpu_promote = Cyan.bold().paint("GPU");
    let gpu_element = format!("{}: {}", gpu_promote, gpu);
    CliElement::print_singal(&[&gpu_element], Alignment::Left)
}

fn color_block(start: usize, end: usize, step: usize) -> String {
    let mut color = String::new();
    use nu_ansi_term::Color;
    for index in (start..end).step_by(step) {
        let style = Color::White.on(Color::Rgb(
            index as u8,
            (index * 2 % 255) as u8,
            (index * 3 % 255) as u8,
        ));
        color.push_str(&style.paint("   ").to_string());
    }
    color
}

fn color_emement() -> CliElement {
    CliElement::print_singal(&[&color_block(0, 255, 9)], Alignment::Left)
}

fn wayland_screen(info: OutputInfo) -> CliElement {
    let screen_promote = Cyan.bold().paint("Screen");
    let screen_element = format!(
        "{}: {} {}",
        screen_promote,
        info.name.unwrap_or("".to_string()),
        info.logical_size
            .map(|(x, y)| format!("{}x{}", x, y))
            .unwrap_or("Default".to_string())
    );
    CliElement::print_singal(&[&screen_element], Alignment::Left)
}

#[cfg(feature = "nightly")]
fn os_description() -> CliElement {
    CliElement::print_column(|| {
        yield hostname_element();
        yield CliElement::print_singal(&["----------"], Alignment::Left);
        yield os_name_element();
        yield machine_element();
        yield kernel_element();
        yield uptime_element();
        yield shell_element();
        yield wm_name_element();
        yield terminal_element();
        yield xdg_session_type_element();
        if xdg_session_type() == "wayland" {
            for info in get_output_infos() {
                yield wayland_screen(info);
            }
        }
        yield cpu_element();
        let gpus = get_gpu_names();
        for gpu in gpus {
            yield gpu_element(&gpu);
        }
        yield memory_element();
    })
}

#[cfg(not(feature = "nightly"))]
fn os_description() -> CliElement {
    let mut columns = vec![
        hostname_element(),
        CliElement::print_singal(&["----------"], Alignment::Left),
        os_name_element(),
        machine_element(),
        kernel_element(),
        uptime_element(),
        shell_element(),
        wm_name_element(),
        terminal_element(),
        xdg_session_type_element(),
    ];
    if xdg_session_type() == "wayland" {
        for info in get_output_infos() {
            columns.push(wayland_screen(info));
        }
    }
    columns.push(cpu_element());
    let gpus = get_gpu_names();
    for gpu in gpus {
        columns.push(gpu_element(&gpu));
    }
    columns.push(memory_element());
    CliElement::print_column(columns.into_iter())
}

#[cfg(feature = "nightly")]
fn main() {
    CliElement::print_column(|| {
        yield CliElement::print_row(|| {
            yield os_icon();
            yield os_description();
            Some(RowSettings { spacing: 1 })
        });
        yield color_emement();
    })
    .draw();
}

#[cfg(not(feature = "nightly"))]
fn main() {
    let rowelements = vec![os_icon(), os_description()];
    let top = CliElement::print_row(rowelements.into_iter(), Some(RowSettings { spacing: 1 }));
    CliElement::print_column([top, color_emement()].into_iter()).draw();
}

#[test]
fn tst_split_cpu() {
    let cpus = include_str!("../assert/example.txt");
    assert_eq!(cpus.split("\n\n").count(), 8);
}
