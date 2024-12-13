use std::env;
use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::collections::BTreeMap;
use trust_dns_resolver::config::*;
use trust_dns_resolver::proto::rr::Name;
use trust_dns_resolver::TokioAsyncResolver;
use walkdir::WalkDir;

#[tokio::main]
async fn main() {
    // Lade Umgebungsvariablen
    let conf_dir = "/conf";

    let hosts_env = env::var("DNS_HOSTS").expect("DNS_HOSTS environment variable is required");
    let hosts: Vec<&str> = hosts_env.split(',').collect();

    let interval_env = env::var("CHECK_INTERVAL").expect("CHECK_INTERVAL environment variable is required");
    let interval = interval_env.parse::<u64>().expect("Failed to parse CHECK_INTERVAL");

    // DNS-Resolver mit 1.1.1.1 und ohne Caching konfigurieren
    let resolver_config = ResolverConfig::cloudflare(); // Nutze Cloudflare (1.1.1.1)
    let mut resolver_opts = ResolverOpts::default();
    resolver_opts.cache_size = 0; // Deaktiviere das Caching

    // Der Resolver wird direkt erstellt, kein Result zu handhaben
    let resolver = TokioAsyncResolver::tokio(resolver_config, resolver_opts);

    // Initialisiere die IP-Liste (BTreeMap für geordnete Speicherung)
    let mut ip_map = BTreeMap::new();

    loop {
        let mut any_change = false;
        let mut new_ip_map = BTreeMap::new(); // Verwende BTreeMap für konsistente Sortierung

        // IPs der Hosts abfragen
        for host in &hosts {
            // Konvertiere den Hostnamen in den Typ `Name`
            let name = Name::from_ascii(host).expect("Invalid hostname");

            match resolver.lookup_ip(name).await {
                Ok(lookup) => {
                    let ip = lookup.iter().next().unwrap().to_string();
                    new_ip_map.insert(host.to_string(), ip.clone());

                    // Prüfen, ob sich die IP geändert hat (unabhängig von der Reihenfolge)
                    if ip_map.get(*host) != Some(&ip) {
                        any_change = true;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to lookup host {}: {:?}", host, e);
                }
            }
        }

        // Wenn sich IPs geändert haben, Dateien im Verzeichnis rekursiv aktualisieren
        if any_change {
            println!("IP change detected, updating files...");

            // Verwende WalkDir, um rekursiv durch das Verzeichnis und alle Unterverzeichnisse zu gehen
            for entry in WalkDir::new(conf_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    update_file_with_ips(path, &new_ip_map);
                }
            }

            // Aktualisiere das gespeicherte IP-Map
            ip_map = new_ip_map;
        }

        // Pause bis zur nächsten Überprüfung
        sleep(Duration::from_secs(interval));
    }
}

// Funktion zum Aktualisieren der Dateien mit den neuen IPs
fn update_file_with_ips(file_path: &Path, ip_map: &BTreeMap<String, String>) {
    // Dateiinhalt lesen
    let file = OpenOptions::new().read(true).write(true).open(file_path);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file {:?}: {:?}", file_path, e);
            return;
        }
    };

    let reader = BufReader::new(&file);
    let mut new_lines = Vec::new();
    let mut line_changed = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains("@allowedClients") && line.contains("remote_ip") {
            // Aktualisiere @allowedClients mit neuen IPs
            let new_ips: Vec<String> = ip_map.values().cloned().collect();
            let new_line = format!("@allowedClients remote_ip {}", new_ips.join(" "));
            new_lines.push(new_line);
            line_changed = true;
        } else if line.contains("@disallowedClients") && line.contains("not remote_ip") {
            // Aktualisiere @disallowedClients mit neuen IPs
            let new_ips: Vec<String> = ip_map.values().cloned().map(|ip| format!("not {}", ip)).collect();
            let new_line = format!("@disallowedClients {}", new_ips.join(" "));
            new_lines.push(new_line);
            line_changed = true;
        } else {
            // Behalte alle anderen Zeilen unverändert
            new_lines.push(line);
        }
    }

    // Datei nur überschreiben, wenn sich etwas geändert hat
    if line_changed {
        println!("Updated file {:?} with new IPs: {:?}", file_path, ip_map);
        let mut writer = OpenOptions::new().write(true).truncate(true).open(file_path).unwrap();
        for line in new_lines {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}
