use super::*;
use std::io::Read;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Meta {
    created: u64,
}

const REVISIONS_DIR_NAME: &str = "_revisions";

pub struct FileBook {
    root: String,
    pages: Vec<Page>,
    revisions: std::collections::HashMap<String, Vec<Revision>>,
}

impl FileBook {
    pub fn new<S>(root: S) -> Self
    where
        S: Into<String>,
    {
        let root = root.into();

        let mut book = Self {
            root: root.clone(),
            pages: vec![],
            revisions: std::collections::HashMap::new(),
        };

        book.walk(root).unwrap();
        book
    }

    fn walk<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        for entry in path.as_ref().read_dir()? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }

            if entry.file_name() == REVISIONS_DIR_NAME {
                let (page, revisions) = self.read_page(entry.path())?;
                self.revisions.insert(page.path.clone(), revisions);
                self.pages.push(page);
            }
            self.walk(entry.path())?
        }
        Ok(())
    }

    fn read_page<P>(&self, path: P) -> Result<(Page, Vec<Revision>)>
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref();

        let mut created = std::u64::MAX;
        let mut updated = 0;

        let mut revisions = vec![];

        for entry in path.read_dir()? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                continue;
            }

            let entry_path = entry.path();
            if entry_path.extension().unwrap().to_str().unwrap() != "md" {
                continue;
            }

            let meta_path = entry_path.parent().unwrap().join(
                entry_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".md", ".json"),
            );

            if !meta_path.exists() {
                return Err(InternalError::new("metadata not exists"));
            }

            let meta = match serde_json::from_reader::<_, Meta>(std::fs::File::open(meta_path)?) {
                Err(e) => return Err(InternalError::wrap(e, "failed to decode metadata")),
                Ok(o) => o,
            };

            let mut content = String::new();
            std::fs::File::open(entry_path)?.read_to_string(&mut content)?;

            created = std::cmp::min(meta.created, created);
            updated = std::cmp::max(meta.created, updated);

            revisions.push(Revision {
                content: content,
                created: meta.created,
            })
        }

        revisions.sort_by_key(|x| x.created);

        Ok((
            Page {
                created: created,
                updated: updated,

                path: path
                    .strip_prefix(&self.root)?
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            },
            revisions,
        ))
    }
}

impl Book for FileBook {
    fn get_all_pages(&self) -> Result<Vec<Page>> {
        Ok(self.pages.clone())
    }
    fn get_all_revisions(&self, path: &str) -> Result<Vec<Revision>> {
        Ok(self.revisions.get(path).ok_or("no such page")?.clone())
    }
    fn put_revision(&self, _path: &str, _content: &str) -> Result<(Page, Revision)> {
        unimplemented!();
    }
}
