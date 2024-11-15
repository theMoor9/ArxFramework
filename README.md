# ArxFramework

## Introduction

**ArxFramework** is a modular and flexible library that takes the form of a "Boilerplate" once configured, facilitating the development of a designated application. It is designed to support the development of various types of applications, including web applications, API backends, desktop applications, automation and scripting systems, and embedded systems.

### Key Features:

1. **Modular Architecture**: Composed of specialized modules like the core system, authentication, CRUD, file management, monitoring, and others, each optimized for its domain and easily configurable.
2. **Multi-language**: Primarily implemented in **Rust** for performance and security, with **Python** integrations for flexibility, especially in machine learning operations.
3. **Versatility**: ArxFramework adapts to the needs of five main types of applications, allowing specific customizations through configuration.
4. **Robust Core System**: A central system in **Rust** that manages fundamental operations such as memory management, concurrency, and other base operations that can be extended.
5. **Frontend Integration**: Support for dynamic frontends using **Svelte**, allowing the creation of modern interfaces with ease.
6. **Advanced Capabilities**: Includes modules for **machine learning**, **blockchain** integration, and task automation.
7. **Logging and Monitoring**: Integrated logging and performance monitoring system, essential for tracking the application's behavior.
8. **File and Resource Management**: A dedicated module for efficient file and resource management, configurable based on the application in use.
9. **Documentation and Templates**: Provides comprehensive documentation and **templates** that facilitate module configuration and the integration of custom models.

### Scope

ArxFramework offers a solid and flexible foundation for the development of modular applications. By combining the efficiency of **Rust** with the versatility of **Python** and other modern tools, the framework is designed to be easily extensible and adaptable. Thanks to its CLI-based configuration and **templates**, it allows the creation of tailored applications, from rapid prototyping to building complex, high-performance systems, maximizing potential through the customized integration of modules.

---

**Table of Contents**

- [Type of Applications](#Type-of-Applications)
- [Functionality and Unique Technologies](#Functionality-and-Unique-Technologies)
- [Folders](#Folders)
- [Scope](#Scope)
- [HOW to USE](#HOW-to-USE)

---

# Type of Applications

1. **Web Applications**: Apps that run in browsers, can be single-page applications (SPA) or multi-page apps.
2. **API Backend**: Services that provide data and functions to other applications (mobile, web, desktop) through a REST or GraphQL API.
3. **Desktop Applications**: Applications that run directly on operating systems like Windows, macOS, or Linux, with offline functionality and local resource management.
4. **Automation and Scripting**: Tools or scripts that automate specific processes, such as data extraction, analysis, or repetitive tasks.
5. **Embedded or Performance-Critical Systems**: Apps that require high performance, such as IoT systems, microservices, or applications for specific hardware.

---

# Functionality and Unique Technologies

## Modular Structure Table for 5 Solutions

| Module                    | Web App | API Backend | Desktop App | Automation & Scripting | Embedded Systems |
| ------------------------- | ------- | ----------- | ----------- | ---------------------- | ---------------- |
| Core System (Rust)        | ✓       | ✓           | ✓           | ✓                      | ✓                |
| Authentication & Security | ✓       | ✓           | ✓           | ○                      | ○                |
| CRUD Management           | ✓       | ✓           | ✓           | ○                      | ○                |
| API Layer                 | ✓       | ✓           | ○           | ○                      | ○                |
| Network                   | ✓<br>   | ✓<br>       | ✓<br>       | ✓<br>                  | ✓<br>            |
| File/Resource Management  | ○       | ○           | ✓           | ✓                      | ○                |
| Monitoring & Logging      | ✓       | ✓           | ✓           | ✓                      | ✓                |
| Task Automation           | ○       | ○           | ○           | ✓                      | ○                |
| Dynamic Frontend          | ✓       | ○           | ✓           | ○                      | ○                |
| Blockchain Integration    | ✓       | ✓           | ✓           | ✓                      | ✓                |
| Machine Learning          | ✓       | ✓           | ✓           | ✓                      | ✓                |

Legend:

- ✓: Typically used in this solution
- ○: Optional module or used based on specific requirements

### Expanded Module Details for Development

| Module                    | Primary Language      | Support Language | Wrapping | Main Framework/Libraries |
| ------------------------- | --------------------- | ---------------- | -------- | ------------------------ |
| Core System               | Rust                  | -                | -        | tokio (async runtime)    |
| Authentication & Security | Rust                  | Python           | Yes      | jsonwebtoken, bcrypt     |
| CRUD Management           | Rust                  | -                | -        | diesel (ORM)             |
| API Layer                 | Rust (warp/actix-web) | Python (FastAPI) | No       | serde (serialization)    |
| Network                   | Rust<br>              | -<br>            | -        | Diesel,Tokio,pyo3        |
| File/Resource Management  | Rust                  | -                | -        | std::fs, tokio::fs       |
| Monitoring & Logging      | Rust                  | -                | -        | tracing, prometheus      |
| Task Automation           | Rust                  | Python           | Yes      | rayon (parallelism)      |
| Dynamic Frontend          | Svelte (TypeScript)   | -                | -        | SvelteKit, DaisyUI (CSS) |
| Blockchain Integration    | Rust                  | Python           | Yes      | Solana web3              |
| Machine Learning          | Python                | Rust             | Yes      | PyTorch, scikit-learn    |

Additional Notes:

- For each module, consider implementing unit and integration tests.
- Document internal and external APIs for each module.
- Implement a flexible configuration system for each module.
- Consider using Docker containers to standardize the development and deployment environment.

### Support Languages (Optional)

#### Distinction Between Wrapping and Non-Wrapping Approaches

1. **Wrapping Approach** (e.g., Authentication and Security): Rust code is encapsulated in Python functions for easier use. Example Rust (with PyO3):
    
    ```Rust
	#[pyfunction] 
	fn verify_password(password: &str, hash: &str) -> PyResult<bool> {     
		// Rust Implementation    
		Ok(true) 
		// placeholder 
	}
	```
    
    Python Example:
	
    ```Python
	from rust_auth import verify_password 
	
	def user_login(username, password):     
		stored_hash = get_stored_hash(username)    
		return verify_password(password, stored_hash)
	```
    
2. **Unwrapped Approach** (e.g., Task Automation): Rust and Python cooperate, with Python orchestrating and calling Rust functions when needed. Example Rust (with PyO3):
    
    ```Rust
	#[pyfunction] 
	fn process_data(data: Vec<f64>) -> PyResult<Vec<f64>> {
	     // EIntensive processing in Rust    
	     Ok(data.iter().map(|&x| x * 2.0).collect()) 
	}
	```
    
    Python Example:
    
    ```Rust
	import rust_processor 
	
	def run_pipeline():     
		data = load_data()    
		processed = rust_processor.process_data(data)    
		save_results(processed)
	```
	
- In the first approach, Rust functionality is “wrapped” in a Python interface.
- In the second, Python directly uses Rust functions as part of a larger workflow.

> Apply these methodologies in relation to the earlier table.

---

# Folders

```sh
ArxFramework/
├── core/                 # Core System (Rust)
├── auth/                 # Authentication and Security (Rust, optional Python)
├── crud/                 # CRUD Management (Rust)
| └── models/               # Models folder
├── api/                  # API Layer (Primary Rust, optional Python FastAPI)
├── network/              # Handles networking
├── file_management/      # File/Resource Management (Rust)
├── monitoring/           # Monitoring and logging (Rust)
| └── logs/               # Logs folder
├── task_automation/      # Task Automation (Primary Rust, secondary Python)
├── blockchain/           # Blockchain Integration (Rust)
├── frontend/             # Hosts the front end
├── ml/                   # Machine Learning (Primary Python, secondary Rust)
├── docs/                 # Framework documentation
├── config/               # Global configurations
├── src/                  # Source
├── tests/                # Unit tests
└── README.md             # Main framework documentation

```

**core/**

- `system_core.rs`: Central system management
- `memory_management.rs`: Memory optimization

**auth/**

- `auth_core.rs`: Core authentication functionality
- `auth_wrapper.py`: Python wrapper for integrations

**crud/**

- **models/**: Directory containing `.rs` models built following the default procedure for integration
    - `user_model.rs`: Model containing the `struct` that defines it and the `enum` indicating the properties inheritable from `crud_ops.rs`.
    - `article_model.rs`: As described for `user_model.rs`
    - other models...
- `crud_ops.rs`: General CRUD operations

**api/**

- `api_server.rs`: Main API server
- `routes.rs`: API route definitions
- `fastapi_integration.py`: Integration with FastAPI

**network/**

- `connection_manager.rs`: Manages active connections and the connection pool.  
	- Functions to establish, maintain, and close persistent connections.  
	- Retry handling for failed connections.

- `load_balancer.rs`: Implements various load-balancing algorithms.  
	- Data structures to monitor the load on each server (e.g., the number of active connections).  
	- Functions to distribute requests based on the chosen balancing algorithm.

 - `resource_pool.rs`: Centrally manages network resources, such as connections.  
	- Connection pooling to minimize latency.  
	- Configurable options to limit the maximum number of simultaneous connections

**file_management/**

- `file_ops.rs`: File operations
- `resource_manager.rs`: Resource management

**monitoring/**

- **logs/**: Contains log files for respective modules
- `logger.rs`: Logging system
- `metrics.rs`: Metrics collection and reporting

**task_automation/**

- `task_core.rs`: Core functions for automation
- `automation_scripts.py`: Python automation scripts

**blockchain/**

- `blockchain_integration.rs`: Blockchain interface
- `smart_contracts.rs`: Smart contract management

**frontend/** Hosts the frontend

- `App.svelte`: Root Svelte component
- `index.js`: Application entry point
- `components/`: Reusable components directory

**ml/**

- `ml_models.py`: ML model implementation
- `data_processing.rs`: High-performance data processing

**docs/**

- Documentation files for each module

**tests/**

- Contains unit tests.

**config/**

- `global_config.rs`: Configuration file for the entire framework
- `network_config.rs`: Guess what

**src/**

- `main.rs`: Framework entry point and CLI encapsulator.
- `lib.rs`: Contains references to all module folders (_crates_).
](module_proprieties.md)

---

# How to Use ArxFramework

##### You can find useful infos about the modules [here](./docs/module-templates) 

## Setup

ArxFramework is designed to help you create projects with predefined templates for various types of applications: Web App, API Backend, Desktop App, Automation and Scripting, and Embedded Systems. Follow the steps below to install and use the framework effectively.

#### **Step 1: Install the CLI Tool**

To start using ArxFramework, you need to install the CLI tool globally on your system using Cargo.

```sh
cargo install arx_framework
```

This command will:

- Download and build the ArxFramework CLI tool (called `arx`).
- Install the `arx` binary in Cargo’s bin directory (`~/.cargo/bin` on Linux/macOS or `%USERPROFILE%\.cargo\bin` on Windows).

#### **Step 2: Verify Installation**

After installation, ensure that the `arx` command is available in your terminal. You may need to add Cargo’s bin directory to your system’s PATH variable if it isn’t already.

To verify that the installation was successful:

```sh
arx --version
```

This command will print the version of the installed `arx` CLI tool.

#### **Step 3: Initialize a New Project**

Now you can create a new project by running the `arx init` command. This command will set up the entire framework structure for the application type you choose.

1. **Create a new directory for your project**:
    
    ```sh
	mkdir mio_nuovo_progetto 
	cd mio_nuovo_progetto
	```
    
2. **Run the `arx init` command** with the application type (`--AppType`), where `--AppType` can be `webapp`, `api_backend`, `desktop_app`, `automation`, or `embedded`:
    
    ```sh
	arx init --AppType webapp
	```
    
    This command will generate a project structure tailored to the specified application type.

#### **Step 4: Project Structure**

Once the project is initialized, you will have a pre-configured directory structure based on the type of application you selected. For example, for a web app, the structure might look like this:

```sh
ArxFramework/
├── core/                 # Core Sistema (Rust)
├── auth/                 # Autenticazione e Sicurezza (Rust, Python opzionale)
├── crud/                 # Gestione CRUD (Rust)
| └── models/               # Cartella dei modelli
├── api/                  # API Layer (Rust primario, Python FastAPI opzionale)
├── file_management/      # Gestione file/risorse (Rust)
├── monitoring/           # Monitoraggio e logging (Rust)
| └── logs/               # Cartella dei log
├── task_automation/      # Task Automation (Rust primario, Python secondario)
├── blockchain/           # Integrazione Blockchain (Rust)
├── frontend/             # Ospita il front end
├── ml/                   # Machine Learning (Python primario, Rust secondario)
├── docs/                 # Documentazione del framework
├── config/               # Configurazioni globali
├── src/                  # Source
├── tests/                # Test unitari 
└── README.md             # Documentazione principale del framework
```

Each folder contains specific modules to help you start developing your application.

#### **Step 5: Configure and Customize**

1. **Update the `Cargo.toml`**: Depending on your project, you may want to customize the dependencies or features within the `Cargo.toml` file.
    
2. **Modify or Add Functionality**:
    
    - You can modify or extend the models and components located in the `modules/` directory to suit the specific needs of your application.
    - For any customization, follow the documented structure inside each module.
    
3. **Command Line Arguments**: The CLI tool allows you to pass various arguments to customize the project initialization. Refer to the CLI documentation (`arx --help`) to explore the available options.
    

#### **Step 6: Build and Run**

Once your project is configured and customized, you can build and run it using Cargo:

```sh
cargo build cargo run
```

This command will compile the project and run the application based on the configuration specified during initialization.

#### **Step 7: Add More Features or Modify Components**

- You can further customize the framework by adding new models or extending existing ones in the `crud/` or `core/` folders.
- Use the macros provided in the `crud_ops.rs` file to implement additional CRUD functionality for new models.

#### **Step 8: Continuous Development**

- Use a version control system like Git to manage changes to your project.
- During development, continue to customize the generated modules to meet the needs of your application.

### Example Workflow

1. **Install the CLI**:
    
    ```sh
	cargo install arx_framework
	```
    
2. **Create a new directory and initialize a web app**:
    
    ```sh
	mkdir mia_webapp
	cd mia_webapp 
	arx init --AppType webapp
	```
    
3. **Build and run your new web app**:
    
    ```sh
	cargo build cargo run
	```
    

This will launch your web application with the pre-configured structure of ArxFramework.

## Project Scalability
Implementation of new system's components


---

**Author**: Kenneth Boldrini