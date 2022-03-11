pub(crate) struct UpdateResult<T> {
    pub updates: Vec<T>,
    pub kind: UpdateKind,
}

impl<T: ToCommitMessage> UpdateResult<T> {
    pub(crate) fn was_updated(&self) -> bool {
        !self.updates.is_empty()
    }
}

impl<T: ToCommitMessage + Clone> ToCommitMessage for UpdateResult<T> {
    fn to_commit_message(&self) -> String {
        let prefix = match self.kind {
            UpdateKind::Add => "Add",
            UpdateKind::Update => "Update",
        };

        let info = self
            .updates
            .clone()
            .into_iter()
            .map(|u| u.to_commit_message())
            .collect::<Vec<_>>()
            .join(", ");

        format!("(hmm) {} {}", prefix, info)
    }
}

pub trait ToCommitMessage {
    fn to_commit_message(&self) -> String;
}

pub(crate) enum UpdateKind {
    Add,
    Update,
}

#[derive(Clone)]
pub(crate) struct Add {
    pub program: String,
    pub version: Option<String>,
}

impl ToCommitMessage for Add {
    fn to_commit_message(&self) -> String {
        let version_string: String = match self.version.clone() {
            Some(v) => format!(": {}", v),
            None => "".to_string(),
        };

        format!("{}{}", self.program.as_str(), version_string)
    }
}

#[derive(Clone)]
pub(crate) struct Update {
    pub program: String,
    pub from: String,
    pub to: String,
}

impl ToCommitMessage for Update {
    fn to_commit_message(&self) -> String {
        format!("{}: {} -> {}", self.program, self.from, self.to)
    }
}
