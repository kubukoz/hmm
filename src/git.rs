use std::path::Path;

use git2::Repository;

use crate::files::root_path;

pub(crate) fn git_commit(paths: &Vec<&Path>, message: String) -> Result<(), git2::Error> {
    let repo = Repository::open(root_path())?;

    let mut index = repo.index()?;

    paths
        .into_iter()
        .for_each(|p| index.add_path(p).expect("Couldn't add path"));

    let user = repo.signature()?;
    let tree = repo.find_tree(index.write_tree()?)?;
    let parents = &[&repo.head()?.peel_to_commit()?];

    repo.commit(Some("HEAD"), &user, &user, &message, &tree, parents)?;
    repo.checkout_head(None)?;

    Ok(())
}
