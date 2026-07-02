use crate::models::component::Component;
use simsearch::SimSearch;
use uuid::Uuid;

pub struct SearchEngine {
    index: SimSearch<Uuid>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            index: SimSearch::new(),
        }
    }

    /// Add a single component to the index
    pub fn index_component(&mut self, component: &Component) {
        let searchable = format!(
            "{} {} {}",
            component.name,
            component.description,
            component.tags.join(" ")
        );
        self.index.insert(component.id, &searchable);
    }

    /// Remove a component from the index by rebuilding without it
    #[allow(dead_code)]
    pub fn remove_component(&mut self, components: &[Component], remove_id: &Uuid) {
        self.index = SimSearch::new();
        for c in components {
            if &c.id != remove_id {
                self.index_component(c);
            }
        }
    }

    /// Search for components matching a query
    pub fn search(&self, query: &str) -> Vec<Uuid> {
        if query.trim().is_empty() {
            return vec![];
        }
        self.index.search(query)
    }

    /// Rebuild the entire index from a list of components
    pub fn rebuild(&mut self, components: &[Component]) {
        self.index = SimSearch::new();
        for component in components {
            self.index_component(component);
        }
        tracing::debug!("Search index rebuilt with {} components", components.len());
    }
}
