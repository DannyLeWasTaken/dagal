/// This serves as an abstraction to create opaque type safe [Handle] which MIGHT
/// be backed by [Storage]. [Handle] can become out of date if it's location in storage
/// has been overwritten (i.e. resource backed by handle has been mutated).
use std::marker::PhantomData;

/// An opaque [Handle<T>] representing a resource state that MIGHT be backed in [Storage<T>]
pub struct Handle<T> {
    /// Unique identifier of the [Handle]
    /// Indicates it index into the `resource` member of [Storage]
    identifier: u64,
    /// Indicates the revision of the [Handle]
    /// to allow for resources to be outdated
    revision: u64,
    /// Type safety purposes
    phantom_marker: PhantomData<T>,
}

/// Manages [Handle<T>] and backs their items they MIGHT represent in storage
pub struct Storage<T> {
    /// Where all resources are stored in
    storage: Vec<T>,
    /// Keeps track of the latest version of [Handle] to allow for rejection of old [Handle]
    revisions: Vec<u64>,
    /// Type safety purposes
    phantom_marker: PhantomData<T>,
}

impl<T> Storage<T> {
    /// Internal use only and is used to update the revision of [Handle] in [Storage]
    fn update_revision(&mut self, index: usize) -> u64 {
        let mut revision = self.revisions.get(index);
        if let Some(revision) = revision {
            self.revisions.insert(index, revision + 1);
            *self.revisions.get(index).unwrap()
        } else {
            self.revisions.insert(index, 0);
            0
        }
    }

    /// Add a [Vec<T>] of resources into [Storage] and get the subsequent [Vec] of
    /// their associated [Handle]
    pub fn append(&mut self, mut resources: Vec<T>) -> Vec<Handle<T>> {
        let resource_size = resources.len();
        let start_index = self.storage.len();
        self.storage.append(&mut resources);
        (0..resource_size)
            .map(|local_index| {
                let global_index = local_index + start_index; // set offset
                Handle {
                    identifier: global_index as u64,
                    revision: self.update_revision(global_index),
                    phantom_marker: Default::default(),
                }
            })
            .collect()
    }

    /// Insert a new resource into [Storage] without overriding any [Handle]
    pub fn push_back(&mut self, resource: T) -> Handle<T> {
        self.storage.push(resource);
        let id = self.storage.len() - 1;
        let revision = self.update_revision(id);
        Handle {
            identifier: id as u64,
            revision,
            phantom_marker: Default::default(),
        }
    }

    /// Override a resource and get a new [Handle] representing the new resource
    pub fn insert(&mut self, location: usize, resource: T) -> Handle<T> {
        let revision = self.update_revision(location);
        self.storage.insert(location, resource);
        Handle {
            identifier: location as u64,
            revision,
            phantom_marker: Default::default(),
        }
    }

    /// Retrieve resource backed by [Handle]
    /// Returns [None] if the handle passed in is outdated or does not exist
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        // check if handle is out of date or not
        let storage_revision = self.revisions.get(handle.identifier as usize);
        if storage_revision.is_none() {
            return None;
        } else if let Some(storage_revision) = storage_revision {
            if *storage_revision == handle.revision {
                return self.storage.get(*storage_revision as usize);
            }
        }
        None
    }
}
