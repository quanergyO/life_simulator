# Life Simulator

A Rust-based life simulator that allows users to model their financial life by tracking income, expenses, and balance over time.

## Features

- **Personal Setup**: Create a character with name, age, and current annual income
- **Expense Management**: Add expenses with different frequencies (yearly, monthly, daily) and time periods
- **Financial Simulation**: Calculate projected balance at any future age
- **Visual Analytics**: Interactive graph showing balance progression over time
- **Cross-platform Desktop GUI**: Built with egui for a native experience
- **Command-Line Interface**: Alternative CLI interface for terminal users
- **Feature-based Build System**: Choose between CLI or Desktop version at compile time

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Building from Source

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd life_simulator
   ```

2. Build the project with desired features:

   **Using Makefile (recommended):**

   **Desktop GUI (default):**
   ```bash
   make desktop          # Debug build
   make desktop-release  # Release build
   ```

   **Command-Line Interface:**
   ```bash
   make cli              # Debug build
   make cli-release      # Release build
   ```

   **Using Cargo directly:**

   **Desktop GUI (default):**
   ```bash
   cargo build --release
   # or explicitly:
   cargo build --release --features desktop
   ```

   **Command-Line Interface:**
   ```bash
   cargo build --release --features cli
   ```

3. Run the application:

   **Using Makefile:**

   **Desktop GUI:**
   ```bash
   make run-desktop
   ```

   **Command-Line Interface:**
   ```bash
   make run-cli
   ```

   **Using Cargo directly:**

   **Desktop GUI (default):**
   ```bash
   cargo run
   # or explicitly:
   cargo run --features desktop
   ```

   **Command-Line Interface:**
   ```bash
   cargo run --features cli
   ```

4. **Additional Makefile targets:**
   ```bash
   make test     # Run tests
   make clean    # Clean build artifacts
   make format   # Format code
   make check    # Check code without building
   make lint     # Run clippy linter
   ```

## Usage

### Desktop GUI Version
The application features a three-tab interface:

#### 1. Setup Tab
- Enter your name, current age, and annual income
- Click "Create Person" to initialize your simulation

#### 2. Expenses Tab
- Add various expenses with:
  - Name
  - Amount per period
  - Frequency (Yearly, Monthly, Daily)
  - Start age
  - Optional end age (leave blank for ongoing expenses)
- View all current expenses

#### 3. Simulation Tab
- Enter a target age to calculate projected balance
- View interactive graph of balance progression over time
- See detailed table of balance at each age

### CLI Version
The command-line interface provides the same functionality through an interactive terminal:

1. Enter your name, age, and income when prompted
2. Use the menu to:
   - View current status
   - Add expenses
   - Calculate balance at specific ages
   - Show balance history
   - Exit the application

## Example Scenario

1. Create a person: "John Doe", age 25, income $50,000
2. Add expenses:
   - Rent: $1,000/month starting at age 25
   - Food: $300/month starting at age 25
   - Car payment: $400/month from age 25 to 35
3. Calculate balance at age 40 to see projected financial status

## Architecture

The project follows a clean architecture pattern:

- `domain/`: Core business logic and entities
- `desktop/ui/`: GUI implementation using egui
- `cli/`: Command-line interface
- `tests/`: Unit tests for core functionality

## Feature Flags

The application supports two feature flags:

- `desktop` (default): Enables the GUI interface using egui
- `cli`: Enables the command-line interface
- Both features can be enabled together, but the application will run in desktop mode by default

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [egui](https://github.com/emilk/egui) for the GUI framework
- Uses [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) for the application framework
- Uses [clap](https://github.com/clap-rs/clap) for CLI parsing