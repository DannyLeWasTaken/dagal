pub trait Nameable {
    /// Set the debug name of the object
    fn set_name<T: Into<String>>(&self, name: T);
}
