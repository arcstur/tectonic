// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! A Tectonic document-build workspace.
//!
//! For the time being, this is just a thin wrapper to provide access to a
//! `Document` instance. This API exists to future-proof a bit for a potential
//! world where one workspace can contain multiple documents.

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    config::PersistentConfig, ctry, document::Document, errors::Result, status::StatusBackend,
};

/// A Tectonic workspace.
#[derive(Debug)]
pub struct Workspace {
    /// The root directory of the workspace.
    root_dir: PathBuf,

    /// This workspace's document. In the future, there might be more than one.
    doc: Document,
}

impl Workspace {
    /// Get the first document in the workspace.
    ///
    /// Right now, workspaces in fact only include one document. That may change
    /// in the future.
    pub fn first_document(&self) -> &Document {
        &self.doc
    }

    /// Get the first document in the workspace, mutably.
    ///
    /// Right now, workspaces in fact only include one document. That may change
    /// in the future.
    pub fn first_document_mut(&mut self) -> &mut Document {
        &mut self.doc
    }
}

/// A type for creating a new workspace.
#[derive(Debug)]
pub struct WorkspaceCreator {
    /// The root directory of the workspace to be created.
    root_dir: PathBuf,
}

impl WorkspaceCreator {
    /// Initialize a `WorkspaceCreator` variable.
    pub fn new<P: Into<PathBuf>>(root_dir: P) -> Self {
        WorkspaceCreator {
            root_dir: root_dir.into(),
        }
    }

    /// Consume this object and attempt to create the new workspace.
    pub fn create(
        self,
        config: &PersistentConfig,
        status: &mut dyn StatusBackend,
    ) -> Result<Workspace> {
        let doc = Document::new_for_creator(&self, config, status)?;

        ctry!(
            fs::create_dir_all(&self.root_dir);
            "couldn\'t create workspace directory `{}`", self.root_dir.display()
        );

        doc.create_toml()?;

        Ok(Workspace {
            root_dir: self.root_dir,
            doc,
        })
    }

    /// Get the root directory of the workspace.
    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }
}
