use std::marker::PhantomData;

use super::index_collection::IndexCollection;




pub trait IdMap<TIdA:Copy,TIdB:Copy>{
    fn add(&mut self,ida:TIdA,idb:TIdB);
    fn a2b(&self,ida:TIdA)->TIdB;
    fn b2a(&self,idb:TIdB)->TIdA;
    fn remove(&mut self,ida:TIdA,idb:TIdB);
    #[inline]
    fn remove_by_ida(&mut self,ida:TIdA){
        self.remove(ida,self.a2b(ida))
    }
    #[inline]
    fn remove_by_idb(&mut self, idb:TIdB){
        self.remove(self.b2a(idb), idb)
    }
}

pub struct IdMapCollection<TIdA:Copy,TIdB:Copy,Ta2bC:IndexCollection<TIdB,TIndex=TIdA>,Tb2aC:IndexCollection<TIdA,TIndex=TIdB>>{
    a2b_collection:Ta2bC,
    b2a_collection:Tb2aC,
    phantom:PhantomData<(TIdA,TIdB)>
}

impl<TIdA:Copy,TIdB:Copy,Ta2bC:IndexCollection<TIdB,TIndex=TIdA>,Tb2aC:IndexCollection<TIdA,TIndex=TIdB>> IdMapCollection<TIdA,TIdB,Ta2bC,Tb2aC> {
    pub fn new(a2b_collection:Ta2bC,b2a_collection:Tb2aC)->IdMapCollection<TIdA,TIdB,Ta2bC,Tb2aC>{
        let res=IdMapCollection::<TIdA,TIdB,Ta2bC,Tb2aC>{
            a2b_collection,b2a_collection,phantom:PhantomData
        };
        return res;
    }
}
impl<TIdA:Copy,TIdB:Copy,Ta2bC:IndexCollection<TIdB,TIndex=TIdA>,Tb2aC:IndexCollection<TIdA,TIndex=TIdB>> 
    IdMap<TIdA,TIdB> for 
    IdMapCollection<TIdA,TIdB,Ta2bC,Tb2aC>{
        fn add(&mut self,ida:TIdA,idb:TIdB) {
            self.a2b_collection.add(ida, idb)
        }
    
        fn a2b(&self,ida:TIdA)->TIdB {
            return self.a2b_collection[ida];
        }
    
        fn b2a(&self,idb:TIdB)->TIdA {
            return self.b2a_collection[idb];
        }
    
        fn remove(&mut self,ida:TIdA,idb:TIdB) {
            self.a2b_collection.remove(ida);
            self.b2a_collection.remove(idb);
        }
}

