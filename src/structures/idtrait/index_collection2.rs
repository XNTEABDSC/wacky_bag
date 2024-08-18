

pub trait IndexCollection2<TValue> :
    IndexCollection<TValue,TIndex=Self::TIndex>+
    Index<Self::TIndex,Output = TValue>+
    IndexMut<Self::TIndex,Output = TValue>
{
    type TIdOf:IdOf<TValue>;
    type TIndex=<TIdOf as IdOf<TValue>>::TIdStruct;
    fn get(&self, index:Self::TIdOf)->&TValue{
        self.get(index.id())
    }
    fn get_mut(&mut self, index:Self::TIdOf)->&mut TValue{
        self.get_mut(index.id())
    }
    fn add(&mut self,index:Self::TIdOf,value:TValue){
        self.add(index.id(), value)
    }
    fn remove(&mut self,index:Self::TIndex){
        self.remove(index.id())
    }
    fn index(&self, index:Self::TIndex)->&TValue{
        return self.get(index.id());
    }
    fn index_mut(&mut self, index:Self::TIndex)->&mut TValue{
        return self.get_mut(index.id());
    }
}