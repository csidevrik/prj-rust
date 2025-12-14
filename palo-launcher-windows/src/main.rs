use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "palo-launcher")]
#[command(about = "Herramienta CLI para gestionar conexiones al Firewall Palo Alto", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Configurar la IP del firewall y navegadores
    Config,
    /// Conectar al firewall (modo interactivo)
    Connect,
    /// Mostrar configuración actual
    Show,
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    firewall_ip: String,
    firefox_profile: String,
    brave_profile: String,
    chrome_profile: String,
    edge_profile: String,
    default_browsers: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            firewall_ip: String::new(),
            firefox_profile: "palo-firefox".to_string(),
            brave_profile: "palo-brave".to_string(),
            chrome_profile: "palo-chrome".to_string(),
            edge_profile: "palo-edge".to_string(),
            #[cfg(target_os = "windows")]
            default_browsers: vec!["firefox".to_string(), "edge".to_string()],
            #[cfg(not(target_os = "windows"))]
            default_browsers: vec!["firefox".to_string(), "brave".to_string()],
        }
    }
}

struct PaloLauncher {
    config_path: PathBuf,
    config: Config,
}

impl PaloLauncher {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("No se pudo obtener el directorio de configuración")?
            .join("palo-launcher");
        
        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("config.json");
        
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content)?
        } else {
            Config::default()
        };
        
        Ok(PaloLauncher { config_path, config })
    }
    
    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, json)?;
        Ok(())
    }
    
    fn configure(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", "=== Configuración de Palo Alto Launcher ===".green().bold());
        println!();
        
        let ip: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("IP de administración del Palo Alto")
            .default(self.config.firewall_ip.clone())
            .interact_text()?;
        
        self.config.firewall_ip = ip;
        
        println!("\n{}", "Selecciona los navegadores a usar (por defecto):".cyan());
        
        #[cfg(target_os = "windows")]
        let browsers = vec![
            "Firefox + Edge",
            "Firefox + Chrome", 
            "Edge + Chrome",
            "Solo Firefox (2 ventanas)"
        ];
        
        #[cfg(not(target_os = "windows"))]
        let browsers = vec![
            "Firefox + Brave", 
            "Firefox + Chrome", 
            "Solo Firefox (2 ventanas)"
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&browsers)
            .default(0)
            .interact()?;
        
        #[cfg(target_os = "windows")]
        {
            self.config.default_browsers = match selection {
                0 => vec!["firefox".to_string(), "edge".to_string()],
                1 => vec!["firefox".to_string(), "chrome".to_string()],
                2 => vec!["edge".to_string(), "chrome".to_string()],
                3 => vec!["firefox".to_string(), "firefox2".to_string()],
                _ => vec!["firefox".to_string(), "edge".to_string()],
            };
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            self.config.default_browsers = match selection {
                0 => vec!["firefox".to_string(), "brave".to_string()],
                1 => vec!["firefox".to_string(), "chrome".to_string()],
                2 => vec!["firefox".to_string(), "firefox2".to_string()],
                _ => vec!["firefox".to_string(), "brave".to_string()],
            };
        }
        
        self.save_config()?;
        println!("\n{}", "✓ Configuración guardada exitosamente".green());
        
        Ok(())
    }
    
    fn show_config(&self) {
        println!("{}", "=== Configuración Actual ===".cyan().bold());
        println!("IP del Firewall: {}", self.config.firewall_ip.yellow());
        println!("Navegadores: {}", self.config.default_browsers.join(" + ").yellow());
        println!("Perfil Firefox: {}", self.config.firefox_profile.yellow());
        println!("Perfil Brave: {}", self.config.brave_profile.yellow());
        println!("Perfil Chrome: {}", self.config.chrome_profile.yellow());
        
        #[cfg(target_os = "windows")]
        println!("Perfil Edge: {}", self.config.edge_profile.yellow());
        
        #[cfg(target_os = "windows")]
        println!("\nDirectorio de config: {}", 
            dirs::config_dir().unwrap().join("palo-launcher").display().to_string().yellow());
        
        #[cfg(not(target_os = "windows"))]
        println!("\nDirectorio de config: {}", "~/.config/palo-launcher".yellow());
    }
    
    fn interactive_connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.firewall_ip.is_empty() {
            println!("{}", "⚠ Primero debes configurar la IP del firewall".red());
            println!("Ejecuta: {} config", "palo-launcher".cyan());
            return Ok(());
        }
        
        println!("{}", "=== Conexión a Palo Alto ===".green().bold());
        println!("Firewall: {}", self.config.firewall_ip.yellow());
        println!();
        
        let sections = vec![
            "Dashboard Principal",
            "Monitor",
            "Policies",
            "Objects",
            "Network",
            "Logs de Tráfico",
            "Configuración Personalizada",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Selecciona la sección a abrir")
            .items(&sections)
            .default(0)
            .interact()?;
        
        let url = match selection {
            0 => format!("https://{}", self.config.firewall_ip),
            1 => format!("https://{}/#monitor", self.config.firewall_ip),
            2 => format!("https://{}/#policies", self.config.firewall_ip),
            3 => format!("https://{}/#objects", self.config.firewall_ip),
            4 => format!("https://{}/#network", self.config.firewall_ip),
            5 => format!("https://{}/#monitor/traffic", self.config.firewall_ip),
            6 => {
                let custom: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Ingresa la ruta (ej: #monitor/system)")
                    .interact_text()?;
                format!("https://{}/{}", self.config.firewall_ip, custom)
            }
            _ => format!("https://{}", self.config.firewall_ip),
        };
        
        self.open_browsers(&url)?;
        
        Ok(())
    }
    
    fn open_browsers(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "Abriendo navegadores...".cyan());
        
        for (i, browser) in self.config.default_browsers.iter().enumerate() {
            match browser.as_str() {
                "firefox" | "firefox2" => {
                    let profile = if browser == "firefox2" {
                        format!("{}-2", self.config.firefox_profile)
                    } else {
                        self.config.firefox_profile.clone()
                    };
                    self.open_firefox(url, &profile)?;
                    println!("  {} Firefox (perfil: {})", "✓".green(), profile);
                }
                "brave" => {
                    self.open_brave(url, &self.config.brave_profile)?;
                    println!("  {} Brave (perfil: {})", "✓".green(), self.config.brave_profile);
                }
                "chrome" => {
                    self.open_chrome(url, &self.config.chrome_profile)?;
                    println!("  {} Chrome (perfil: {})", "✓".green(), self.config.chrome_profile);
                }
                "edge" => {
                    self.open_edge(url, &self.config.edge_profile)?;
                    println!("  {} Edge (perfil: {})", "✓".green(), self.config.edge_profile);
                }
                _ => {}
            }
            
            if i < self.config.default_browsers.len() - 1 {
                std::thread::sleep(std::time::Duration::from_millis(800));
            }
        }
        
        println!("\n{}", "¡Navegadores abiertos exitosamente!".green().bold());
        println!("{}", "Las sesiones se mantendrán en los perfiles específicos.".yellow());
        
        Ok(())
    }
    
    // ========== FIREFOX ==========
    #[cfg(target_os = "windows")]
    fn open_firefox(&self, url: &str, profile: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Rutas comunes de Firefox en Windows
        let userprofile_firefox = format!("{}\\AppData\\Local\\Mozilla Firefox\\firefox.exe", 
            std::env::var("USERPROFILE").unwrap_or_default());
        
        let firefox_paths = vec![
            "C:\\Program Files\\Mozilla Firefox\\firefox.exe",
            "C:\\Program Files (x86)\\Mozilla Firefox\\firefox.exe",
            userprofile_firefox.as_str(),
        ];
        
        for firefox_path in firefox_paths {
            if std::path::Path::new(&firefox_path).exists() {
                Command::new(&firefox_path)
                    .arg("-P")
                    .arg(profile)
                    .arg("--new-instance")
                    .arg(url)
                    .spawn()?;
                return Ok(());
            }
        }
        
        // Intenta con 'firefox' en PATH
        Command::new("firefox")
            .arg("-P")
            .arg(profile)
            .arg("--new-instance")
            .arg(url)
            .spawn()
            .map_err(|_| "Firefox no está instalado o no se encuentra en el PATH".into())
            .map(|_| ())
    }
    
    #[cfg(not(target_os = "windows"))]
    fn open_firefox(&self, url: &str, profile: &str) -> Result<(), Box<dyn std::error::Error>> {
        let firefox_cmd = if Command::new("firefox").arg("--version").output().is_ok() {
            "firefox"
        } else {
            "firefox-esr"
        };
        
        Command::new(firefox_cmd)
            .arg("-P")
            .arg(profile)
            .arg("--new-instance")
            .arg(url)
            .spawn()?;
        
        Ok(())
    }
    
    // ========== BRAVE ==========
    #[cfg(target_os = "windows")]
    fn open_brave(&self, url: &str, profile: &str) -> Result<(), Box<dyn std::error::Error>> {
        let userprofile_brave = format!("{}\\AppData\\Local\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
            std::env::var("USERPROFILE").unwrap_or_default());
        
        let brave_paths = vec![
            "C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
            "C:\\Program Files (x86)\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
            userprofile_brave.as_str(),
        ];
        
        for brave_path in brave_paths {
            if std::path::Path::new(&brave_path).exists() {
                let profile_dir = dirs::data_local_dir()
                    .ok_or("No se pudo obtener el directorio de datos")?
                    .join("BraveSoftware")
                    .join("Brave-Browser")
                    .join(profile);
                
                Command::new(&brave_path)
                    .arg(format!("--user-data-dir={}", profile_dir.display()))
                    .arg(url)
                    .spawn()?;
                return Ok(());
            }
        }
        
        Err("Brave no está instalado o no se encuentra".into())
    }
    
    #[cfg(not(target_os = "windows"))]
    fn open_brave(&self, url: &str, profile: &str) -> Result<(), Box<dyn std::error::Error>> {
        let brave_paths = vec!["brave-browser", "brave", "/usr/bin/brave-browser", "/usr/bin/brave"];
        
        for brave_cmd in brave_paths {
            if Command::new(brave_cmd).arg("--version").output().is_ok() {
                let profile_dir = dirs::home_dir()
                    .ok_or("No se pudo obtener el directorio home")?
                    .join(".config")
                    .join("BraveSoftware")
                    .join("Brave-Browser")
                    .join(profile);
                
                Command::new(brave_cmd)
                    .arg(format!("--user-data-dir={}", profile_dir.display()))
                    .arg(url)
                    .spawn()?;
                
                return Ok(());
            }
        }
        
        Err("Brave no está instalado o no se encuentra en el PATH".into())
    }
    
    // ========== CHROME ==========
    #[cfg(target_os = "windows")]
    fn open_chrome(&self, url: &str, profile: &str) -> Result<(), Box<dyn std::error::Error>> {
        let userprofile_chrome = format!("{}\\AppData\\Local\\Google\\Chrome\\Application\\chrome.exe",
            std::env::var("USERPROFILE").unwrap_or_default());
        
        let chrome_paths = vec![
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
            "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
            userprofile_chrome.as_str(),
        ];
        
        for chrome_path in chrome_paths {
            if std::path::Path::new(&chrome_path).exists() {
                let profile_dir = dirs::data_local_dir()
                    .ok_or("No se pudo obtener el directorio de datos")?
                    .join("Google")
                    .join("Chrome")
                    .join(profile);
                
                Command::new(&chrome_path)
                    .arg(format!("--user-data-dir={}", profile_dir.display()))
                    .arg(url)
                    .spawn()?;
                return Ok(());
            }
        }
        
        Err("Chrome no está instalado o no se encuentra".into())
    }
    
    #[cfg(not(target_os = "windows"))]
    fn open_chrome(&self, url: &str, profile: &str) -> Result<(), Box<dyn std::error::Error>> {
        let chrome_paths = vec!["google-chrome", "google-chrome-stable", "chromium", "chromium-browser"];
        
        for chrome_cmd in chrome_paths {
            if Command::new(chrome_cmd).arg("--version").output().is_ok() {
                let profile_dir = dirs::home_dir()
                    .ok_or("No se pudo obtener el directorio home")?
                    .join(".config")
                    .join("google-chrome")
                    .join(profile);
                
                Command::new(chrome_cmd)
                    .arg(format!("--user-data-dir={}", profile_dir.display()))
                    .arg(url)
                    .spawn()?;
                
                return Ok(());
            }
        }
        
        Err("Chrome/Chromium no está instalado o no se encuentra en el PATH".into())
    }
    
    // ========== EDGE (solo Windows) ==========
    #[cfg(target_os = "windows")]
    fn open_edge(&self, url: &str, profile: &str) -> Result<(), Box<dyn std::error::Error>> {
        let edge_paths = vec![
            "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
            "C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe",
        ];
        
        for edge_path in edge_paths {
            if std::path::Path::new(&edge_path).exists() {
                let profile_dir = dirs::data_local_dir()
                    .ok_or("No se pudo obtener el directorio de datos")?
                    .join("Microsoft")
                    .join("Edge")
                    .join(profile);
                
                Command::new(&edge_path)
                    .arg(format!("--user-data-dir={}", profile_dir.display()))
                    .arg(url)
                    .spawn()?;
                return Ok(());
            }
        }
        
        Err("Edge no está instalado o no se encuentra".into())
    }
    
    #[cfg(not(target_os = "windows"))]
    fn open_edge(&self, _url: &str, _profile: &str) -> Result<(), Box<dyn std::error::Error>> {
        Err("Edge solo está disponible en Windows".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut launcher = PaloLauncher::new()?;
    
    match cli.command {
        Some(Commands::Config) => {
            launcher.configure()?;
        }
        Some(Commands::Show) => {
            launcher.show_config();
        }
        Some(Commands::Connect) => {
            launcher.interactive_connect()?;
        }
        None => {
            launcher.interactive_connect()?;
        }
    }
    
    Ok(())
}
