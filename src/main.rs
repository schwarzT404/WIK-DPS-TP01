use std::collections::HashMap;
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // Chargement des variables d'environnement (optionnel)
    dotenv::dotenv().ok();
    
    // Récupération du port depuis la variable d'environnement PING_LISTEN_PORT
    // ou utilisation du port 8080 par défaut
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    
    // Utilisation de 127.0.0.1 (loopback) pour éviter les problèmes de connectivité
    let addr = format!("127.0.0.1:{}", port);
    
    // Affichage des informations de démarrage
    println!("Démarrage du serveur sur {}...", addr);
    
    // Création du serveur TCP
    // bind() peut échouer si le port est déjà utilisé, d'où le expect()
    let listener = TcpListener::bind(&addr)
        .expect("Impossible de démarrer le serveur. Le port est peut-être déjà utilisé.");
    
    println!("Serveur démarré! En attente de connexions...");

    // Boucle principale: traitement de chaque connexion entrante
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Traitement de la connexion
                handle_connection(stream);
            }
            Err(e) => {
                // Journalisation des erreurs
                eprintln!("Erreur lors de la connexion entrante: {}", e);
            }
        }
    }
}

// Fonction qui traite une connexion TCP
fn handle_connection(mut stream: TcpStream) {
    // Buffer pour stocker les données reçues
    let mut buffer = [0; 1024];
    
    // Lecture des données de la requête HTTP
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Conversion des bytes en chaîne de caractères
            let request = String::from_utf8_lossy(&buffer[..]);
            
            // Extraction de la première ligne pour analyser la méthode et le chemin
            let request_line = request.lines().next().unwrap_or("");
            let parts: Vec<&str> = request_line.split_whitespace().collect();
            
            // Vérification si la requête est GET /ping
            if parts.len() >= 2 && parts[0] == "GET" && parts[1] == "/ping" {
                // Extraction des en-têtes HTTP de la requête
                let headers = parse_headers(&request);
                
                // Conversion des en-têtes en JSON
                let json = serde_json::to_string(&headers).unwrap_or_else(|_| "{}".to_string());
                
                // Construction de la réponse HTTP 200 OK avec le JSON
                let response = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Type: application/json\r\n\
                    Content-Length: {}\r\n\
                    Connection: close\r\n\
                    \r\n\
                    {}",
                    json.len(),
                    json
                );
                
                // Envoi de la réponse
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Erreur lors de l'envoi de la réponse: {}", e);
                }
            } else {
                // Pour toute autre requête, réponse 404 Not Found vide
                let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Erreur lors de l'envoi de la réponse 404: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Erreur lors de la lecture de la requête: {}", e);
        }
    }
}

// Fonction qui extrait les en-têtes HTTP d'une requête
fn parse_headers(request: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    
    // Parcours des lignes de la requête (en sautant la première ligne)
    for line in request.lines().skip(1) {
        // Les en-têtes se terminent par une ligne vide
        if line.is_empty() {
            break;
        }
        
        // Recherche du séparateur ':'
        if let Some(idx) = line.find(':') {
            // Extraction de la clé et de la valeur
            let key = line[..idx].trim();
            let value = line[(idx + 1)..].trim();
            
            // Ajout de l'en-tête au HashMap
            headers.insert(key.to_string(), value.to_string());
        }
    }
    
    headers
}