use dam1::layout::HUB_FILES;
use hf_hub::api::sync::ApiBuilder;
use hf_hub::{Repo, RepoType};
use patterns::because;
use std::path::{Path, PathBuf};

pub fn pulled(repo_id: &str, revision: &str) -> Result<PathBuf, String> {
    let api = ApiBuilder::from_env().build().map_err(|e| e.to_string())?;
    let repo = api.repo(Repo::with_revision(repo_id.to_string(), RepoType::Model, revision.to_string()));
    let files = HUB_FILES
        .iter()
        .map(|name| repo.get(name).map_err(|e| format!("{repo_id} {name}: {e}")))
        .collect::<Result<Vec<PathBuf>, String>>()?;
    files.iter().find_map(|file| file.parent().map(Path::to_path_buf)).ok_or_else(|| format!("{repo_id}: the layout names no files"))
}
because!(
    pulled,
    "a model's files fetched from the hub into the local Hugging Face cache, or taken from the cache when already there, with a signed-in user's token for a private repository, returning the folder they sit in, which the loader reads like any other folder"
);
