use clap::{Parser, Subcommand};
use glob::glob;
use std::collections::HashSet;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    ///// Optional name to operate on
    //name: Option<String>,

    ///// Sets a custom config file
    //#[arg(short, long, value_name = "FILE")]
    //config: Option<PathBuf>,

    ///// Turn debugging information on
    //#[arg(short, long, action = clap::ArgAction::Count)]
    //debug: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    //// does testing things

    // Test {
    //     /// lists test values
    //     #[arg(short, long)]
    //     list: bool,
    // },
    /// C 言語のヘッダーファイルが存在するディレクトリを列挙する
    ListHeaderDirs {
        /// 検索するルートディレクトリを指定
        #[arg(index(1))]
        root: String,
    },
}

fn main() {
    let cli = Cli::parse();

    //// You can check the value provided by positional arguments, or option arguments
    //if let Some(name) = cli.name.as_deref() {
    //    println!("Value for name: {name}");
    //}

    //if let Some(config_path) = cli.config.as_deref() {
    //    println!("Value for config: {}", config_path.display());
    //}

    //// You can see how many times a particular flag or argument occurred
    //// Note, only flags can have multiple occurrences
    //match cli.debug {
    //    0 => println!("Debug mode is off"),
    //    1 => println!("Debug mode is kind of on"),
    //    2 => println!("Debug mode is on"),
    //    _ => println!("Don't be crazy"),
    //}
    //
    //// You can check for the existence of subcommands, and if found use their
    //// matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::ListHeaderDirs { root }) => {
            list_header_dirs(root);
        }
        None => {}
    }

    // Continued program logic goes here...
}

fn list_header_dirs(root: &String) {
    let mut root = String::from(root);
    root.push_str("/**/*.h");
    root = root.replace("\\", "/");
    let mut dirnames: HashSet<String> = HashSet::new();

    for entry in glob(&root).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // 親ディレクトリを取得
                if let Some(parent) = path.parent() {
                    // ディレクトリ名を取得
                    dirnames.insert(String::from(parent.to_str().unwrap()));

                } else {
                    println!("親ディレクトリが取得できませんでした");
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    for dirname in dirnames {
        println!("\"{}\",", dirname);
    }

}
