use std::{mem::transmute, ops::{Deref, Range}};

use crate::{structures::n_dim_array::{n_dim_index::NDimIndex, n_dim_index_operator::NDimIndexOperator, t_n_dim_indexer::TNDimIndexer}, traits::scope_no_ret::ThreadScopeCreator};


pub trait TNDimArray<const DIM: usize, T> {
    fn lens(&self) -> impl Deref<Target = [Range<isize>; DIM]>;

    fn get(&self, indexes: &NDimIndex<DIM>) -> Option<&T>;
    fn get_mut(&mut self, indexes: &NDimIndex<DIM>) -> Option<&mut T>;

    fn for_each<'env, Func>(&'env self, func: &Func)
    where
        Func: Fn(&'env T, NDimIndex<DIM>),
        T: 'env;

    fn for_each_mut<'env, Func>(&'env mut self, func: &Func)
    where
        Func: Fn(&'env mut T, NDimIndex<DIM>),
        T: 'env;

    // fn for_each_mut<Func>(&mut self, func: &Func)
    // where
    //     Func: for<'scope> FnMut(&'scope mut T, &'scope NDimIndex<DIM>);

    fn for_each_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope T, NDimIndex<DIM>) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync;

    fn for_each_mut_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env mut self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope mut T, NDimIndex<DIM>) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync;

    // fn for_each_mut_parallel<'ext_env, Func, TScopeCreator>(
    //     &'ext_env mut self,
    //     func: &Func,
    //     scope_creator: &TScopeCreator,
    // ) where
    //     Func: for<'scope> Fn(&'scope mut T, &'scope NDimIndex<DIM>) + 'ext_env + Sync + Send,
    //     TScopeCreator: ThreadScopeCreator + Sync,
    //     T: Send + Sync;
}


#[derive(Debug)]
pub struct NDimArrayGetWithNeiborhoodsResult<const DIM: usize, T, TNeiborhood> {
    pub cur: T,
    pub neiborhoods: [((TNeiborhood, NDimIndex<DIM>), (TNeiborhood, NDimIndex<DIM>)); DIM],
}

pub trait TNDimArrayGetWithNeiborhoods<'a, const DIM: usize, T>: TNDimArray<DIM, T> {
    type TIndexer: Deref<Target: TNDimIndexer<DIM>> + 'a;
    fn get_with_neiborhoods_generic<ForNeiborhood, NeiborhoodResult>(
        &'a self,
        index: &NDimIndex<DIM>,
        for_neiborhood: ForNeiborhood,
    ) -> Option<NDimArrayGetWithNeiborhoodsResult<DIM, &'a T, NeiborhoodResult>>
    where
        ForNeiborhood: Fn(
            &Self,
            &mut NDimIndexOperator<DIM, Self::TIndexer>,
            usize,
            bool,
            bool,
        ) -> NeiborhoodResult;

    fn get_mut_with_neiborhoods_generic<ForNeiborhood, NeiborhoodResult>(
        &'a mut self,
        index: &NDimIndex<DIM>,
        for_neiborhood: ForNeiborhood,
    ) -> Option<NDimArrayGetWithNeiborhoodsResult<DIM, &'a mut T, NeiborhoodResult>>
    where
        ForNeiborhood: FnMut(
            &mut Self,
            &mut NDimIndexOperator<DIM, Self::TIndexer>,
            usize,
            bool,
            bool,
        ) -> NeiborhoodResult;

    fn get_with_neiborhoods(
        &'a self,
        index: &NDimIndex<DIM>,
    ) -> Option<NDimArrayGetWithNeiborhoodsResult<DIM, &'a T, Option<&'a T>>> {
        self.get_with_neiborhoods_generic(index, |s, i, _, _, c| {
            if c {
                None
            } else {
                unsafe { transmute(s.get(i.get())) }
            }
        })
    }

    fn get_mut_with_neiborhoods_mut(
        &'a mut self,
        index: &NDimIndex<DIM>,
    ) -> Option<NDimArrayGetWithNeiborhoodsResult<DIM, &'a mut T, Option<&'a mut T>>> {
        self.get_mut_with_neiborhoods_generic(index, |s, i, _, _, c| {
            if c {
                None
            } else {
                unsafe { transmute(s.get_mut(i.get())) }
            }
        })
    }

    fn get_mut_with_neiborhoods(
        &'a mut self,
        index: &NDimIndex<DIM>,
    ) -> Option<NDimArrayGetWithNeiborhoodsResult<DIM, &'a mut T, Option<&'a T>>> {
        self.get_mut_with_neiborhoods_generic(index, |s, i, _, _, c| {
            if c {
                None
            } else {
                unsafe { transmute(s.get(i.get())) }
            }
        })
    }

    fn get_with_neiborhoods_loop(
        &'a self,
        index: &NDimIndex<DIM>,
    ) -> Option<NDimArrayGetWithNeiborhoodsResult<DIM, &'a T, Option<&'a T>>> {
        self.get_with_neiborhoods_generic(index, |s, i, _, _, _| unsafe {
            transmute(s.get(i.get()))
        })
    }

    fn get_mut_with_neiborhoods_loop(
        &'a mut self,
        index: &NDimIndex<DIM>,
    ) -> Option<NDimArrayGetWithNeiborhoodsResult<DIM, &'a mut T, Option<&'a T>>> {
        self.get_mut_with_neiborhoods_generic(index, |s, i, _, _, _| unsafe {
            transmute(s.get(i.get()))
        })
    }
}

pub trait TNDimArrayParallelIterPair<const DIM: usize, T>: TNDimArray<DIM, T> {
    fn iter_pair_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope T, &'scope T, usize) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync;

    fn iter_pair_mut_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env mut self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope mut T, &'scope mut T, usize) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync;
}

pub trait TNDimArrayIterPair<const DIM: usize, T>: TNDimArray<DIM, T> {
    fn iter_pair<'ext_env, Func>(
        &'ext_env self,
        func: &Func,
    ) where
        Func: for<'scope> Fn(&'scope T, &'scope T, usize) + 'ext_env + Sync + Send,
        T: Send + Sync;

    fn iter_pair_mut<'ext_env, Func>(
        &'ext_env mut self,
        func: &Func,
    ) where
        Func: for<'scope> Fn(&'scope mut T, &'scope mut T, usize) + 'ext_env + Sync + Send,
        T: Send + Sync;
}