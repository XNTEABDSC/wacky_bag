use std::{
    mem::transmute,
    ops::{Deref, Range},
};

use crate::{
    structures::n_dim_array::{
        dim_dir::DimDir, n_dim_index::NDimIndex, n_dim_index_operator::NDimIndexOperator,
        t_n_dim_indexer::TNDimIndexer,
    },
    traits::scope_no_ret::ThreadScopeCreator,
};

pub trait TNDimArray<const DIM: usize, T> {
    fn lens(&self) -> impl Deref<Target = [Range<isize>; DIM]>;

    fn get(&self, indexes: &NDimIndex<DIM>) -> Option<&T>;
    fn get_mut(&mut self, indexes: &NDimIndex<DIM>) -> Option<&mut T>;

    // fn for_each_mut<Func>(&mut self, func: &Func)
    // where
    //     Func: for<'scope> FnMut(&'scope mut T, &'scope NDimIndex<DIM>);

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

pub trait TNDimArrayForEach<const DIM: usize, T>: TNDimArray<DIM, T> {
    fn for_each<'env, Func>(&'env self, func: &mut Func)
    where
        Func: FnMut(&'env T, NDimIndex<DIM>),
        T: 'env;

    fn for_each_mut<'env, Func>(&'env mut self, func: &mut Func)
    where
        Func: FnMut(&'env mut T, NDimIndex<DIM>),
        T: 'env;
}

pub trait TNDimArrayForEachParallel<const DIM: usize, T>: TNDimArray<DIM, T> {
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
}

// fn pair_to_2_side<T,F>(f:F)
// ->impl Fn(T,T,usize)
// 	where F:Fn(T,T,DimDir),T:Copy
// {
// 	move |a,b,dim|{
// 		f(a,b,DimDir { dim, dir_positive: true });
// 		f(b,a,DimDir { dim, dir_positive: false });
// 	}
// }

pub trait TNDimArrayIterPair<const DIM: usize, T>: TNDimArray<DIM, T> {
    fn iter_pair<'ext_env, Func>(&'ext_env self, func: &mut Func)
    where
        Func: for<'scope> FnMut(&'scope T, &'scope T, usize),
		T:'ext_env
		;

    fn iter_pair_mut<'ext_env, Func>(&'ext_env mut self, func: &mut Func)
    where
        Func: for<'scope> FnMut(&'scope mut T, &'scope mut T, usize),
		T:'ext_env
		;

	fn iter_pair_2_side<'ext_env, Func>(&'ext_env self, func: &mut Func)
    where
        Func: for<'scope> FnMut(&'scope T, &'scope T, DimDir),
		T:'ext_env
	{
		self.iter_pair(&mut |a,b,dim|{
			func(a,b,DimDir { dim, dir_positive: true });
			func(b,a,DimDir { dim, dir_positive: false });
		});
	}

	fn iter_pair_2_side_mut<'ext_env, Func>(&'ext_env mut self, func: &mut Func)
    where
        Func: for<'scope> FnMut(&'scope mut T, &'scope mut T, DimDir),
		T:'ext_env
	{
		self.iter_pair_mut(&mut |a,b,dim|{
			func(a,b,DimDir { dim, dir_positive: true });
			func(b,a,DimDir { dim, dir_positive: false });
		});
	}
}

pub trait TNDimArrayIterPairParallel<const DIM: usize, T>: TNDimArray<DIM, T> {
    fn iter_pair_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope T, &'scope T, usize) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync;

	fn iter_pair_2_side_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope T, &'scope T, DimDir) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync{
		self.iter_pair_parallel(&|a,b,dim|{
			func(a,b,DimDir { dim, dir_positive: true });
			func(b,a,DimDir { dim, dir_positive: false });
		}, scope_creator);
	}

    fn iter_pair_mut_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env mut self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope mut T, &'scope mut T, usize) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync;

	fn iter_pair_2_side_mut_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env mut self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope mut T, &'scope mut T, DimDir) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync{
		self.iter_pair_mut_parallel(&|a,b,dim|{
			func(a,b,DimDir { dim, dir_positive: true });
			func(b,a,DimDir { dim, dir_positive: false });
		}, scope_creator);
	}
}

pub trait TNDimArrayForEachEdge<const DIM: usize, T>: TNDimArray<DIM, T> {
    fn for_each_edge<'ext_env, Func>(&'ext_env self, dim_dir: DimDir, func: &mut Func)
    where
        Func: FnMut(&'ext_env T, NDimIndex<DIM>),
        T: Send + Sync + 'ext_env;

    fn for_each_edge_mut<'ext_env, Func>(&'ext_env mut self, dim_dir: DimDir, func: &mut Func)
    where
        Func: FnMut(&'ext_env mut T, NDimIndex<DIM>),
        T: Send + Sync + 'ext_env;
}

pub trait TNDimArrayForEachEdgeParallel<const DIM: usize, T>: TNDimArray<DIM, T> {
    fn for_each_edge_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env self,
        dim_dir: DimDir,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope T, NDimIndex<DIM>) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync;

    fn for_each_edge_mut_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env mut self,
        dim_dir: DimDir,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope mut T, NDimIndex<DIM>) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync;
}
