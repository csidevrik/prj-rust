use std::env;
use std::fs;
use std::process::Command;
use std::path::Path;
use std::io::{self, Write};

fn main() {
    // Verificar el sistema operativo
    let os = env::consts::OS;
    println!("Sistema operativo detectado: {}", os);

    if os == "linux" {
        // Definir el directorio
        let onedrive_dir = "/home/$USER/OneDriveP";

        // Comprobar si el directorio existe
        if !Path::new(onedrive_dir).exists() {
            // Crear el directorio con permisos de lectura y escritura
            match fs::create_dir(onedrive_dir) {
                Ok(_) => {
                    println!("Directorio {} creado con éxito.", onedrive_dir);
                    // Cambiar permisos del directorio
                    Command::new("chmod")
                        .arg("u+rw")
                        .arg(onedrive_dir)
                        .output()
                        .expect("Fallo al cambiar permisos del directorio");
                },
                Err(e) => println!("Error al crear el directorio: {}", e),
            }
        } else {
            println!("El directorio {} ya existe.", onedrive_dir);
        }

        // Verificar si rclone está instalado
        let rclone_check = Command::new("which")
            .arg("rclone")
            .output()
            .expect("Fallo al ejecutar el comando");

        if rclone_check.stdout.is_empty() {
            println!("rclone no está instalado. Por favor, instálalo y vuelve a ejecutar el programa.");
            return;
        } else {
            println!("rclone está instalado.");
        }

        // Montar OneDriveP usando rclone
        let mount_command = format!(
            "nohup rclone --vfs-cache-mode writes mount OneDriveP: ~/OneDriveP > /dev/null 2>&1 &"
        );

        match Command::new("sh")
        .arg("-c")
        .arg(&mount_command)
        .spawn() {
            Ok(_) => println!("OneDrive montado en segundo plano."),
            Err(e) => println!("Error al montar OneDrive: {}", e),
        }
    } else {
        println!("Este programa solo está diseñado para Linux.");
    }
}
