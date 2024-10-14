# Main - CLI

**`main.rs`** is used to start the entire application/framework. The role of the **CLI** is to configure the environment and pass the parameters to **`main`**, which in turn configures the rest of the modules.

### Relationship between `cli.rs` and `main.rs`

- **`cli.rs`**: This file is responsible for handling user input, specifically the input from the command line (`arx init --AppType webapp`). When you run the command `arx init --AppType webapp`, `cli.rs` interprets and parses the options provided by the user (in this case, `webapp` as the application type) and passes the information to the `application` via its output.
    
- **`main.rs`**: This file is the core of the application, using the information provided by `cli.rs` to configure the system. Essentially, `main.rs` takes the values collected by `cli.rs` from the user and, based on those, executes the necessary logic to initialize the system and properly configure the application. Specifically, `main.rs` uses the configuration passed from `cli.rs` to launch the `CoreSystem`.
    

### Project Creation Process with the CLI

Imagine running the command `arx init --AppType webapp` from the terminal. Here's a chain of events that occur:

1. **Parsing CLI Arguments in `cli.rs`**:
    
    - When you write `arx init --AppType webapp`, the command is passed to `cli.rs`, which parses it and determines that it is an initialization for a `webapp` type application.
    - `cli.rs` captures this input (`webapp`) and passes it as a structure (a `CoreConfig` object) to `main.rs`.
2. **Application Configuration in `main.rs`**:
    
    - Once `cli.rs` has passed the necessary information to `main.rs`, the latter creates an instance of the `CoreSystem` using the provided configurations.
    - These configurations are used to properly set up the application (`webapp`, `api_backend`, etc.).
3. **Project Structure Generation**:
    
    - One of the functions in `main.rs` (or a specific module of the framework) will generate the necessary files for the project structure based on the selected application type. For example, for a `WebApp`, specific files like `routes.rs`, `api_server.rs`, and so on are generated.
4. **Initialization and Launch**:
    
    - Once the project structure is generated, the system starts with the configured parameters, potentially launching a server (if it’s an API or WebApp) or setting up the modules for other types of applications like `desktop` or `embedded`.

# src/`cli.rs`
Through the CLI, the following aspects will be customized:

- Application type.
- The setup of the `.toml` file to include only the necessary modules based on the selected application type.
- Number of threads for the core system.
- Memory configurations such as pool size and buffer size.


```Rust
use clap::{Parser, Subcommand};
use crate::config::global_config::ApplicationType; 

/// CLI for ArxFramework
#[derive(Parser)]
#[command(name = "Arx")]
#[command(about = "CLI for the Arx framework", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Supported commands by Arx
#[derive(Subcommand)]
pub enum Commands {
    /// Initializes a new project with a specific application type
    Init {
        /// The application type to initialize (WebApp, ApiBackend, DesktopApp, etc.)
        #[arg(short, long)]
        app_type: ApplicationType, // Inheriting from config
    },
    /// Displays version information of the framework
    Version, // Implements Version
    Help, // Implements Help
}

/// Parses arguments and returns the CLI configuration
pub fn parse_arguments() -> Result<Cli, clap::Error> {
    Cli::try_parse()
}

```


# src/`main.rs`

```Rust
use crate::core_system::CoreSystem;
use crate::cli::{parse_arguments, Cli, Commands}; // Ensure the CLI is imported
use crate::config::global_config::ApplicationType;
use log::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logging system
    monitoring::logger::setup_logging().expect("Error initializing the logging system");

    // Parse the arguments passed from the CLI
    let cli_args = parse_arguments().expect("Error parsing CLI arguments");

    // Handle the Init command
    let config = match cli_args.command {
        Commands::Init { app_type } => {
            // Initialization based on the chosen application type
            match app_type {
                ApplicationType::WebApp => {
                    info!("Initializing WebApp project");
                    // Generate configuration for a WebApp
                    CoreConfig {
                        app_type: ApplicationType::WebApp,
                        max_threads: 4, // example configuration
                    }
                }
                ApplicationType::ApiBackend => {
                    info!("Initializing ApiBackend project");
                    CoreConfig {
                        app_type: ApplicationType::ApiBackend,
                        max_threads: 8, // example
                    }
                }
                ApplicationType::DesktopApp => {
                    info!("Initializing DesktopApp project");
                    CoreConfig {
                        app_type: ApplicationType::DesktopApp,
                        max_threads: 2, // example
                    }
                }
                ApplicationType::AutomationScript => {
                    info!("Initializing AutomationScript project");
                    CoreConfig {
                        app_type: ApplicationType::AutomationScript,
                        max_threads: 2, // example
                    }
                }
                ApplicationType::EmbeddedSystem => {
                    info!("Initializing EmbeddedSystem project");
                    CoreConfig {
                        app_type: ApplicationType::EmbeddedSystem,
                        max_threads: 1, // example
                    }
                }
            }
        }
        // Implementation for Version and Help
        Commands::Version => {
            println!("ArxFramework version 1.0.0");
            return Ok(()); // Exit the application
        }
        Commands::Help => {
            println!("Usage example: arx init --AppType webapp");
            return Ok(());
        }
    };

    // Initialize the CoreSystem with the obtained configuration
    let core_system = CoreSystem::new(config).expect("Error initializing the Core System");

    // Run the core system
    core_system.run()?;

    Ok(())
}

```

#### Customization: 

##### Max Threads
Customizing `max_threads` is crucial for optimizing your application’s performance based on its type and the resources available. The `max_threads` parameter controls the maximum number of threads that your application will use to process tasks concurrently. Here are some important criteria to consider when deciding how to customize it:

1. **Application Type**: Different types of applications will have different thread requirements.
    
    - For instance, a **WebApp** typically needs to handle multiple concurrent requests, so it would benefit from a higher number of threads, such as 4 or more.
    - On the other hand, **Embedded Systems** are usually resource-constrained, so they should have fewer threads, often limited to just 1, to avoid exhausting the system's resources.
2. **System Resources**: The hardware on which the application is running plays a significant role.
    
    - If the system has multiple CPU cores and ample memory, allocating more threads can help improve performance.
    - On systems with limited CPU power or memory, like low-power devices, it's important to limit the thread count to avoid performance degradation caused by resource exhaustion.
3. **Concurrency Requirements**: Depending on how many tasks your application handles concurrently, you may need to increase the number of threads.
    
    - For example, an application serving many API requests or performing numerous I/O-bound tasks will benefit from more threads.
    - However, if your application performs mostly sequential processing, increasing the thread count might not provide any performance gains.
4. **Performance Testing**: It is always advisable to benchmark your application with different thread configurations to find the optimal number.
    
    - Be aware that setting too many threads can lead to performance issues due to the overhead of context switching. Performance testing helps find the sweet spot.
5. **Use Case**: The specific use case of your application also plays a role in deciding the thread count.
    
    - Compute-intensive applications, such as **Machine Learning** tasks or **Automation scripts**, can benefit from parallelism provided by multiple threads.
    - For other use cases that rely on tasks being completed in a specific order, fewer threads might be more effective.

In summary, the `max_threads` value should be tailored based on the application's needs and the environment it runs in. For a **WebApp**, setting `max_threads = 4` may offer a good balance for handling traffic, while an **Embedded System** might only need `max_threads = 1` to avoid overburdening the system.