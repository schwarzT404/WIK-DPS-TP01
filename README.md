# WIK-DPS-TP01

API HTTP en Rust qui retourne les headers de la requête au format JSON pour les requêtes GET sur `/ping`.

## Fonctionnalités

- Écoute sur un port configurable via la variable d'environnement `PING_LISTEN_PORT` (8080 par défaut)
- Retourne les headers HTTP au format JSON pour les requêtes GET sur `/ping`
- Retourne une réponse vide avec code 404 pour toute autre requête

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (version 1.58.0 ou supérieure)
- Toolchain GNU pour Windows (`rustup default stable-x86_64-pc-windows-gnu`)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (installé avec Rust)

## Installation et configuration

1. Installer Rust
```bash
# Télécharger et exécuter rustup-init depuis https://rustup.rs/
```

2. Configurer Rust avec la toolchain GNU (important pour Windows)
```bash
rustup default stable-x86_64-pc-windows-gnu
```

3. Cloner le dépôt
```bash
git clone https://github.com/votre-username/wik-dps-tp01.git
cd wik-dps-tp01
```

4. Compiler le projet
```bash
cargo build --release
```

L'exécutable se trouve dans le dossier `target/release/`.

## Exécution

### Avec le port par défaut (8080)

```bash
cargo run --release
```

Ou directement avec l'exécutable :

```bash
./target/release/wik-dps-tp01.exe
```

### Avec un port personnalisé

Sous Linux/macOS:
```bash
PING_LISTEN_PORT=3000 cargo run --release
```

Sous Windows (PowerShell):
```powershell
$env:PING_LISTEN_PORT=3333; cargo run --release
```

**Note**: Utilisez l'adresse `127.0.0.1` plutôt que `localhost` pour les tests. Par exemple:
```powershell
curl -X GET http://127.0.0.1:3333/ping
```

## Test de l'API

Une fois le serveur démarré, vous pouvez tester l'API avec curl :

```bash
curl -X GET http://127.0.0.1:3333/ping
```

Vous devriez recevoir une réponse au format JSON contenant les headers de votre requête, comme:
```json
{"Host":"127.0.0.1:3333","User-Agent":"curl/8.10.1","Accept":"*/*"}
```

Pour toute autre requête, par exemple :

```bash
curl -X GET http://127.0.0.1:3333/autre
```

Vous recevrez une réponse vide avec le code d'état 404.

## Résolution de problèmes

Si vous rencontrez des erreurs de compilation:

1. **Erreur de linker sur Windows**
   - Assurez-vous d'utiliser la toolchain GNU et non MSVC
   - Exécutez `rustup default stable-x86_64-pc-windows-gnu`

2. **Problèmes de connexion au serveur**
   - Vérifiez qu'aucun pare-feu ne bloque le port utilisé
   - Utilisez `127.0.0.1` au lieu de `localhost`
   - Essayez un port différent (comme 3333) avec la variable d'environnement

## Structure du projet

```
wik-dps-tp01/
├── Cargo.toml        # Fichier de configuration du projet et dépendances
├── src/
│   └── main.rs       # Point d'entrée de l'application
└── README.md         # Documentation du projet
```

## Explications du code

Le code source est commenté pour expliquer le fonctionnement de chaque partie :

1. Configuration et démarrage du serveur
2. Traitement des connexions entrantes
3. Analyse des requêtes HTTP
4. Extraction des headers
5. Génération des réponses au format JSON