
pub trait TModel{
    fn name(&self) -> String;
}

pub struct Model {
    
}

impl TModel for Model {
    fn name(&self) -> String {
        "name".to_owned()
    }
}

impl Model {
    pub fn parent() -> impl TModel{
        Model {}
    }
}
