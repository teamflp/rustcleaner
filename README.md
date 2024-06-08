# Cleaner Tool Documentation

## Overview

The Cleaner Tool is a command-line application designed to scan, clean, and manage unnecessary files on your system. It supports multiple features including scanning directories, cleaning files, clearing the Downloads folder and Trash, generating reports, scheduling automatic cleanups, and more. The tool is bilingual, supporting both English and French languages.


## Prerequisites

Ensure that you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

## Features
### 1. Scan Directories for Unnecessary Files
Scans the specified directories for files with extensions such as .tmp, .log, .old, .bak.

### 2. Clean Unnecessary Files

Scans and cleans the specified directories of unnecessary files. Prompts the user before deletion.

### 3. Clear the Downloads Folder

Clears all files in the Downloads folder.

### 4. Clear the Trash

Empties the system's Trash directory.

### 5. Generate a Report After Cleaning

Generates a report summarizing the cleaning operation, including the total number of files cleaned and the total size.

### 6. Schedule an Automatic Cleaning (Every x Hours)

Allows you to schedule an automatic cleaning job at regular intervals (e.g., every hour, every 2 hours).

### 7. Analyze and Remove Duplicate Files

## Installation

To install the Cleaner Tool, ensure you have Rust installed on your system. Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/teamflp/rustcleaner.git
cd rustcleaner
cargo build --release
```

## Usage
### Run the Cleaner Tool using the following command:

```sh
cargo run
```

### Upon launching, the tool will prompt you to select a language:

```sh
Choose a language:
Choose your language / Choisissez votre langue:
> English
  Français
```

Select the desired language to proceed.

## Main Menu
After selecting the language, you will be presented with the main menu:

```sh
Choose an action:
  1 => Scan directories for unnecessary files
  2 => Clean unnecessary files
  3 => Clear the Downloads folder
  4 => Clear the Trash
  5 => Generate a report after cleaning
> 6 => Schedule an automatic cleaning (every x hours)
  7 => Analyze and remove duplicate files
  8 => Interactive mode for file deletion
  9 => Clean browser cache files
  10 => Restore deleted files
  11 => Secure file cleaning
  12 => Clean files older than a specified number of days
  q => Enter q to quit
```

### Select the scheduling option from the main menu:

```sh
6 => Schedule an automatic cleaning (every x hours)
```
Choose the interval:

```sh
> 1 hour
  2 hours
  3 hours
  4 hours
  5 hours
  6 hours
  12 hours
```
The application will now run indefinitely, performing the scheduled cleaning job at the selected interval.
```sh
Select the interval for automatic cleaning: 1 hour

Scheduled job set. The application will now run indefinitely.
```







