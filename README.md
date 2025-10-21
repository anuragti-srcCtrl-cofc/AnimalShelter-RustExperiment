
# 🐾 Rust Animal Shelter VCard Generator

Below are the goals / semi complete features

This project is a high-performance, asynchronous file processor written in idiomatic Rust. It reads animal records from a text file, parses the data, and generates a standard vCard (.vcf) file for each animal concurrently.

This application was developed as an exercise in translating a simple I/O task into a robust, production-ready Rust program, demonstrating advanced concepts such as:

* Asynchronous I/O with **Tokio**.
* Concurrent processing using lightweight async tasks.
* Idiomatic error handling with custom error types (`thiserror`).
* Object-Oriented Programming (OOP) principles (Polymorphism, Abstraction, Encapsulation) modeled with Rust's trait system.
* Safe concurrency patterns (`Arc`) for sharing data between tasks.
* A modular, maintainable codebase with clear separation of concerns.

## ✨ Key Features

-   **Asynchronous Processing:** Reads the input file and writes all output `.vcf` files asynchronously, making it highly efficient for large datasets.
-   **Concurrent by Default:** Each line of the input file is processed in its own concurrent `tokio` task for maximum throughput.
-   **Robust Error Handling:** Skips malformed lines gracefully and reports specific errors without crashing.
-   **Polymorphic Architecture:** Uses Rust traits to model different animal types, allowing for extensible and clean code.
-   **Automatic Directory Creation:** Creates the `vcf_output/` directory to store the generated vCards.

## 📂 Project Structure

The project is organized into distinct modules, each with a single responsibility:


```
animal\_shelter/
├── .git/
├── src/
│   ├── main.rs              // Entry point, async runtime setup
│   ├── error.rs             // Custom AppError and AppResult types
│   ├── models.rs            // Core data structures and the `Animal` trait (OOP logic)
│   ├── parser.rs            // Logic for parsing a single line into an Animal object
│   └── file\_processor.rs    // File I/O and concurrency orchestration
├── animals.txt              // Input data file
├── Cargo.toml               // Project dependencies (tokio, thiserror)
└── README.md                // This file

````

## 🚀 Getting Started

### Prerequisites

You must have the Rust toolchain installed on your system. If you haven't already, install it via [rustup](https://rustup.rs/).

```bash
# Installs rustup, cargo, rustc, and other tools
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
````

### Installation & Execution

1.  **Clone the Repository:**

    ```bash
    git clone [https://github.com/YOUR_USERNAME/animal_shelter.git](https://github.com/YOUR_USERNAME/animal_shelter.git)
    cd animal_shelter
    ```

2.  **Create the Input File:**
    Create a file named `animals.txt` in the root of the project directory and populate it with animal data. The format is described below.

3.  **Build and Run:**
    Use `cargo`, the Rust package manager, to build and run the project with a single command:

    ```bash
    cargo run --release
    ```

      * The first time you run this, Cargo will download and compile the dependencies.
      * The `--release` flag enables optimizations, making the program run much faster.

### Input File Format (`animals.txt`)

Each line in `animals.txt` must contain 7 space-separated fields:

`<reg_num> <name> <type> <vacc_date> <owner_name> <contact> <behavior>`

  - **`reg_num`**: A unique unsigned 64-bit integer.
  - **`name`**: The animal's name.
  - **`type`**: A single character: `D` (Dog), `C` (Cat), `B` (Bird), `H` (Horse), or `R` (Rabbit).
  - **`vacc_date`**: Date in `MM-DD-YYYY` format.
  - **`owner_name`**: Owner's name. Use underscores `_` for spaces.
  - **`contact`**: A phone number or an email address.
  - **`behavior`**: A description. Use underscores `_` for spaces.

**Example `animals.txt`:**

```
1000000001 Buddy D 02-07-2005 Sarah_Jones 555-123-4567 friendly
1000000002 Whiskers C 11-12-2019 Tom_Baker tom.baker@example.com fully_socialized
1000000003 Kiwi B 06-21-2018 Maria_Gonzalez 555-987-6543 feral
```

## 📋 Output

After running the program, a directory named `vcf_output/` will be created containing a `.vcf` file for each valid animal record.

**Example Output File (`Buddy-Dog.vcf`):**

```vcard
BEGIN:VCARD
VERSION:3.0
FN:Buddy
N:;;Buddy;;
ORG:Sarah Jones
TEL;TYPE=CELL:555-123-4567
NOTE:Registration: 1000000001; Type: Dog; Last Vaccination: 02-07-2005; Behavior: friendly
X-ANIMAL-TYPE:Dog
END:VCARD
```

## 🏛️ Architectural Concepts: OOP in Rust

One of the goals of this project was to implement the four pillars of OOP in idiomatic Rust. Here's how they are mapped:

1.  **Encapsulation:** The fields of `AnimalData` are private to the `models` module. Data is accessed and manipulated only through the public methods and functions provided, ensuring data integrity.

2.  **Abstraction:** The `Animal` trait defines a shared, abstract interface (`data()`, `animal_type()`, `adoption_message()`). Concrete types like `Dog` and `Cat` implement this interface, hiding their specific details from the `file_processor` logic.

3.  **Inheritance (via Composition):** Rust favors composition over classical inheritance. Here, `Dog`, `Cat`, etc., "inherit" common functionality by containing an `AnimalData` struct. They build their specific functionality on top of this shared data.

4.  **Polymorphism:** The `parser` returns a `Box<dyn Animal>`, which is a **trait object**. This is Rust's mechanism for dynamic dispatch. It allows us to have a collection of different animal types that all satisfy the `Animal` trait, and we can call methods on them without knowing their concrete type at compile time.

## 📄 License

This project is licensed under the MIT License. See the `LICENSE` file for details.
