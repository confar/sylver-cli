use anyhow::Context;

use super::source::{source_from_file, Source};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FileSpec {
    pub include: Vec<String>,
}

pub trait FileSpecLoader {
    fn load(&self, spec: &FileSpec) -> anyhow::Result<Vec<Source>>;
}

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct FsFileSpecLoader {}

impl FileSpecLoader for FsFileSpecLoader {
    fn load(&self, spec: &FileSpec) -> anyhow::Result<Vec<Source>> {
        let mut result = Vec::new();

        for glob in &spec.include {
            result.extend(sources_from_glob(glob)?);
        }

        Ok(result)
    }
}

fn sources_from_glob(pattern: &str) -> anyhow::Result<Vec<Source>> {
    glob::glob(pattern)
        .context("Failed to parse glob pattern")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to evaluate glob")?
        .iter()
        .map(|p| source_from_file(p))
        .collect::<Result<_, _>>()
        .context("Failed to build source")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use maplit::hashset;
    use temp_dir::TempDir;

    use super::*;
    use crate::util::test::create_tmp_child;

    #[test]
    fn fs_file_spec_ok() {
        let d = TempDir::new().unwrap();

        let match1 = create_tmp_child(&d, "match1.ok", "content1").unwrap();
        let match2 = create_tmp_child(&d, "match2.ok", "content2").unwrap();
        let _ = create_tmp_child(&d, "nomatch.other", "nomatch_content").unwrap();

        let spec = FileSpec {
            include: vec![format!("{}/*.ok", d.path().display())],
        };

        let loaded = FsFileSpecLoader::default().load(&spec).unwrap();

        assert_eq!(
            loaded.into_iter().collect::<HashSet<Source>>(),
            hashset![
                source_from_file(&match1).unwrap(),
                source_from_file(&match2).unwrap(),
            ]
        )
    }
}
