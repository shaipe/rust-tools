pub trait TModel
{
    fn name(&self) -> String;

    fn parent<T>() -> Option<T> where T: std::default::Default + TModel;
}

pub struct Object {}
impl TModel for Object {
    fn name(&self) -> String {
        "".to_owned()
    }

    fn parent<T>() -> Option<T> where T: std::default::Default + TModel {
        None
    }
}

impl std::default::Default for Object {
    fn default() -> Self {
        Object{}
    }
}

pub struct Model {}

impl TModel for Model {
    fn name(&self) -> String {
        "name".to_owned()
    }

    fn parent<T: std::default::Default + TModel>() -> Option<T> {
        Some(T::default())
    }
}

