// use std::any::Any;
// // use std::error::Error;
// use std::fmt::Debug;
// use std::path::PathBuf;
// use anyhow::Result;
// use clap::{crate_version, Parser, Subcommand};
// use crate::commands::{ClusterAction, ClusterSubcommand};
// use crate::commands::{SystemAction, SystemSubcommand};
// use crate::commands::{UserAction, UsersSubcommand};
// // use clap::{FromArgMatches, Parser, Subcommand as ClapSubcommand};
// 
// /// Simple program to manage personal cli application
// // #[derive(Parser, Debug)]
// // #[command(author, version, about, long_about = "Long about of an command")]
// // #[clap(
// //     version = "1.0",
// //     author = "Yuri Krupnik",
// //     about = "Manage users, projects, and systems"
// // )]
// // pub struct Args {
// //     /// Subcommand Name
// //     #[clap(subcommand)]
// //     subcommand: Subcommand,
// // }
// // 
// // #[derive(Parser, Debug)]
// // pub enum Subcommand {
// //     Users(UsersSubcommand),
// //     Systems(SystemSubcommand),
// //     Cluster(ClusterSubcommand),
// // }
// 
// // pub async fn create_api(command: String) -> std::io::Result<Child> {
// //     println!("{command} PASSED!!");
// //     let child = Command::new("sh").arg("-c").arg(&command).spawn()?;
// //
// //     Ok(child)
// // }
// // use futures::{StreamExt, TryStreamExt};
// // use k8s_openapi::api::apps::v1::Deployment;
// // use k8s_openapi::api::core::v1::Pod;
// // use kube::{
// //     api::{Api, ListParams, PostParams, ResourceExt},
// //     Client, Config,
// // };
// // async fn list_pods() -> Result<Api<Pod>, Box<dyn std::error::Error>> {
// //     let client = Client::try_default().await?;
// //     // Read pods in the configured namespace into the typed interface from k8s-openapi
// //     let pods: Api<Pod> = Api::default_namespaced(client.clone());
// //     let deployment: Api<Deployment> = Api::default_namespaced(client.clone());
// //     let deployments = deployment.list(&ListParams::default()).await?;
// //     let list = pods.list(&ListParams::default()).await?;
// //     for p in list {
// //         println!("found pod {}", p.name_any());
// //     }
// //     for d in deployments {
// //         println!("found pod {}", d.name_any());
// //     }
// //
// //     Ok(pods)
// // }
// 
// fn handle_read() {
//     println!("calling my script1!!");
//     run_command_with_spawn("kubectx");
// }
// 
// use std::process::{Child, Command};
// pub fn run_command_with_spawn(command: &str) -> Child {
//     let child = Command::new("sh")
//         .arg("-c")
//         .arg(command)
//         // .output()?;
//         .spawn()
//         .expect("command failed to run");
//     child
// }
// 
// // Modify run_command to return a Result<ExitStatus, Box<dyn Error>> instead of Child
// // fn run_command_with_statuses_blocking(command: &str) -> Result<Child, Box<dyn Error>> {
// //     let status = Command::new(command)
// //         // .arg("-c")
// //         // .arg(command)
// //         .spawn()?; // This will block until the command completes
// //     Ok(status)
// // }
// 
// // #[derive(Debug, Deserialize, Serialize)]
// // struct HelmSearchResult {
// //     // Define the fields you need based on the JSON structure
// //     // For example, if the JSON has a field named "name", you can add it here.
// //     name: String,
// //     version: String,
// //     app_version: String,
// //     description: String,
// //     // Add other fields as needed.
// // }
// 
// fn default_system() -> String {
//     let arch = if cfg!(target_arch = "aarch64") {
//         "aarch64"
//     } else if cfg!(target_arch = "x86_64") {
//         "x86_64"
//     } else {
//         "unknown architecture"
//     };
// 
//     let os = if cfg!(target_os = "linux") {
//         "linux"
//     } else if cfg!(target_os = "windows") {
//         "windows"
//     } else if cfg!(target_os = "macos") {
//         "darwin" // macOS is referred to as "darwin" in target triples
//     } else {
//         "unknown OS"
//     };
//     format!("{arch}-{os}")
// }
// 
// #[derive(Parser)]
// #[command(
// color = clap::ColorChoice::Auto,
// dont_delimit_trailing_values = true,
// about = format!("https://devenv.sh {}: Fast, Declarative, Reproducible, and Composable Developer Environments", crate_version!())
// )]
// struct Cli {
//     #[arg(short, long, help = "Enable debug log level.")]
//     verbose: bool,
// 
//     #[arg(short = 'j', long, help = "Maximum number of Nix builds at any time.", default_value_t = max_jobs())]
//     max_jobs: u8,
// 
//     #[arg(
//     short = 'j',
//     long,
//     help = "Maximum number CPU cores being used by a single build..",
//     default_value = "2"
//     )]
//     cores: u8,
// 
//     #[arg(short, long, default_value_t = default_system())]
//     system: String,
// 
//     #[arg(short, long, help = "Relax the hermeticity of the environment.")]
//     impure: bool,
// 
//     // TODO: --no-clean?
//     #[arg(
//     short,
//     long,
//     num_args = 0..,
//     value_delimiter = ',',
//     help = "Ignore existing environment variables when entering the shell. Pass a list of comma-separated environment variables to let through."
//     )]
//     clean: Option<Vec<String>>,
// 
//     #[arg(short = 'd', long, help = "Enter Nix debugger on failure.")]
//     nix_debugger: bool,
// 
//     #[arg(
//     short,
//     long,
//     num_args = 2,
//     value_delimiter = ' ',
//     help = "Pass additional options to nix commands, see `man nix.conf` for full list."
//     )]
//     nix_option: Vec<String>,
// 
//     #[arg(
//     short,
//     long,
//     num_args = 2,
//     value_delimiter = ' ',
//     help = "Override inputs in devenv.yaml."
//     )]
//     override_input: Vec<String>,
// 
//     #[command(subcommand)]
//     command: Commands,
// }
// 
// #[derive(Subcommand, Clone)]
// enum Commands {
//     #[command(about = "Scaffold devenv.yaml, devenv.nix, .gitignore and .envrc.")]
//     Init { target: Option<PathBuf> },
// 
//     #[command(about = "Activate the developer environment. https://devenv.sh/basics/")]
//     Shell {
//         cmd: Option<String>,
//         args: Vec<String>,
//     },
// 
//     #[command(about = "Update devenv.lock from devenv.yaml inputs. http://devenv.sh/inputs/")]
//     Update { name: Option<String> },
// 
//     #[command(
//     about = "Search for packages and options in nixpkgs. https://devenv.sh/packages/#searching-for-a-file"
//     )]
//     Search { name: String },
// 
//     #[command(
//     alias = "show",
//     about = "Print information about this developer environment."
//     )]
//     Info {},
// 
//     #[command(about = "Start processes in the foreground. https://devenv.sh/processes/")]
//     Up {
//         #[arg(help = "Start a specific process.")]
//         process: Option<String>,
// 
//         #[arg(short, long, help = "Start processes in the background.")]
//         detach: bool,
//     },
// 
//     Processes {
//         #[command(subcommand)]
//         command: ProcessesCommand,
//     },
// 
//     #[command(about = "Run tests. http://devenv.sh/tests/", alias = "ci")]
//     Test {
//         #[arg(short, long, help = "Don't override .devenv to a temporary directory.")]
//         dont_override_dotfile: bool,
//     },
// 
//     Container {
//         #[arg(short, long)]
//         registry: Option<String>,
// 
//         #[arg(long, hide = true)]
//         copy: bool,
// 
//         #[arg(long, hide = true)]
//         docker_run: bool,
// 
//         #[arg(short, long)]
//         copy_args: Vec<String>,
// 
//         #[arg(hide = true)]
//         name: Option<String>,
// 
//         #[command(subcommand)]
//         command: Option<ContainerCommand>,
//     },
// 
//     Inputs {
//         #[command(subcommand)]
//         command: InputsCommand,
//     },
// 
//     #[command(
//     about = "Deletes previous shell generations. See http://devenv.sh/garbage-collection"
//     )]
//     Gc {},
// 
//     #[command(about = "Build any attribute in devenv.nix.")]
//     Build {
//         #[arg(num_args=1..)]
//         attributes: Vec<String>,
//     },
// 
//     #[command(about = "Print the version of devenv.")]
//     Version {},
// 
//     #[clap(hide = true)]
//     Assemble,
// 
//     #[clap(hide = true)]
//     PrintDevEnv {
//         #[arg(short, long)]
//         json: bool,
//     },
// 
//     #[clap(hide = true)]
//     GenerateJSONSchema,
// }
// 
// #[derive(Subcommand, Clone)]
// #[clap(about = "Start or stop processes.")]
// enum ProcessesCommand {
//     #[command(alias = "start")]
//     Up {
//         process: Option<String>,
// 
//         #[arg(short, long)]
//         detach: bool,
//     },
// 
//     #[command(alias = "stop")]
//     Down {},
//     // TODO: Status/Attach
// }
// 
// #[derive(Subcommand, Clone)]
// #[clap(about = "Build, copy, or run a container. https://devenv.sh/containers/")]
// enum ContainerCommand {
//     #[clap(about = "Build a container.")]
//     Build { name: String },
// 
//     #[clap(about = "Copy a container.")]
//     Copy { name: String },
// 
//     #[clap(about = "Run a container.")]
//     Run { name: String },
// }
// 
// // use 
// 
// #[derive(Subcommand, Clone)]
// #[clap(about = "Add an input to devenv.yaml. https://devenv.sh/inputs/")]
// enum InputsCommand {
//     #[clap(about = "Add an input to devenv.yaml.")]
//     Add {
//         #[arg(help = "The name of the input.")]
//         name: String,
// 
//         #[arg(
//         help = "See https://devenv.sh/reference/yaml-options/#inputsnameurl for possible values."
//         )]
//         url: String,
// 
//         #[arg(short, long, help = "What inputs should follow your inputs?")]
//         follows: Vec<String>,
//     },
// }
// struct App {
//     // config: config::Config,
//     cli: Cli,
//     // has_processes: Option<bool>,
//     // container_name: Option<String>,
//     // logger: Logger,
//     // devenv_root: PathBuf,
//     // devenv_dotfile: PathBuf,
//     // devenv_dot_gc: PathBuf,
//     // devenv_home_gc: PathBuf,
//     // cachix_trusted_keys: PathBuf,
//     // cachix_caches: Option<CachixCaches>,
// }
// 
// pub fn parse_subcommand() {
//     let cli = Cli::parse();
//     // let level = if cli.verbose {
//     //     log::Level::Debug
//     // } else {
//     //     log::Level::Info
//     // };
//     // println!("Hello {:?}!", cli.subcommand.type_id());
//     // for _ in 0..cli.count {
//     // }
//     // cli.subcommand
//     let mut app = App {
//         cli,
//         // config,
//         // has_processes: None,
//         // logger,
//         // container_name: None,
//         // devenv_root,
//         // devenv_dotfile,
//         // devenv_dot_gc,
//         // devenv_home_gc,
//         // cachix_trusted_keys,
//         // cachix_caches: None
//     };
// 
//     match app.cli.command.clone() {
//         Commands::Shell { cmd, args } => {
//             println!("shell command here!!");
//             // app.shell(&cmd, args, true);
//             // run_command_with_spawn("task -a");
//             // run_command_with_spawn("cargo run -p clapper -r -- cluster read");
//         },
//         Commands::Search { name } => {
//             println!("search command here!! {name}");
//             run_command_with_spawn("nix search");
//             // run_command_with_spawn("cargo run -p clapper -r -- cluster read");
//         },
//         // App::Update { name } => {
//         //     println!("update command here!!");
//         //     // run_command_with_spawn("task -a");
//         //     // run_command_with_spawn("cargo run -p clapper -r -- cluster read");
//         // },
//         // Subcommand::Cluster(cmd) => match cmd.action {
//         //     ClusterAction::Create(dto) => {
//         //         // let ds = dto.update_from(&["--name", "test"]);
//         //         // let sd= dto.update_from_arg_matches("");
//         //         println!("create_cluster.type_id() {:?}", dto.type_id());
//         //         println!("update_cluster {dto:?} ");
//         //     }
//         //     ClusterAction::Update(dto) => {
//         //         println!("update_cluster.type_id() {:?}", dto.type_id());
//         //         println!("update_cluster {dto:?} ");
//         //     }
//         //     ClusterAction::Delete(delete_cluster) => {
//         //         println!("delete_cluster.type_id() {:?}", delete_cluster.type_id());
//         //         println!("deleteCluster {delete_cluster:?} ");
//         //         // run_command_with_spawn("just");
//         //         // run_command_with_statuses_blocking_try1("task -a");
//         //         // run_multiple_commands("ls");
//         //     }
//         //     // ClusterAction::Delete(handle_read(deleteCluster)),
//         //     ClusterAction::Read => handle_read(),
//         // },
//         // Subcommand::Systems(cmd) => match cmd.action {
//         //     SystemAction::Create(cmd) => {
//         //         // cmd:
//         //         println!("system apply here!!");
//         //         run_command_with_spawn("kubectl apply -f https://raw.githubusercontent.com/metallb/metallb/v0.13.11/config/manifests/metallb-native.yaml");
//         //     }
//         //     SystemAction::Update(_) => {
//         //         println!("system update here!!");
//         //         run_command_with_spawn("task -a");
//         //         run_command_with_spawn("cargo run -p clapper -r -- cluster read");
//         // 
//         //         // ClusterAction::Read;
//         //         // let s = create_api("task -a".to_string()).await.unwrap();
//         //     }
//         //     SystemAction::Delete(_) => {
//         //         println!("system delete here!!");
//         //         run_command_with_spawn("kubectl delete -f https://raw.githubusercontent.com/metallb/metallb/v0.13.11/config/manifests/metallb-native.yaml");
//         //     }
//         //     SystemAction::Read => {
//         //         run_command_with_spawn("task cargo:build");
//         //         // let s = CreateCluster {
//         //         //     name: "ads".to_string()
//         //         // };
//         //         // println!("{s:?}")
//         //         // ClusterAction::Create()
//         //         // CreateCluster::
//         //         // ClusterSubcommand::default().action
//         //         // Subcommand::Cluster(ClusterSubcommand);
//         //     }
//         // },
//         // Subcommand::Users(cmd) => match cmd.action {
//         //     UserAction::Create(_) => {}
//         //     UserAction::Update(_) => {}
//         //     UserAction::Delete(_) => {}
//         //     UserAction::Read => {}
//         // },
//         _ => {}
//     }
// }
// 
// impl App  {
//     fn shell(&mut self, cmd: &Option<String>, args: &[String], replace_shell: bool) -> Result<()> {
//         // let develop_args = self.prepare_shell(cmd, args)?;
//         // 
//         // let options = command::Options {
//         //     replace_shell,
//         //     ..command::Options::default()
//         // };
//         // 
//         // let develop_args = develop_args
//         //     .iter()
//         //     .map(|s| s.as_str())
//         //     .collect::<Vec<&str>>();
//         // 
//         // self.run_nix(SPECIAL_APP, &develop_args, &options)?;
//         Ok(())
//     }
// 
// }
