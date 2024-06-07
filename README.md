# RUST CLEANER

`RUST CLEANER` est un outil CLI (interface en ligne de commande) écrit en Rust, conçu pour scanner, nettoyer et vider les fichiers inutiles sur votre système. Il prend en charge macOS, Linux et Windows, et offre des fonctionnalités pour gérer les dossiers de téléchargements et la poubelle.

## Fonctionnalités

- **Scan des fichiers inutiles** : Scanne les répertoires spécifiés pour identifier les fichiers temporaires, journaux, anciens fichiers et fichiers de sauvegarde.
- **Nettoyage des fichiers inutiles** : Supprime les fichiers inutiles identifiés lors du scan.
- **Vider le dossier Téléchargements** : Supprime tous les fichiers du dossier Téléchargements.
- **Vider la poubelle** : Supprime tous les fichiers de la poubelle.
- **Barre de progression stylisée** : Affiche une barre de progression avec des pourcentages et des couleurs pour un suivi visuel des opérations.
- **Rapport de nettoyage** : Génère un rapport après chaque opération de nettoyage.
- **Planificateur de nettoyage** : Permet de planifier des nettoyages automatiques.
- **Analyse des fichiers en double** : Scanne le système pour les fichiers en double.
- **Exclusion de types de fichiers** : Permet d'exclure certains types de fichiers du nettoyage.
- **Support pour les répertoires personnalisés** : Permet de spécifier des répertoires supplémentaires ou spécifiques à nettoyer.
- **Mode interactif** : Offre un mode interactif pour la suppression des fichiers.
- **Nettoyage des applications spécifiques** : Nettoie les fichiers de cache des applications spécifiques.
- **Restauration des fichiers supprimés** : Déplace les fichiers supprimés dans une corbeille spécifique avant de les supprimer définitivement, avec la possibilité de les restaurer.
- **Nettoyage sécurisé** : Supprime les fichiers de manière sécurisée.
- **Nettoyage basé sur l'âge des fichiers** : Supprime des fichiers en fonction de leur âge.

## Prérequis

Assurez-vous que vous avez [Rust](https://www.rust-lang.org/tools/install) installé sur votre machine.

## Installation

Clonez le dépôt et compilez le projet avec Cargo :

```sh
git clone <URL_DU_DÉPÔT>
cd <NOM_DU_RÉPÔT>
cargo 
```

## Utilisation
### Scanner les fichiers inutiles

Cette commande scanne les répertoires spécifiés pour les fichiers inutiles et affiche les résultats.

```sh
cargo run -- --scan
```

### Nettoyer les fichiers inutiles
Cette commande scanne et supprime les fichiers inutiles des répertoires spécifiés après confirmation de l'utilisateur.

```sh
cargo run -- --clean
```

### Vider le dossier Téléchargements
Cette commande supprime tous les fichiers du dossier Téléchargements après confirmation de l'utilisateur.

```sh
cargo run -- --clear-downloads
```

### Vider la poubelle
Cette commande supprime tous les fichiers de la poubelle après confirmation de l'utilisateur.

```sh
cargo run -- --clear-trash
```
### Générer un rapport de nettoyage
Cette commande génère un rapport détaillé après l'opération de nettoyage.

La commande `--report` doit être utilisée en conjonction avec les commandes qui génèrent un rapport, comme `--scan` ou `--clean`.

`cargo run -- --report` affichera un message d'erreur si utilisé seul. Utilisez `cargo run -- --scan --report` ou `cargo run -- --clean --report`
```sh
cargo run -- --scan --report 
```
ou

```sh
cargo run -- --clean --report
```
### Planifier un nettoyage automatique
Les commandes `cargo run -- --schedule daily` ou `cargo run -- --schedule weekly` permettent de planifier des nettoyages automatiques à des intervalles réguliers.

utiliser cargo run -- --schedule daily ou cargo run -- --schedule weekly pour planifier des nettoyages automatiques. Le programme restera en cours d'exécution pour exécuter les tâches planifiées aux moments spécifiés.

```sh
cargo run -- --schedule daily
```
Exemple d'utilisation :  

- `daily`: Si vous souhaitez que le nettoyage automatique se fasse quotidiennement à minuit.
```sh
Enter the schedule (daily, weekly): daily
```

- `weekly`: Si vous préférez que le nettoyage automatique se fasse chaque semaine, le dimanche à minuit.
```sh
Enter the schedule (daily, weekly): weekly
```

```sh
cargo run -- --schedule weekly
```

### Analyser les fichiers en double
Cette commande scanne le système pour les fichiers en double et offre une option pour les supprimer.

```sh
cargo run -- --duplicates
```

### Exclure des types de fichiers du nettoyage
Cette commande permet d'exclure certains types de fichiers du nettoyage.

```sh
cargo run -- --exclude-type log tmp
```

### Nettoyer des répertoires personnalisés
Cette commande permet de spécifier des répertoires supplémentaires ou spécifiques à nettoyer.

```sh
cargo run -- --dir /path/to/custom/dir
```

### Utiliser le mode interactif
Cette commande offre un mode interactif pour la suppression des fichiers.

```sh
cargo run -- --interactive
```

### Nettoyer les fichiers de cache des applications spécifiques
Cette commande nettoie les fichiers de cache des applications spécifiques.

```sh
cargo run -- --clean-browser
```

### Restaurer des fichiers supprimés
Cette commande déplace les fichiers supprimés dans une corbeille spécifique avant de les supprimer définitivement, avec la possibilité de les restaurer.

```sh
cargo run -- --restore
```

### Nettoyer des fichiers de manière sécurisée
Cette commande supprime les fichiers de manière sécurisée, en écrasant les données.

```sh
cargo run -- --secure-clean
```

### Nettoyer des fichiers en fonction de leur âge
Cette commande supprime des fichiers qui n'ont pas été modifiés depuis une certaine période.

```sh
cargo run -- --age 30
```










