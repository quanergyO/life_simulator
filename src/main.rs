#[cfg(feature = "desktop")]
use life_simulator::desktop::ui::run_gui;

#[cfg(feature = "cli")]
use life_simulator::cli::Cli;

fn main() {
    #[cfg(feature = "desktop")]
    {
        run_gui();
    }

    #[cfg(feature = "cli")]
    {
        let mut cli = Cli::new();
        cli.run();
    }
}
