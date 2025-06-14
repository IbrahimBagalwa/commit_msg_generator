use clap::{ Arg, Command };
use git2::Repository;

mod commit_generator;
mod get_git_diff;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let matches = Command::new("Commit message suggester")
        .version("0.0.1")
        .author("Ibrahim Bagalwa, <ibrahim.bagalwa.dev@gmail.com>")
        .about("Suggests commit message based on git diff using AI")
        .arg_required_else_help(true)
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Path to the git repository")
                .value_parser(clap::value_parser!(String))
        )
        .get_matches();

    let repo_path = matches.get_one::<String>("path").map(String::as_str).unwrap_or(".");
    let repo = Repository::open(repo_path).expect("Failed to open repository");
    let diff = get_git_diff::get_git_diff(&repo);
    let commit_message = commit_generator::generate_commit_message(diff).await;

    println!("\x1b[32mSuggested Commit Message:\x1b[0m\n\x1b[34m{}\x1b[0m", commit_message);
}
