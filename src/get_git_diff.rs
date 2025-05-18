use git2::Repository;

pub fn get_git_diff(repo: &Repository) -> String {
    let mut diff_options = git2::DiffOptions::new();
    let index = repo.index().expect("Failed to get index");

    let diff = repo
        .diff_tree_to_index(None, Some(&index), Some(&mut diff_options))
        .expect("Failed to get git diff");

    let mut diff_content = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        diff_content.push_str(std::str::from_utf8(line.content()).unwrap());
        true
    }).expect("Failed to print diff");

    diff_content
}
