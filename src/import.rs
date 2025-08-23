use crate::visibility::Vis;

/// Defines an import (`use` statement).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Import {
    #[allow(dead_code)]
    line: String,

    /// Function visibility
    vis: Vis,
}

impl Import {
    /// Creates a new import.
    pub fn new(path: impl Into<String>, ty: impl Into<String>) -> Self {
        Import {
            line: format!("{}::{}", path.into(), ty.into()),
            vis: Vis::Private,
        }
    }

    /// Returns the import line.
    pub fn line(&self) -> &str {
        &self.line
    }

    /// Gets the import visibility.
    pub fn vis(&self) -> &Vis {
        &self.vis
    }

    /// Sets the import visibility.
    pub fn set_vis(&mut self, vis: impl Into<Vis>) -> &mut Self {
        self.vis = vis.into();
        self
    }

    /// Sets the import visibility.
    pub fn with_vis(mut self, vis: impl Into<Vis>) -> Self {
        self.set_vis(vis);
        self
    }

    /// Gets a mutable reference to the import visibility.
    pub fn vis_mut(&mut self) -> &mut Vis {
        &mut self.vis
    }
}
