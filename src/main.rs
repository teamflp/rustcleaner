use dialoguer::Input;
use clap::{Arg, Command};
use dialoguer::{Select, Confirm};
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::{SystemTime, Duration};
use tokio::task;
use tokio::sync::mpsc;
use std::str::FromStr;
use cron::Schedule;
use dirs;
use chrono::Utc;
use terminal_size::{Width, terminal_size};

mod cleaner;

/// Obtient les répertoires exclus par défaut en fonction du système d'exploitation.
fn get_default_excludes() -> HashSet<String> {
    #[cfg(target_os = "windows")]
    {
        vec![
            r"C:\ProgramData".to_string(),
            r"C:\Windows".to_string(),
        ].into_iter().collect()
    }
    #[cfg(target_os = "macos")]
    {
        vec![
            "/Users/user/Library/Caches/com.apple.HomeKit".to_string(),
            "/Users/user/Library/Caches/CloudKit".to_string(),
            "/Users/user/Library/Caches/com.apple.Safari".to_string(),
            "/Users/user/Library/Caches/com.apple.containermanagerd".to_string(),
            "/Users/user/Library/Caches/com.apple.Safari.SafeBrowsing".to_string(),
            "/Users/user/Library/Caches/FamilyCircle".to_string(),
            "/Users/user/Library/Caches/com.apple.homed".to_string(),
            "/Users/user/Library/Caches/com.apple.findmy.fmipcore".to_string(),
            "/Users/user/Library/Caches/com.apple.ap.adprivacyd".to_string(),
            "/Users/user/Library/Caches/com.apple.fmfcore".to_string(),
        ].into_iter().collect()
    }
    #[cfg(target_os = "linux")]
    {
        vec![
            "/home/user/.cache".to_string(),
            "/home/user/.local/share/Trash".to_string(),
        ].into_iter().collect()
    }
}

/// Exécute une tâche de nettoyage planifiée en fonction de l'expression cron fournie.
async fn run_scheduled_job(schedule_expression: &str, dirs_to_scan: Vec<PathBuf>, exclude_dirs: HashSet<String>, exclude_types: HashSet<String>, secure_clean: bool, mut shutdown: mpsc::Receiver<()>) {
    let schedule = Schedule::from_str(schedule_expression).unwrap();
    let mut upcoming = schedule.upcoming(Utc);

    while let Some(next) = upcoming.next() {
        let now = Utc::now();
        let duration = next - now;
        let delay = duration.to_std().unwrap_or(Duration::from_secs(60));

        tokio::select! {
            _ = tokio::time::sleep(delay) => {
                println!("Running scheduled cleaning job...");
                let files_to_clean = cleaner::scan_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

                if !files_to_clean.is_empty() {
                    cleaner::clean_files(files_to_clean, secure_clean);
                } else {
                    println!("No files to clean.");
                }
            }
            _ = shutdown.recv() => {
                println!("Shutting down scheduled job...");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Sélection de la langue
    let lang = Select::new()
        .with_prompt("Choose your language / Choisissez votre langue")
        .item("English")
        .item("Français")
        .default(0)
        .interact()
        .unwrap();

    let lang = if lang == 0 { "en" } else { "fr" };

    println!();
    println!("==========================================================");
    println!("{}", match lang {
        "en" => "|                 Welcome to Cleaner Tool                |",
        "fr" => "|                Bienvenue à Cleaner Tool                |",
        _ => "|                 Welcome to Cleaner Tool                |",
    });
    println!("==========================================================");
    println!();

    loop {
        // Définition des options en fonction de la langue
        let options = match lang {
            "en" => &[
                "1 => Scan directories for unnecessary files",
                "2 => Clean unnecessary files",
                "3 => Clear the Downloads folder",
                "4 => Clear the Trash",
                "5 => Generate a report after cleaning",
                "6 => Schedule an automatic cleaning (every x hours)",
                "7 => Analyze and remove duplicate files",
                "8 => Interactive mode for file deletion",
                "9 => Clean browser cache files",
                "10 => Restore deleted files",
                "11 => Secure file cleaning",
                "12 => Clean files older than a specified number of days",
                "q => Enter q to quit"
            ],
            "fr" => &[
                "1 => Scanner les répertoires pour les fichiers inutiles",
                "2 => Nettoyer les fichiers inutiles",
                "3 => Vider le dossier Téléchargements",
                "4 => Vider la corbeille",
                "5 => Générer un rapport après le nettoyage",
                "6 => Planifier un nettoyage automatique (toutes les x heures)",
                "7 => Analyser et supprimer les fichiers en double",
                "8 => Mode interactif pour la suppression de fichiers",
                "9 => Nettoyer les fichiers de cache du navigateur",
                "10 => Restaurer les fichiers supprimés",
                "11 => Nettoyage sécurisé des fichiers",
                "12 => Nettoyer les fichiers plus anciens qu'un nombre de jours spécifié",
                "q => Entrer q pour quitter"
            ],
            _ => &[
                "1 => Scan directories for unnecessary files",
                "2 => Clean unnecessary files",
                "3 => Clear the Downloads folder",
                "4 => Clear the Trash",
                "5 => Generate a report after cleaning",
                "6 => Schedule an automatic cleaning (every x hours)",
                "7 => Analyze and remove duplicate files",
                "8 => Interactive mode for file deletion",
                "9 => Clean browser cache files",
                "10 => Restore deleted files",
                "11 => Secure file cleaning",
                "12 => Clean files older than a specified number of days",
                "q => Enter q to quit"
            ]
        };

        // Sélection de l'action
        let selection = Select::new()
            .with_prompt(match lang {
                "en" => "Choose an action",
                "fr" => "Choisissez une action",
                _ => "Choose an action",
            })
            .items(options)
            .default(0)
            .interact()
            .unwrap();
        println!();
        println!();

        // Gestion de la sortie
        if options[selection] == "q => Enter q to quit" || options[selection] == "q => Entrer q pour quitter" {
            let proceed = Confirm::new()
                .with_prompt(match lang {
                    "en" => "Are you sure you want to quit?",
                    "fr" => "Êtes-vous sûr de vouloir quitter?",
                    _ => "Are you sure you want to quit?",
                })
                .interact()
                .unwrap();

            if proceed {
                println!("{}",match lang {
                    "en" => "Exiting...",
                    "fr" => "Sortie...",
                    _ => "Exiting...",
                });
                return;
            } else {
                println!("{}",match lang {
                    "en" => "Continuing...",
                    "fr" => "Continuer...",
                    _ => "Continuing...",
                });
                continue; // Revenir au menu principal
            }
        }

        // Configuration des arguments de la ligne de commande
        let matches = Command::new(match lang {
            "en" => "Cleaner Tool",
            "fr" => "Outil de Nettoyage",
            _ => "Cleaner Tool",
        })
            .version("1.0")
            .author("Paterne G. G. it.devwebm@gmail.com")
            .about(match lang {
                "en" => "Cleans unnecessary files from your system",
                "fr" => "Nettoie les fichiers inutiles de votre système",
                _ => "Cleans unnecessary files from your system",
            })
            .arg(Arg::new("scan")
                .short('s')
                .long("scan")
                .help(match lang {
                    "en" => "Scans directories for unnecessary files",
                    "fr" => "Scanne les répertoires pour les fichiers inutiles",
                    _ => "Scans directories for unnecessary files",
                })
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("clean")
                .short('c')
                .long("clean")
                .help(match lang {
                    "en" => "Cleans unnecessary files",
                    "fr" => "Nettoie les fichiers inutiles",
                    _ => "Cleans unnecessary files",
                })
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("clear-downloads")
                .short('d')
                .long("clear-downloads")
                .help(match lang {
                    "en" => "Clears the Downloads folder",
                    "fr" => "Vider le dossier Téléchargements",
                    _ => "Clears the Downloads folder",
                })
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("clear-trash")
                .short('t')
                .long("clear-trash")
                .help(match lang {
                    "en" => "Clears the Trash",
                    "fr" => "Vider la corbeille",
                    _ => "Clears the Trash",
                })
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("report")
                .short('r')
                .long("report")
                .help(match lang {
                    "en" => "Generates a report after cleaning",
                    "fr" => "Génère un rapport après le nettoyage",
                    _ => "Generates a report after cleaning",
                })
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("schedule")
                .long("schedule")
                .value_name("SCHEDULE")
                .help(match lang {
                    "en" => "Schedules an automatic cleaning (every x hours)",
                    "fr" => "Planifie un nettoyage automatique (toutes les x heures)",
                    _ => "Schedules an automatic cleaning (every x hours)",
                })
                .action(clap::ArgAction::Set))
            .arg(Arg::new("duplicates")
                .long("duplicates")
                .help(match lang {
                    "en" => "Analyzes and removes duplicate files",
                    "fr" => "Analyser et supprimer les fichiers en double",
                    _ => "Analyzes and removes duplicate files",
                })
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("exclude")
                .short('e')
                .long("exclude")
                .value_name("DIR")
                .num_args(1..)
                .help(match lang {
                    "en" => "Excludes specified directories from the scan",
                    "fr" => "Exclut les répertoires spécifiés de l'analyse",
                    _ => "Excludes specified directories from the scan",
                }))
            .arg(Arg::new("exclude-type")
                .long("exclude-type")
                .value_name("EXTENSIONS")
                .num_args(1..)
                .help(match lang {
                    "en" => "Excludes specified file types from the scan",
                    "fr" => "Exclut les types de fichiers spécifiés de l'analyse",
                    _ => "Excludes specified file types from the scan",
                }))
            .arg(Arg::new("dir")
                .long("dir")
                .value_name("DIRECTORY")
                .num_args(1..)
                .help(match lang {
                    "en" => "Adds custom directories to clean",
                    "fr" => "Ajoute des répertoires personnalisés à nettoyer",
                    _ => "Adds custom directories to clean",
                }))
            .arg(Arg::new("interactive")
                .long("interactive")
                .help(match lang {
                    "en" => "Interactive mode for file deletion",
                    "fr" => "Mode interactif pour la suppression de fichiers",
                    _ => "Interactive mode for file deletion",
                })
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("clean-browser")
                .long("clean-browser")
                .help(match lang {
                    "en" => "Cleans browser cache files",
                    "fr" => "Nettoyer les fichiers de cache du navigateur",
                    _ => "Cleans browser cache files",
                })
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("restore")
                .long("restore")
                .help(match lang {
                    "en" => "Restores deleted files",
                    "fr" => "Restaurer les fichiers supprimés",
                    _ => "Restores deleted files",
                })
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("secure-clean")
                .long("secure-clean")
                .help(match lang {
                    "en" => "Secure file cleaning",
                    "fr" => "Nettoyage sécurisé des fichiers",
                    _ => "Secure file cleaning",
                })
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("age")
                .long("age")
                .value_name("DAYS")
                .help(match lang {
                    "en" => "Cleans files older than the specified number of days",
                    "fr" => "Nettoyer les fichiers plus anciens qu'un nombre de jours spécifié",
                    _ => "Cleans files older than the specified number of days",
                })
                .action(clap::ArgAction::Set))
            .get_matches();

        // Collecte des répertoires à exclure
        let mut exclude_dirs: HashSet<String> = matches.get_many::<String>("exclude")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect();
        exclude_dirs.extend(get_default_excludes());

        // Collecte des types de fichiers à exclure
        let exclude_types: HashSet<String> = matches.get_many::<String>("exclude-type")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect();

        // Collecte des répertoires à analyser
        let mut dirs_to_scan: Vec<PathBuf> = matches.get_many::<String>("dir")
            .unwrap_or_default()
            .map(PathBuf::from)
            .collect();

        #[cfg(target_os = "windows")]
        dirs_to_scan.push(dirs::download_dir().unwrap());
        #[cfg(not(target_os = "windows"))]
        dirs_to_scan.push(dirs::home_dir().unwrap().join("Downloads"));
        dirs_to_scan.push(dirs::cache_dir().unwrap());

        let start_time = SystemTime::now();

        let _terminal_width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            100 // Largeur par défaut si la taille du terminal n'est pas détectée
        };

        match selection {
            // Scanner les répertoires pour les fichiers inutiles
            0 => {
                let files_to_clean = cleaner::scan_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

                if !files_to_clean.is_empty() {
                    cleaner::report_clean(files_to_clean.clone(), start_time);

                    let proceed = Confirm::new()
                        .with_prompt(match lang {
                            "en" => "Do you want to delete these files?",
                            "fr" => "Voulez-vous supprimer ces fichiers?",
                            _ => "Do you want to delete these files?",
                        })
                        .interact()
                        .unwrap();

                    if proceed {
                        cleaner::clean_files(files_to_clean, matches.get_flag("secure-clean"));
                    } else {
                        println!("{}", match lang {
                            "en" => "Operation cancelled by user.",
                            "fr" => "Opération annulée par l'utilisateur.",
                            _ => "Operation cancelled by user.",
                        });
                    }
                } else {
                    println!("{}", match lang {
                        "en" => "No files to clean.",
                        "fr" => "Aucun fichier à nettoyer.",
                        _ => "No files to clean.",
                    });
                }
            },
            // Nettoyer les fichiers inutiles
            1 => {
                let files_to_clean = cleaner::scan_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

                if !files_to_clean.is_empty() {
                    cleaner::report_clean(files_to_clean.clone(), start_time);

                    let proceed = Confirm::new()
                        .with_prompt(match lang {
                            "en" => "Do you want to delete the above files?",
                            "fr" => "Voulez-vous supprimer les fichiers ci-dessus?",
                            _ => "Do you want to delete the above files?",
                        })
                        .interact()
                        .unwrap();

                    if proceed {
                        cleaner::clean_files(files_to_clean, matches.get_flag("secure-clean"));
                    } else {
                        println!("{}", match lang {
                            "en" => "Operation cancelled by user.",
                            "fr" => "Opération annulée par l'utilisateur.",
                            _ => "Operation cancelled by user.",
                        });
                    }
                } else {
                    println!("{}", match lang {
                        "en" => "No files to clean.",
                        "fr" => "Aucun fichier à nettoyer.",
                        _ => "No files to clean.",
                    });
                }
            },
            // Vider le dossier Téléchargements
            2 => {
                let proceed = Confirm::new()
                    .with_prompt(match lang {
                        "en" => "Do you want to clear the Downloads folder?",
                        "fr" => "Voulez-vous vider le dossier Téléchargements?",
                        _ => "Do you want to clear the Downloads folder?",
                    })
                    .interact()
                    .unwrap();

                if proceed {
                    #[cfg(target_os = "windows")]
                        let downloads_dir = dirs::download_dir().unwrap();
                    #[cfg(not(target_os = "windows"))]
                        let downloads_dir = dirs::home_dir().unwrap().join("Downloads");
                    cleaner::clear_directory(downloads_dir);
                } else {
                    println!("{}", match lang {
                        "en" => "Operation cancelled by user.",
                        "fr" => "Opération annulée par l'utilisateur.",
                        _ => "Operation cancelled by user.",
                    });
                }
            },
            // Vider la corbeille
            3 => {
                let proceed = Confirm::new()
                    .with_prompt(match lang {
                        "en" => "Do you want to clear the Trash?",
                        "fr" => "Voulez-vous vider la corbeille?",
                        _ => "Do you want to clear the Trash?",
                    })
                    .interact()
                    .unwrap();

                if proceed {
                    cleaner::clear_trash();
                } else {
                    println!("{}", match lang {
                        "en" => "Operation cancelled by user.",
                        "fr" => "Opération annulée par l'utilisateur.",
                        _ => "Operation cancelled by user.",
                    });
                }
            },
            // Générer un rapport après le nettoyage (doit être utilisé avec --scan ou --clean)
            4 => {
                eprintln!("{}", match lang {
                    "en" => "Error: The --report flag must be used with --scan or --clean.",
                    "fr" => "Erreur : Le drapeau --report doit être utilisé avec --scan ou --clean.",
                    _ => "Error: The --report flag must be used with --scan or --clean.",
                });
            },
            // Planifier un nettoyage automatique
            5 => {
                let intervals = match lang {
                    "en" => &[
                        "1 hour",
                        "2 hours",
                        "3 hours",
                        "4 hours",
                        "5 hours",
                        "6 hours",
                        "12 hours",
                        "24 hours (daily)"
                    ],
                    "fr" => &[
                        "1 heure",
                        "2 heures",
                        "3 heures",
                        "4 heures",
                        "5 heures",
                        "6 heures",
                        "12 heures",
                        "24 heures (quotidien)"
                    ],
                    _ => &[
                        "1 hour",
                        "2 hours",
                        "3 hours",
                        "4 hours",
                        "5 hours",
                        "6 hours",
                        "12 hours",
                        "24 hours (daily)"
                    ]
                };

                let interval_selection = Select::new()
                    .with_prompt(match lang {
                        "en" => "Select the interval for automatic cleaning",
                        "fr" => "Sélectionnez l'intervalle pour le nettoyage automatique",
                        _ => "Select the interval for automatic cleaning",
                    })
                    .items(intervals)
                    .default(0)
                    .interact()
                    .unwrap();

                let schedule_expression = match interval_selection {
                    0 => "0 0 * * * *",     // Every 1 hour
                    1 => "0 0 */2 * * *",   // Every 2 hours
                    2 => "0 0 */3 * * *",   // Every 3 hours
                    3 => "0 0 */4 * * *",   // Every 4 hours
                    4 => "0 0 */5 * * *",   // Every 5 hours
                    5 => "0 0 */6 * * *",   // Every 6 hours
                    6 => "0 0 */12 * * *",  // Every 12 hours
                    7 => "0 0 0 * * *",     // Every 24 hours (daily)
                    _ => {
                        eprintln!("{}", match lang {
                            "en" => "Invalid selection. Please try again.",
                            "fr" => "Sélection invalide. Veuillez réessayer.",
                            _ => "Invalid selection. Please try again.",
                        });
                        return;
                    }
                };

                if Schedule::from_str(schedule_expression).is_ok() {
                    let dirs_to_scan = dirs_to_scan.clone();
                    let exclude_dirs = exclude_dirs.clone();
                    let exclude_types = exclude_types.clone();
                    let secure_clean = matches.get_flag("secure-clean");

                    let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

                    task::spawn(async move {
                        run_scheduled_job(schedule_expression, dirs_to_scan, exclude_dirs, exclude_types, secure_clean, shutdown_rx).await;
                    });

                    println!("{}", match lang {
                        "en" => "Scheduled job set. The application will now run indefinitely.",
                        "fr" => "Tâche planifiée définie. L'application va maintenant s'exécuter indéfiniment.",
                        _ => "Scheduled job set. The application will now run indefinitely.",
                    });

                    tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");

                    shutdown_tx.send(()).await.expect("Failed to send shutdown signal");
                    println!("{}", match lang {
                        "en" => "Shutdown signal sent. Exiting.",
                        "fr" => "Signal d'arrêt envoyé. Sortie.",
                        _ => "Shutdown signal sent. Exiting.",
                    });
                } else {
                    eprintln!("{} {}", match lang {
                        "en" => "Invalid cron expression: {}",
                        "fr" => "Expression cron invalide : {}",
                        _ => "Invalid cron expression: {}",
                    }, schedule_expression);
                }
            },
            // Analyser et supprimer les fichiers en double
            6 => {
                let files_to_clean = cleaner::find_duplicate_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

                if !files_to_clean.is_empty() {
                    cleaner::print_duplicates_report(&files_to_clean);

                    let proceed = Confirm::new()
                        .with_prompt(match lang {
                            "en" => "Do you want to delete these duplicate files?",
                            "fr" => "Voulez-vous supprimer ces fichiers en double?",
                            _ => "Do you want to delete these duplicate files?",
                        })
                        .interact()
                        .unwrap();

                    if proceed {
                        cleaner::clean_files(files_to_clean, matches.get_flag("secure-clean"));
                    } else {
                        println!("{}", match lang {
                            "en" => "Operation cancelled by user.",
                            "fr" => "Opération annulée par l'utilisateur.",
                            _ => "Operation cancelled by user.",
                        });
                    }
                } else {
                    println!("{}", match lang {
                        "en" => "No duplicate files found.",
                        "fr" => "Aucun fichier en double trouvé.",
                        _ => "No duplicate files found.",
                    });
                }
            },
            // Mode interactif pour la suppression de fichiers
            7 => {
                let files_to_clean = cleaner::scan_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

                if !files_to_clean.is_empty() {
                    for file in &files_to_clean {
                        let proceed = Confirm::new()
                            .with_prompt(format!("{} {}", match lang {
                                "en" => "Do you want to delete this file: {}?",
                                "fr" => "Voulez-vous supprimer ce fichier : {}?",
                                _ => "Do you want to delete this file: {}?",
                            }, file.display()))
                            .interact()
                            .unwrap();

                        if proceed {
                            cleaner::clean_files(vec![file.clone()], matches.get_flag("secure-clean"));
                        }
                    }
                } else {
                    println!("{}", match lang {
                        "en" => "No files to clean.",
                        "fr" => "Aucun fichier à nettoyer.",
                        _ => "No files to clean.",
                    });
                }
            },
            // Nettoyer les fichiers de cache du navigateur
            8 => {
                let proceed = Confirm::new()
                    .with_prompt(match lang {
                        "en" => "Do you want to clean browser cache files?",
                        "fr" => "Voulez-vous nettoyer les fichiers de cache du navigateur?",
                        _ => "Do you want to clean browser cache files?",
                    })
                    .interact()
                    .unwrap();

                if proceed {
                    cleaner::clean_browser_cache();
                    println!("{}", match lang {
                        "en" => "Browser cache cleaned.",
                        "fr" => "Cache du navigateur nettoyé.",
                        _ => "Browser cache cleaned.",
                    });
                } else {
                    println!("{}", match lang {
                        "en" => "Operation cancelled by user.",
                        "fr" => "Opération annulée par l'utilisateur.",
                        _ => "Operation cancelled by user.",
                    });
                }
            },
            // Restaurer les fichiers supprimés
            9 => {
                let proceed = Confirm::new()
                    .with_prompt(match lang {
                        "en" => "Do you want to restore deleted files?",
                        "fr" => "Voulez-vous restaurer les fichiers supprimés?",
                        _ => "Do you want to restore deleted files?",
                    })
                    .interact()
                    .unwrap();

                if proceed {
                    cleaner::restore_files();
                    println!("{}", match lang {
                        "en" => "Files restored.",
                        "fr" => "Fichiers restaurés.",
                        _ => "Files restored.",
                    });
                } else {
                    println!("{}", match lang {
                        "en" => "Operation cancelled by user.",
                        "fr" => "Opération annulée par l'utilisateur.",
                        _ => "Operation cancelled by user.",
                    });
                }
            },
            // Nettoyage sécurisé des fichiers
            10 => {
                let files_to_clean = cleaner::scan_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

                if !files_to_clean.is_empty() {
                    cleaner::report_clean(files_to_clean.clone(), start_time);

                    let proceed = Confirm::new()
                        .with_prompt(match lang {
                            "en" => "Do you want to securely delete these files?",
                            "fr" => "Voulez-vous supprimer ces fichiers en toute sécurité?",
                            _ => "Do you want to securely delete these files?",
                        })
                        .interact()
                        .unwrap();

                    if proceed {
                        cleaner::secure_clean_files(files_to_clean);
                    } else {
                        println!("{}", match lang {
                            "en" => "Operation cancelled by user.",
                            "fr" => "Opération annulée par l'utilisateur.",
                            _ => "Operation cancelled by user.",
                        });
                    }
                } else {
                    println!("{}", match lang {
                        "en" => "No files to clean.",
                        "fr" => "Aucun fichier à nettoyer.",
                        _ => "No files to clean.",
                    });
                }
            },
            // Nettoyer les fichiers plus anciens qu'un nombre de jours spécifié
            11 => {
                let age_in_days: i64 = Input::new()
                    .with_prompt(match lang {
                        "en" => "Enter the number of days",
                        "fr" => "Entrez le nombre de jours",
                        _ => "Enter the number of days",
                    })
                    .interact_text()
                    .unwrap();

                let files_to_clean = cleaner::scan_files_for_age(&dirs_to_scan, &exclude_dirs, age_in_days);

                if !files_to_clean.is_empty() {
                    cleaner::report_clean(files_to_clean.clone(), start_time);

                    let proceed = Confirm::new()
                        .with_prompt(match lang {
                            "en" => "Do you want to delete the above files?",
                            "fr" => "Voulez-vous supprimer les fichiers ci-dessus?",
                            _ => "Do you want to delete the above files?",
                        })
                        .interact()
                        .unwrap();

                    if proceed {
                        cleaner::clean_files(files_to_clean, matches.get_flag("secure-clean"));
                    } else {
                        println!("{}", match lang {
                            "en" => "Operation cancelled by user.",
                            "fr" => "Opération annulée par l'utilisateur.",
                            _ => "Operation cancelled by user.",
                        });
                    }
                } else {
                    println!("{}", match lang {
                        "en" => "No files to clean.",
                        "fr" => "Aucun fichier à nettoyer.",
                        _ => "No files to clean.",
                    });
                }
            },
            // Sélection invalide
            _ => {
                println!("{}", match lang {
                    "en" => "Invalid selection.",
                    "fr" => "Sélection invalide.",
                    _ => "Invalid selection.",
                });
            }
        }
    }
}
