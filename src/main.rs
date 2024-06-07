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

enum Language {
    English,
    French,
}

fn get_options(lang: &Language) -> Vec<&'static str> {
    match lang {
        Language::English => vec![
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
            "q => Enter q to quit",
        ],
        Language::French => vec![
            "1 => Scanner les répertoires pour les fichiers inutiles",
            "2 => Nettoyer les fichiers inutiles",
            "3 => Vider le dossier Téléchargements",
            "4 => Vider la Corbeille",
            "5 => Générer un rapport après le nettoyage",
            "6 => Planifier un nettoyage automatique (toutes les x heures)",
            "7 => Analyser et supprimer les fichiers en double",
            "8 => Mode interactif pour la suppression des fichiers",
            "9 => Nettoyer les fichiers de cache du navigateur",
            "10 => Restaurer les fichiers supprimés",
            "11 => Nettoyage sécurisé des fichiers",
            "12 => Nettoyer les fichiers plus anciens qu'un certain nombre de jours",
            "q => Entrer q pour quitter",
        ],
    }
}

#[tokio::main]
async fn main() {
    let languages = &["English", "Français"];
    let language_selection = Select::new()
        .with_prompt("Choose your language / Choisissez votre langue")
        .items(languages)
        .default(0)
        .interact()
        .unwrap();

    let lang = if language_selection == 0 {
        Language::English
    } else {
        Language::French
    };

    let welcome_message = match lang {
        Language::English => "==========================================================\n|                 Welcome to Cleaner Tool                |\n==========================================================",
        Language::French => "==========================================================\n|                 Bienvenue à Cleaner Tool                |\n==========================================================",
    };

    let choose_action_prompt = match lang {
        Language::English => "Choose an action",
        Language::French => "Choisissez une action",
    };

    let quit_prompt = match lang {
        Language::English => "Are you sure you want to quit?",
        Language::French => "Êtes-vous sûr de vouloir quitter?",
    };

    let options = get_options(&lang);

    println!();
    println!("{}", welcome_message);
    println!();

    let selection = Select::new()
        .with_prompt(choose_action_prompt)
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    println!();

    if options[selection] == *options.last().unwrap() {
        let proceed = Confirm::new()
            .with_prompt(quit_prompt)
            .interact()
            .unwrap();

        if proceed {
            println!("Exiting...");
            return;
        } else {
            println!("Continuing...");
        }
    }
    println!();
    let matches = Command::new("Cleaner Tool")
        .version("1.0")
        .author("Paterne G. G. it.devwebm@gmail.com")
        .about(if let Language::English = lang { "Cleans unnecessary files from your system" } else { "Nettoie les fichiers inutiles de votre système" })
        .arg(Arg::new("scan")
            .short('s')
            .long("scan")
            .help(if let Language::English = lang { "Scans directories for unnecessary files" } else { "Scanne les répertoires pour les fichiers inutiles" })
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("clean")
            .short('c')
            .long("clean")
            .help(if let Language::English = lang { "Cleans unnecessary files" } else { "Nettoie les fichiers inutiles" })
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("clear-downloads")
            .short('d')
            .long("clear-downloads")
            .help(if let Language::English = lang { "Clears the Downloads folder" } else { "Vider le dossier Téléchargements" })
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("clear-trash")
            .short('t')
            .long("clear-trash")
            .help(if let Language::English = lang { "Clears the Trash" } else { "Vider la Corbeille" })
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("report")
            .short('r')
            .long("report")
            .help(if let Language::English = lang { "Generates a report after cleaning" } else { "Générer un rapport après le nettoyage" })
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("schedule")
            .long("schedule")
            .value_name("SCHEDULE")
            .help(if let Language::English = lang { "Schedules an automatic cleaning (every x hours)" } else { "Planifie un nettoyage automatique (toutes les x heures)" })
            .action(clap::ArgAction::Set))
        .arg(Arg::new("duplicates")
            .long("duplicates")
            .help(if let Language::English = lang { "Analyzes and removes duplicate files" } else { "Analyser et supprimer les fichiers en double" })
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("exclude")
            .short('e')
            .long("exclude")
            .value_name("DIR")
            .num_args(1..)
            .help(if let Language::English = lang { "Excludes specified directories from the scan" } else { "Exclut les répertoires spécifiés de l'analyse" }))
        .arg(Arg::new("exclude-type")
            .long("exclude-type")
            .value_name("EXTENSIONS")
            .num_args(1..)
            .help(if let Language::English = lang { "Excludes specified file types from the scan" } else { "Exclut les types de fichiers spécifiés de l'analyse" }))
        .arg(Arg::new("dir")
            .long("dir")
            .value_name("DIRECTORY")
            .num_args(1..)
            .help(if let Language::English = lang { "Adds custom directories to clean" } else { "Ajouter des répertoires personnalisés à nettoyer" }))
        .arg(Arg::new("interactive")
            .long("interactive")
            .help(if let Language::English = lang { "Interactive mode for file deletion" } else { "Mode interactif pour la suppression des fichiers" })
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("clean-browser")
            .long("clean-browser")
            .help(if let Language::English = lang { "Cleans browser cache files" } else { "Nettoie les fichiers de cache du navigateur" })
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("restore")
            .long("restore")
            .help(if let Language::English = lang { "Restores deleted files" } else { "Restaurer les fichiers supprimés" })
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("secure-clean")
            .long("secure-clean")
            .help(if let Language::English = lang { "Secure file cleaning" } else { "Nettoyage sécurisé des fichiers" })
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("age")
            .long("age")
            .value_name("DAYS")
            .help(if let Language::English = lang { "Cleans files older than the specified number of days" } else { "Nettoie les fichiers plus anciens que le nombre de jours spécifié" })
            .action(clap::ArgAction::Set))
        .get_matches();

    let mut exclude_dirs: HashSet<String> = matches.get_many::<String>("exclude")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();
    exclude_dirs.extend(get_default_excludes());

    let exclude_types: HashSet<String> = matches.get_many::<String>("exclude-type")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();

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
        80 // Default width if terminal size is not detected
    };

    match selection {
        0 => {
            let files_to_clean = cleaner::scan_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

            if !files_to_clean.is_empty() {
                if matches.get_flag("report") {
                    cleaner::report_clean(files_to_clean.clone(), start_time);
                }

                let proceed = Confirm::new()
                    .with_prompt(if let Language::English = lang { "Do you want to delete these files?" } else { "Voulez-vous supprimer ces fichiers?" })
                    .interact()
                    .unwrap();

                if proceed {
                    cleaner::clean_files(files_to_clean, matches.get_flag("secure-clean"));
                } else {
                    println!("{}", if let Language::English = lang { "Operation cancelled by user." } else { "Opération annulée par l'utilisateur." });
                }
            } else {
                println!("{}", if let Language::English = lang { "No files to clean." } else { "Aucun fichier à nettoyer." });
            }
        },
        1 => {
            let files_to_clean = cleaner::scan_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

            if !files_to_clean.is_empty() {
                if matches.get_flag("report") {
                    cleaner::report_clean(files_to_clean.clone(), start_time);
                }

                let proceed = Confirm::new()
                    .with_prompt(if let Language::English = lang { "Do you want to delete the above files?" } else { "Voulez-vous supprimer les fichiers ci-dessus?" })
                    .interact()
                    .unwrap();

                if proceed {
                    cleaner::clean_files(files_to_clean, matches.get_flag("secure-clean"));
                } else {
                    println!("{}", if let Language::English = lang { "Operation cancelled by user." } else { "Opération annulée par l'utilisateur." });
                }
            } else {
                println!("{}", if let Language::English = lang { "No files to clean." } else { "Aucun fichier à nettoyer." });
            }
        },
        2 => {
            let proceed = Confirm::new()
                .with_prompt(if let Language::English = lang { "Do you want to clear the Downloads folder?" } else { "Voulez-vous vider le dossier Téléchargements?" })
                .interact()
                .unwrap();

            if proceed {
                #[cfg(target_os = "windows")]
                    let downloads_dir = dirs::download_dir().unwrap();
                #[cfg(not(target_os = "windows"))]
                    let downloads_dir = dirs::home_dir().unwrap().join("Downloads");
                cleaner::clear_directory(downloads_dir);
            } else {
                println!("{}", if let Language::English = lang { "Operation cancelled by user." } else { "Opération annulée par l'utilisateur." });
            }
        },
        3 => {
            let proceed = Confirm::new()
                .with_prompt(if let Language::English = lang { "Do you want to clear the Trash?" } else { "Voulez-vous vider la Corbeille?" })
                .interact()
                .unwrap();

            if proceed {
                cleaner::clear_trash();
            } else {
                println!("{}", if let Language::English = lang { "Operation cancelled by user." } else { "Opération annulée par l'utilisateur." });
            }
        },
        4 => {
            eprintln!("{}", if let Language::English = lang { "Error: The --report flag must be used with --scan or --clean." } else { "Erreur : Le drapeau --report doit être utilisé avec --scan ou --clean." });
        },
        5 => {
            let intervals = if let Language::English = lang {
                vec![
                    "1 hour",
                    "2 hours",
                    "3 hours",
                    "4 hours",
                    "5 hours",
                    "6 hours",
                    "12 hours",
                    "24 hours (daily)"
                ]
            } else {
                vec![
                    "1 heure",
                    "2 heures",
                    "3 heures",
                    "4 heures",
                    "5 heures",
                    "6 heures",
                    "12 heures",
                    "24 heures (quotidien)"
                ]
            };

            let interval_selection = Select::new()
                .with_prompt(if let Language::English = lang { "Select the interval for automatic cleaning" } else { "Sélectionnez l'intervalle pour le nettoyage automatique" })
                .items(&intervals)
                .default(0)
                .interact()
                .unwrap();

            let schedule_expression = match interval_selection {
                0 => "0 * * * *",     // Every 1 hour
                1 => "0 */2 * * *",   // Every 2 hours
                2 => "0 */3 * * *",   // Every 3 hours
                3 => "0 */4 * * *",   // Every 4 hours
                4 => "0 */5 * * *",   // Every 5 hours
                5 => "0 */6 * * *",   // Every 6 hours
                6 => "0 */12 * * *",  // Every 12 hours
                7 => "0 0 * * *",     // Every 24 hours (daily)
                _ => {
                    eprintln!("{}", if let Language::English = lang { "Invalid selection. Please try again." } else { "Sélection invalide. Veuillez réessayer." });
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

                println!("{}", if let Language::English = lang { "Scheduled job set. The application will now run indefinitely." } else { "Travail planifié. L'application fonctionnera maintenant indéfiniment." });

                tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");

                shutdown_tx.send(()).await.expect("Failed to send shutdown signal");
                println!("{}", if let Language::English = lang { "Shutdown signal sent. Exiting." } else { "Signal d'arrêt envoyé. Quitter." });
            } else {
                eprintln!("{} {}", if let Language::English = lang { "Invalid cron expression: {}" } else { "Expression cron invalide : {}" }, schedule_expression);
            }
        },
        6 => {
            // Implementing "Analyze and remove duplicate files"
            let files_to_clean = cleaner::find_duplicate_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

            if !files_to_clean.is_empty() {
                cleaner::print_duplicates_report(&files_to_clean);

                let proceed = Confirm::new()
                    .with_prompt(if let Language::English = lang { "Do you want to delete these duplicate files?" } else { "Voulez-vous supprimer ces fichiers en double?" })
                    .interact()
                    .unwrap();

                if proceed {
                    cleaner::clean_files(files_to_clean, matches.get_flag("secure-clean"));
                } else {
                    println!("{}", if let Language::English = lang { "Operation cancelled by user." } else { "Opération annulée par l'utilisateur." });
                }
            } else {
                println!("{}", if let Language::English = lang { "No duplicate files found." } else { "Aucun fichier en double trouvé." });
            }
        },
        7 => {
            // Implementing "Interactive mode for file deletion"
            let files_to_clean = cleaner::scan_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

            if !files_to_clean.is_empty() {
                for file in &files_to_clean {
                    let proceed = Confirm::new()
                        .with_prompt(format!("{} {}", if let Language::English = lang { "Do you want to delete this file: {}?" } else { "Voulez-vous supprimer ce fichier : {}?" }, file.display()))
                        .interact()
                        .unwrap();

                    if proceed {
                        cleaner::clean_files(vec![file.clone()], matches.get_flag("secure-clean"));
                    }
                }
            } else {
                println!("{}", if let Language::English = lang { "No files to clean." } else { "Aucun fichier à nettoyer." });
            }
        },
        8 => {
            // Implementing "Clean browser cache files"
            let proceed = Confirm::new()
                .with_prompt(if let Language::English = lang { "Do you want to clean browser cache files?" } else { "Voulez-vous nettoyer les fichiers de cache du navigateur?" })
                .interact()
                .unwrap();

            if proceed {
                cleaner::clean_browser_cache();
                println!("{}", if let Language::English = lang { "Browser cache cleaned." } else { "Cache du navigateur nettoyé." });
            } else {
                println!("{}", if let Language::English = lang { "Operation cancelled by user." } else { "Opération annulée par l'utilisateur." });
            }
        },
        9 => {
            // Implementing "Restore deleted files"
            let proceed = Confirm::new()
                .with_prompt(if let Language::English = lang { "Do you want to restore deleted files?" } else { "Voulez-vous restaurer les fichiers supprimés?" })
                .interact()
                .unwrap();

            if proceed {
                cleaner::restore_files();
                println!("{}", if let Language::English = lang { "Files restored." } else { "Fichiers restaurés." });
            } else {
                println!("{}", if let Language::English = lang { "Operation cancelled by user." } else { "Opération annulée par l'utilisateur." });
            }
        },
        10 => {
            // Implementing "Secure file cleaning"
            let files_to_clean = cleaner::scan_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

            if !files_to_clean.is_empty() {
                if matches.get_flag("report") {
                    cleaner::report_clean(files_to_clean.clone(), start_time);
                }

                let proceed = Confirm::new()
                    .with_prompt(if let Language::English = lang { "Do you want to securely delete these files?" } else { "Voulez-vous supprimer ces fichiers de manière sécurisée?" })
                    .interact()
                    .unwrap();

                if proceed {
                    cleaner::secure_clean_files(files_to_clean);
                } else {
                    println!("{}", if let Language::English = lang { "Operation cancelled by user." } else { "Opération annulée par l'utilisateur." });
                }
            } else {
                println!("{}", if let Language::English = lang { "No files to clean." } else { "Aucun fichier à nettoyer." });
            }
        },
        11 => {
            // Implementing "Clean files older than a specified number of days"
            let age_in_days: i64 = Input::new()
                .with_prompt(if let Language::English = lang { "Enter the number of days" } else { "Entrez le nombre de jours" })
                .interact_text()
                .unwrap();

            let files_to_clean = cleaner::scan_files_for_age(&dirs_to_scan, &exclude_dirs, age_in_days);

            if !files_to_clean.is_empty() {
                if matches.get_flag("report") {
                    cleaner::report_clean(files_to_clean.clone(), start_time);
                }

                let proceed = Confirm::new()
                    .with_prompt(if let Language::English = lang { "Do you want to delete the above files?" } else { "Voulez-vous supprimer les fichiers ci-dessus?" })
                    .interact()
                    .unwrap();

                if proceed {
                    cleaner::clean_files(files_to_clean, matches.get_flag("secure-clean"));
                } else {
                    println!("{}", if let Language::English = lang { "Operation cancelled by user." } else { "Opération annulée par l'utilisateur." });
                }
            } else {
                println!("{}", if let Language::English = lang { "No files to clean." } else { "Aucun fichier à nettoyer." });
            }
        },
        _ => {
            println!("{}", if let Language::English = lang { "Invalid selection." } else { "Sélection invalide." });
        }
    }
}
