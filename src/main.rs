use structopt::clap::AppSettings;
use structopt::StructOpt;

mod drain;
use drain::DrainCli;
mod claims;
use claims::ClaimsCli;
mod ctl;
use ctl::CtlCli;
mod generate;
use generate::NewCli;
mod keys;
use keys::KeysCli;
mod par;
use par::ParCli;
mod reg;
use reg::RegCli;
mod smithy;
use smithy::{GenerateCli, LintCli, ValidateCli};
mod call;
use call::CallCli;
mod util;

/// This renders appropriately with escape characters
const ASCII: &str = "
                               _____ _                 _    _____ _          _ _
                              / ____| |               | |  / ____| |        | | |
 __      ____ _ ___ _ __ ___ | |    | | ___  _   _  __| | | (___ | |__   ___| | |
 \\ \\ /\\ / / _` / __| '_ ` _ \\| |    | |/ _ \\| | | |/ _` |  \\___ \\| '_ \\ / _ \\ | |
  \\ V  V / (_| \\__ \\ | | | | | |____| | (_) | |_| | (_| |  ____) | | | |  __/ | |
   \\_/\\_/ \\__,_|___/_| |_| |_|\\_____|_|\\___/ \\__,_|\\__,_| |_____/|_| |_|\\___|_|_|

A single CLI to handle all of your wasmCloud tooling needs
";

#[derive(Debug, Clone, StructOpt)]
#[structopt(global_settings(&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands, AppSettings::DisableHelpSubcommand]),
            name = "wash",
            about = ASCII)]
struct Cli {
    #[structopt(flatten)]
    command: CliCommand,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, StructOpt)]
enum CliCommand {
    /// Invoke a wasmCloud actor
    #[structopt(name = "call")]
    Call(CallCli),
    /// Generate and manage JWTs for wasmCloud actors
    #[structopt(name = "claims")]
    Claims(Box<ClaimsCli>),
    /// Interact with a wasmCloud control interface
    #[structopt(name = "ctl")]
    Ctl(CtlCli),
    /// Manage contents of local wasmCloud caches
    #[structopt(name = "drain")]
    Drain(DrainCli),
    /// Generate code from smithy IDL files
    #[structopt(name = "gen")]
    Gen(GenerateCli),
    /// Utilities for generating and managing keys
    #[structopt(name = "keys", aliases = &["key"])]
    Keys(KeysCli),
    /// Create a new project from template
    #[structopt(name = "new")]
    New(NewCli),
    /// Create, inspect, and modify capability provider archive files
    #[structopt(name = "par")]
    Par(ParCli),
    /// Interact with OCI compliant registries
    #[structopt(name = "reg")]
    Reg(RegCli),
    /// Perform lint checks on smithy models
    #[structopt(name = "lint")]
    Lint(LintCli),
    /// Perform validation checks on smithy models
    #[structopt(name = "validate")]
    Validate(ValidateCli),
}

#[tokio::main]
async fn main() {
    if env_logger::try_init().is_err() {}
    let cli = Cli::from_args();

    let res = match cli.command {
        CliCommand::Call(callcli) => call::handle_command(callcli.command()).await,
        CliCommand::Claims(claimscli) => claims::handle_command(claimscli.command()).await,
        CliCommand::Ctl(ctlcli) => ctl::handle_command(ctlcli.command()).await,
        CliCommand::Drain(draincmd) => drain::handle_command(draincmd.command()),
        CliCommand::Gen(generate_cli) => smithy::handle_gen_command(generate_cli),
        CliCommand::Keys(keyscli) => keys::handle_command(keyscli.command()),
        CliCommand::New(newcli) => generate::handle_command(newcli.command()),
        CliCommand::Par(parcli) => par::handle_command(parcli.command()).await,
        CliCommand::Reg(regcli) => reg::handle_command(regcli.command()).await,
        CliCommand::Lint(lint_cli) => smithy::handle_lint_command(lint_cli).await,
        CliCommand::Validate(validate_cli) => smithy::handle_validate_command(validate_cli).await,
    };

    std::process::exit(match res {
        Ok(out) => {
            println!("{}", out);
            0
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    })
}
