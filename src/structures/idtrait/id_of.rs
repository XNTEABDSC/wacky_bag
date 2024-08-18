pub trait IdOf<TValue>{
    type TIdStruct;
    fn index(&self)->Self::TIdStruct;
}