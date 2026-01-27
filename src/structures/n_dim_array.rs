use std::{
    array,
    mem::transmute,
    ops::{Deref, Index, IndexMut, Range},
};

use crate::{
    structures::{
        n_dim_index::{NDimIndex, NDimIndexer, TNDimIndexer},
        n_dim_index_operator::NDimIndexOperator,
    },
    traits::scope_no_ret::{self, ThreadScopeCreator, ThreadScopeCreatorStd, ThreadScopeUser},
};

#[derive(Debug)]
pub struct NDimArray<TIndexer, const DIM: usize, T, Storage>
where
    TIndexer: Deref<Target: TNDimIndexer<DIM>>,
    Storage: Index<usize, Output = T> + IndexMut<usize>,
{
    values: Storage,
    n_dim_index: TIndexer,
}

// impl<TIndexer, const DIM: usize, T, Storage> NDimArray<TIndexer, DIM, T, Storage>
// where
//     TIndexer: Deref<Target: TNDimIndexer<DIM>>,
//     Storage: Index<usize, Output = T> + IndexMut<usize>,
// {
    
// }

impl<TIndexer, const DIM: usize, T> NDimArray<TIndexer, DIM, T, Vec<T>>
where
    TIndexer: Deref<Target: TNDimIndexer<DIM>>,
{
    pub fn from_fn<Fn>(n_dim_index: TIndexer, mut func: Fn) -> Self
    where
        Fn: FnMut(NDimIndex<DIM>) -> T,
    {
        let index_iter = n_dim_index.iter();
        let values = index_iter.map(|index| func(index)).collect();
        Self {
            values,
            n_dim_index,
        }
    }
}

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
    fn parallel_iter_pair<'ext_env, Func, TScopeCreator>(
        &'ext_env self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope T, &'scope T, usize) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync;

    fn parallel_iter_pair_mut<'ext_env, Func, TScopeCreator>(
        &'ext_env mut self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope mut T, &'scope mut T, usize) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync;
}

impl<TIndexer, const DIM: usize, T, Storage> NDimArray<TIndexer, DIM, T, Storage>
where
    TIndexer: Deref<Target: TNDimIndexer<DIM>>,
    Storage: Index<usize, Output = T> + IndexMut<usize>,
{
    pub fn new(n_dim_index: TIndexer, values: Storage) -> Self {
        Self {
            values,
            n_dim_index,
        }
    }

    pub fn unwrap_ref(&self) -> (&TIndexer, &Storage) {
        (&self.n_dim_index, &self.values)
    }

    pub fn unwrap_mut(&mut self) -> (&mut TIndexer, &mut Storage) {
        (&mut self.n_dim_index, &mut self.values)
    }

    pub fn n_dim_index(&self) -> &TIndexer {
        &self.n_dim_index
    }

    pub fn get_with_compressed(&self, index: usize) -> Option<&T> {
        if !self.n_dim_index.contains_compressed(index) {
            return None;
        }
        Some(self.values.index(index))
    }
    pub fn get_mut_with_compressed(&mut self, index: usize) -> Option<&mut T> {
        if !self.n_dim_index.contains_compressed(index) {
            return None;
        }
        Some(self.values.index_mut(index))
    }
    pub fn values(&self) -> &Storage {
        &self.values
    }
    pub fn values_mut(&mut self) -> &mut Storage {
        &mut self.values
    }
    pub unsafe fn get_other<'a, 'b>(&'a self, indexes: &NDimIndex<DIM>) -> Option<&'b T> {
        self.get(indexes).map(|a| unsafe { &*(a as *const T) })
    }
    pub unsafe fn get_mut_other<'a, 'b>(
        &'a mut self,
        indexes: &NDimIndex<DIM>,
    ) -> Option<&'b mut T> {
        self.get_mut(indexes)
            .map(|a| unsafe { &mut *(a as *mut T) })
    }
}

impl<TIndexer, const DIM: usize, T, Storage> TNDimArray<DIM, T>
    for NDimArray<TIndexer, DIM, T, Storage>
where
    TIndexer: Deref<Target: TNDimIndexer<DIM>>,
    Storage: Index<usize, Output = T> + IndexMut<usize>,
{
    fn lens(&self) -> impl Deref<Target = [std::ops::Range<isize>; DIM]> {
        self.n_dim_index.lens()
    }

    fn get(&self, indexes: &NDimIndex<DIM>) -> Option<&T> {
        if self.n_dim_index.contains(indexes) {
            Some(self.values.index(self.n_dim_index.compress_index(indexes)))
        } else {
            None
        }
    }
    fn get_mut(&mut self, indexes: &NDimIndex<DIM>) -> Option<&mut T> {
        if !self.n_dim_index.contains(indexes) {
            return None;
        }
        Some(
            self.values
                .index_mut(self.n_dim_index.compress_index(indexes)),
        )
    }

    fn for_each<'env, Func>(&'env self, func: &Func)
    where
        Func: Fn(&'env T, NDimIndex<DIM>),
    {
        let array: &Self = self;
        for index in array.n_dim_index.iter().enumerate() {
            if let Some(item) = array.get_with_compressed(index.0) {
                func(item, index.1);
            }
        }
    }

    fn for_each_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope T, NDimIndex<DIM>) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync,
    {
        struct A<'env, Func, TIndexer, const DIM: usize, T, Storage>
        where
            TIndexer: Deref<Target: TNDimIndexer<DIM>>,
            Storage: Index<usize, Output = T> + IndexMut<usize>,
        {
            array: &'env NDimArray<TIndexer, DIM, T, Storage>,
            func: &'env Func,
        }
        impl<'env, Func, TIndexer, const DIM: usize, T, Storage> ThreadScopeUser<'env>
            for A<'env, Func, TIndexer, DIM, T, Storage>
        where
            TIndexer: Deref<Target: TNDimIndexer<DIM>>,
            Storage: Index<usize, Output = T> + IndexMut<usize>,
            Func: for<'scope> Fn(&'scope T, NDimIndex<DIM>) + 'env + Sync + Send,
            T: Send + Sync,
        {
            fn use_scope<'scope, TScope>(self, scope: &'scope TScope) -> ()
            where
                TScope: 'scope + scope_no_ret::ThreadScope<'scope>,
                'env: 'scope,
            {
                let func = self.func;
                let array = self.array;

                array.for_each(&|t, i| {
                    scope.spawn(move || {
                        (func)(t, i);
                    });
                });
            }
        }
        scope_creator.scope(A {
            array: self,
            func: &func,
        });
    }

    fn for_each_mut<'env, Func>(&'env mut self, func: &Func)
    where
        Func: Fn(&'env mut T, NDimIndex<DIM>),
        T: 'env,
    {
        let array = self;
        for index in array.n_dim_index.iter().enumerate() {
            let item = unsafe { transmute(array.values.index_mut(index.0)) };
            func(item, index.1);
        }
    }

    fn for_each_mut_parallel<'ext_env, Func, TScopeCreator>(
        &'ext_env mut self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope mut T, NDimIndex<DIM>) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync,
    {
        struct A<'env, Func, TIndexer, const DIM: usize, T, Storage>
        where
            TIndexer: Deref<Target: TNDimIndexer<DIM>>,
            Storage: Index<usize, Output = T> + IndexMut<usize>,
        {
            array: &'env mut NDimArray<TIndexer, DIM, T, Storage>,
            func: &'env Func,
        }
        impl<'env, Func, TIndexer, const DIM: usize, T, Storage> ThreadScopeUser<'env>
            for A<'env, Func, TIndexer, DIM, T, Storage>
        where
            TIndexer: Deref<Target: TNDimIndexer<DIM>>,
            Storage: Index<usize, Output = T> + IndexMut<usize>,
            Func: for<'scope> Fn(&'scope mut T, NDimIndex<DIM>) + 'env + Sync + Send,
            T: Send + Sync,
        {
            fn use_scope<'scope, TScope>(self, scope: &'scope TScope) -> ()
            where
                TScope: 'scope + scope_no_ret::ThreadScope<'scope>,
                'env: 'scope,
            {
                let func = self.func;
                let array = self.array;

                array.for_each_mut(&|t, i| {
                    scope.spawn(move || {
                        (func)(t, i);
                    });
                });
            }
        }
        scope_creator.scope(A {
            array: self,
            func: func,
        });
    }

    // fn for_each_parallel<'ext_env, Func, TScopeCreator>(
    //     &'ext_env self,
    //     func: &Func,
    //     scope_creator: &TScopeCreator,
    // ) where
    //     Func: for<'scope> Fn(&'scope T, &'scope NDimIndex<DIM>) + 'ext_env + Sync + Send,
    //     TScopeCreator: ThreadScopeCreator + Sync,
    //     T: Send + Sync,
    // {
    //     struct A<'env, Func, TIndexer, const DIM: usize, T, Storage>
    //     where
    //         TIndexer: Deref<Target: TNDimIndexer<DIM>>,
    //         Storage: Index<usize, Output = T> + IndexMut<usize>,
    //     {
    //         array: &'env NDimArray<TIndexer, DIM, T, Storage>,
    //         func: &'env Func,
    //     }
    //     impl<'env, Func, TIndexer, const DIM: usize, T, Storage> ThreadScopeUser<'env>
    //         for A<'env, Func, TIndexer, DIM, T, Storage>
    //     where
    //         TIndexer: Deref<Target: TNDimIndexer<DIM>>,
    //         Storage: Index<usize, Output = T> + IndexMut<usize>,
    //         Func: for<'scope> Fn(&'scope T, &'scope NDimIndex<DIM>) + 'env + Sync + Send,
    //         T: Send + Sync,
    //     {
    //         fn use_scope<'scope, TScope>(self, scope: &'scope TScope) -> ()
    //         where
    //             TScope: 'scope + scope_no_ret::ThreadScope<'scope>,
    //             'env: 'scope,
    //         {
    //             let array = self.array;
    //             let func = self.func;
    //             for index in array.n_dim_index.iter().enumerate() {
    //                 if let Some(item) = array.get_with_compressed(index.0) {
    //                     scope.spawn(move || {
    //                         (func)(item, &index.1);
    //                     });
    //                 }
    //             }
    //         }
    //     }
    //     scope_creator.scope(A {
    //         array: self,
    //         func: &func,
    //     });
    // }
}

impl<'a, TIndexer, const DIM: usize, T, Storage> TNDimArrayGetWithNeiborhoods<'a, DIM, T>
    for NDimArray<TIndexer, DIM, T, Storage>
where
    TIndexer: Deref<Target: TNDimIndexer<DIM> + 'a>,
    Storage: Index<usize, Output = T> + IndexMut<usize>,
{
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
        ) -> NeiborhoodResult,
    {
        let op_may =
            NDimIndexOperator::from_index(unsafe { transmute(self.n_dim_index.deref()) }, *index);
        if let Some(op) = op_may {
            let res = unsafe { transmute(self.values.index(op.get_compressed())) };
            let res_neiborhoods = array::from_fn(|dim| {
                let mut nindex_0 = op.clone();
                let c_0 = nindex_0.move_n_at_dim(dim, 1);
                let res_0 = for_neiborhood(self, &mut nindex_0, dim, true, c_0 != 0);
                let mut nindex_1 = op.clone();
                let c_1 = nindex_0.move_n_at_dim(dim, -1);
                let res_1 = for_neiborhood(self, &mut nindex_1, dim, false, c_1 != 0);
                return (
                    (res_0, nindex_0.get().clone()),
                    (res_1, nindex_1.get().clone()),
                );
            });
            return Some(NDimArrayGetWithNeiborhoodsResult {
                cur: res,
                neiborhoods: res_neiborhoods,
            });
        }

        return None;
    }

    fn get_mut_with_neiborhoods_generic<ForNeiborhood, NeiborhoodResult>(
        &'a mut self,
        index: &NDimIndex<DIM>,
        mut for_neiborhood: ForNeiborhood,
    ) -> Option<NDimArrayGetWithNeiborhoodsResult<DIM, &'a mut T, NeiborhoodResult>>
    where
        ForNeiborhood: FnMut(
            &mut Self,
            &mut NDimIndexOperator<DIM, Self::TIndexer>,
            usize,
            bool,
            bool,
        ) -> NeiborhoodResult,
    {
        let op_may =
            NDimIndexOperator::from_index(unsafe { transmute(self.n_dim_index.deref()) }, *index);
        if let Some(op) = op_may {
            let res = unsafe { transmute(self.values.index_mut(op.get_compressed())) };
            let res_neiborhoods = array::from_fn(|dim| {
                let mut nindex_0 = op.clone();
                let c_0 = nindex_0.move_n_at_dim(dim, 1);
                let res_0 = for_neiborhood(self, &mut nindex_0, dim, true, c_0 != 0);
                let mut nindex_1 = op.clone();
                let c_1 = nindex_0.move_n_at_dim(dim, -1);
                let res_1 = for_neiborhood(self, &mut nindex_1, dim, false, c_1 != 0);
                return (
                    (res_0, nindex_0.get().clone()),
                    (res_1, nindex_1.get().clone()),
                );
            });
            return Some(NDimArrayGetWithNeiborhoodsResult {
                cur: res,
                neiborhoods: res_neiborhoods,
            });
        }

        return None;
    }

    type TIndexer = &'a <TIndexer as Deref>::Target;
}

impl<TIndexer, const DIM: usize, T, Storage> TNDimArrayParallelIterPair<DIM, T>
    for NDimArray<TIndexer, DIM, T, Storage>
where
    TIndexer: Deref<Target: TNDimIndexer<DIM>>,
    Storage: Index<usize, Output = T> + IndexMut<usize>,
{
    fn parallel_iter_pair<'ext_env, Func, TScopeCreator>(
        &'ext_env self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope T, &'scope T, usize) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync,
    {
        for mut_dim in 0..DIM {
            struct AScopeUser<'env, Func, TIndexer, const DIM: usize, T, Storage>
            where
                TIndexer: Deref<Target: TNDimIndexer<DIM>>,
                Storage: Index<usize, Output = T> + IndexMut<usize>,
            {
                values: &'env NDimArray<TIndexer, DIM, T, Storage>,
                mut_dim: usize,
                func: &'env Func,
                plus_1: bool,
            }
            impl<'env, Func, TIndexer, const DIM: usize, T, Storage> ThreadScopeUser<'env>
                for AScopeUser<'env, Func, TIndexer, DIM, T, Storage>
            where
                TIndexer: Deref<Target: TNDimIndexer<DIM>>,
                Storage: Index<usize, Output = T> + IndexMut<usize>,
                Func: for<'scope> Fn(&'scope T, &'scope T, usize) + 'env + Sync + Send,
                T: Send + Sync,
            {
                fn use_scope<'scope, TScope>(self, scope: &'scope TScope) -> ()
                where
                    TScope: 'scope + scope_no_ret::ThreadScope<'scope>,
                    'env: 'scope,
                {
                    let rem = if self.plus_1 { 0 } else { 1 };
                    let values = self.values;
                    let n_dim_index: &TIndexer = unsafe { transmute(&values.n_dim_index) };
                    let mut_dim = self.mut_dim;
                    let func = self.func;
                    for p in n_dim_index.iter() {
                        if p[mut_dim] % 2 == rem {
                            continue;
                        }
                        let mut p2 = p.clone();
                        p2[mut_dim] += 1;
                        if let Some(v1) = unsafe { values.get_other(&p) } {
                            if let Some(v2) = unsafe { values.get_other(&p2) } {
                                scope.spawn(move || {
                                    (func)(v1, v2, mut_dim);
                                });
                            }
                        }
                    }
                }
            }
            scope_creator.scope(AScopeUser {
                values: self,
                mut_dim,
                func: &func,
                plus_1: false,
            });
            scope_creator.scope(AScopeUser {
                values: self,
                mut_dim,
                func: &func,
                plus_1: true,
            });
        }
    }
    fn parallel_iter_pair_mut<'ext_env, Func, TScopeCreator>(
        &'ext_env mut self,
        func: &Func,
        scope_creator: &TScopeCreator,
    ) where
        Func: for<'scope> Fn(&'scope mut T, &'scope mut T, usize) + 'ext_env + Sync + Send,
        TScopeCreator: ThreadScopeCreator + Sync,
        T: Send + Sync,
    {
        for mut_dim in 0..DIM {
            struct AScopeUser<'env, Func, TIndexer, const DIM: usize, T, Storage>
            where
                TIndexer: Deref<Target: TNDimIndexer<DIM>>,
                Storage: Index<usize, Output = T> + IndexMut<usize>,
            {
                values: &'env mut NDimArray<TIndexer, DIM, T, Storage>,
                mut_dim: usize,
                func: &'env Func,
                plus_1: bool,
            }
            impl<'env, Func, TIndexer, const DIM: usize, T, Storage> ThreadScopeUser<'env>
                for AScopeUser<'env, Func, TIndexer, DIM, T, Storage>
            where
                TIndexer: Deref<Target: TNDimIndexer<DIM>>,
                Storage: Index<usize, Output = T> + IndexMut<usize>,
                Func: for<'scope> Fn(&'scope mut T, &'scope mut T, usize) + 'env + Sync + Send,
                T: Send + Sync,
            {
                fn use_scope<'scope, TScope>(self, scope: &'scope TScope) -> ()
                where
                    TScope: 'scope + scope_no_ret::ThreadScope<'scope>,
                    'env: 'scope,
                {
                    let rem = if self.plus_1 { 0 } else { 1 };
                    let values = self.values;
                    let n_dim_index: &TIndexer = unsafe { transmute(&values.n_dim_index) };
                    let mut_dim = self.mut_dim;
                    let func = self.func;
                    for p in n_dim_index.iter() {
                        if p[mut_dim] % 2 == rem {
                            continue;
                        }
                        let mut p2 = p.clone();
                        p2[mut_dim] += 1;
                        if let Some(v1) = unsafe { values.get_mut_other(&p) } {
                            if let Some(v2) = unsafe { values.get_mut_other(&p2) } {
                                scope.spawn(move || {
                                    (func)(v1, v2, mut_dim);
                                });
                            }
                        }
                    }
                }
            }
            scope_creator.scope(AScopeUser {
                values: self,
                mut_dim,
                func: &func,
                plus_1: false,
            });
            scope_creator.scope(AScopeUser {
                values: self,
                mut_dim,
                func: &func,
                plus_1: true,
            });
        }
    }
}

#[derive(Debug)]
pub struct NDimArrayGetWithNeiborhoodsResult<const DIM: usize, T, TNeiborhood> {
    pub cur: T,
    pub neiborhoods: [((TNeiborhood, NDimIndex<DIM>), (TNeiborhood, NDimIndex<DIM>)); DIM],
}
// #[derive(Debug)]
// pub struct NDimArrayGetMutWithNeiborhoodsResult<'a,const DIM:usize,T>{
//     pub cur:&'a mut T,
//     pub neiborhoods:[ ((Option<&'a T>,NDimIndex<DIM>),(Option<&'a T>,NDimIndex<DIM>)) ;DIM]
// }
// #[derive(Debug)]
// pub struct NDimArrayGetMutWithNeiborhoodsMutResult<'a,const DIM:usize,T>{
//     pub cur:&'a mut T,
//     pub neiborhoods:[ ((Option<&'a mut T>,NDimIndex<DIM>),(Option<&'a mut T>,NDimIndex<DIM>)) ;DIM]
// }

#[cfg(test)]
mod test {

    // use core::time;
    // use std::thread;

    // use crate::structures::{n_dim_array::{NDimArray}, n_dim_index::NDimIndexer};
    use super::*;

    #[test]
    fn test_ndim_array_bi_iter_mut() {
        // const ADIM:usize=3;
        let andidx = NDimIndexer::new_len([0..3, 0..3, 0..3]);
        let mut andarr = NDimArray::from_fn(&andidx, |idx| (idx, Vec::<NDimIndex<3>>::new()));
        andarr.parallel_iter_pair_mut(
            &|a, b, dim| {
                // thread::sleep(time::Duration::from_millis());
                // println!("({:?},{:?})",a.0,b.0);
                a.0.iter()
                    .zip(b.0.iter())
                    .enumerate()
                    .for_each(|(i, (a_i, b_i))| {
                        if i == dim {
                            assert_eq!(*a_i + 1, *b_i);
                        } else {
                            assert_eq!(a_i, b_i);
                        }
                    });
                a.1.push(b.0.clone());
                b.1.push(a.0.clone());
            },
            &ThreadScopeCreatorStd,
        );
        // for awdawd in andidx.iter() {
        //     let got=andarr.get_mut_with_neiborhoods(&awdawd);
        //     println!("{:#?}",got);
        // }
        for i in andidx.iter() {
            // println!("{:?} : {:?}", i, andarr.get(&i).unwrap().1);
        }
    }
}
