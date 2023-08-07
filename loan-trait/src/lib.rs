pub trait GetStructFieldsTrait {
    fn get_fields() -> Vec<String>;
}

pub trait ToVecTrait {
    fn to_vec(&self) -> Vec<String>;
}

impl<T: ToVecTrait> ToVecTrait for Vec<T> {
    fn to_vec(&self) -> Vec<String> {
        self.iter().flat_map(|item| item.to_vec()).collect()
    }
}
