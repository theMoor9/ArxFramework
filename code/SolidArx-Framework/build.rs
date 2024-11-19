use std::process::Command;
fn main() {
    // Verifica se Python è installato
    let python_check = Command::new("python")
        .arg("--version")
        .output();

    match python_check {
        Ok(output) => {
            println!("Python trovato: {}", String::from_utf8_lossy(&output.stdout));
        }
        Err(_) => {
            // Se Python non è installato, prova ad installarlo
            println!("Python non trovato. Tentando di installare...");
            
            #[cfg(target_os = "linux")]
            {
                // Linux 
                let install = Command::new("sudo")
                    .arg("apt-get")
                    .arg("install")
                    .arg("-y")
                    .arg("python3")
                    .output();
                
                match install {
                    Ok(_) => println!("Python installato correttamente."),
                    Err(e) => println!("Errore durante l'installazione di Python: {}", e),
                }
            }

            #[cfg(target_os = "macos")]
            {
                // MacOS 
                let install = Command::new("brew")
                    .arg("install")
                    .arg("python")
                    .output();
                
                match install {
                    Ok(_) => println!("Python installato correttamente."),
                    Err(e) => println!("Errore durante l'installazione di Python: {}", e),
                }
            }
            #[cfg(target_os = "windows")]
            {
                // Windows 
                let install = Command::new("choco")
                    .arg("install")
                    .arg("-y")
                    .arg("python")
                    .output();
                
                match install {
                    Ok(_) => println!("Python installato correttamente."),
                    Err(e) => println!("Errore durante l'installazione di Python: {}", e),
                }
            }
        }
    }

    // Notifica il compilatore Rust che c'è una dipendenza nativa (Python) da trovare
    println!("cargo:rerun-if-changed=build.rs");
}
