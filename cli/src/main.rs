use clap::{App, Arg};
use log::{info, warn};
use regex::Regex;
use std::env;

fn main() {
    // Inicializar el logger
    env_logger::init();

    let matches = App::new("MAC Formatter")
        .version("1.0")
        .author("Carlos Sigua")
        .about("Formatea direcciones MAC en diferentes formatos")
        .arg(
            Arg::with_name("MAC")
                .help("Dirección MAC en formato xx:xx:xx:xx:xx:xx")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Obtener el valor del argumento MAC
    let mac_address = matches.value_of("MAC").unwrap();
    recognize_format_mac(mac_address);  // Llamada a la función recognize_format_mac
    // Aquí puedes continuar con la lógica para formatear la dirección MAC
}

fn recognize_format_mac(mac_address: &str) {
    let re_colon = Regex::new(r"^[0-9a-fA-F]{2}(:[0-9a-fA-F]{2}){5}$").unwrap();
    let re_dash = Regex::new(r"^[0-9a-fA-F]{2}(-[0-9a-fA-F]{2}){5}$").unwrap();
    let re_no_delimiter = Regex::new(r"^[0-9a-fA-F]{12}$").unwrap();

    if re_colon.is_match(mac_address) {
        log::info!("Formato: xx:xx:xx:xx:xx:xx");
    } else if re_dash.is_match(mac_address) {
        log::info!("Formato: xx-xx-xx-xx-xx-xx");
    } else if re_no_delimiter.is_match(mac_address) {
        log::info!("Formato: xxxxxxxxxxxx");
    } else {
        log::warn!("Formato no reconocido para la dirección MAC: {}", mac_address);
    }
}
