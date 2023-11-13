# DevOps Rust Project - TP1

## Description

Ce projet Rust a pour objectif de fournir un serveur HTTP simple avec une API qui retourne les headers de la requête au format JSON lorsqu'une requête HTTP GET est effectuée sur `/ping`. Le serveur écoute sur un port configurable via la variable d'environnement `PING_LISTEN_PORT` ou par défaut sur le port 8080.

## Fonctionnalités

- **Endpoint `/ping`:** Lorsqu'une requête HTTP GET est effectuée sur `/ping`, le serveur renvoie les headers de la requête au format JSON. Pour tout autre requête, le serveur renvoie une erreur 404.

- **Port configurable:** Le serveur écoute sur le port spécifié par la variable d'environnement `PING_LISTEN_PORT`. Si cette variable n'est pas définie, le serveur utilise le port par défaut 8080.

## Environnement de développement

- **Langage:** Rust
- **Gestionnaire de paquets:** Cargo
- **Outils:** 
<span style="color:green">**Actix-web**</span> pour le développement du serveur HTTP, 
<span style="color:green">**Serde**</span> pour la sérialisation/désérialisation JSON, 
<span style="color:green">**Dotenv**</span> pour la gestion des variables d'environnement
- **Dépendances:** Voir `Cargo.toml` pour la liste complète

## Configuration

Le port sur lequel le serveur écoute peut être configuré via la variable d'environnement `PING_LISTEN_PORT`. Si cette variable n'est pas définie, le serveur utilise le port par défaut 8080.

Exemple de configuration via la variable d'environnement via le fichier `.env`:

```bash
PING_LISTEN_PORT=8081
```

## Installation 
### 1. Cloner le projet

```bash
git clone https://github.com/ratpiGene/devops
```
Il ne reste plus qu'à accéder au dossier devops (via VSCode) 

```bash
cd devops
```
### 2. Construire et run le projet avec Cargo

```bash
cargo build
# une fois que le build est terminé
cargo run
```
Le serveur devrait être accessible à l'adresse http://127.0.0.1:8080 (ou sur le port spécifié par **PING_LISTEN_PORT**).