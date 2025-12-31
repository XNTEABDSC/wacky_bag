use std::{marker::PhantomData, thread::{self, ScopedJoinHandle}};

pub trait ThreadScope<'scope>{
    fn spawn<F>(&'scope self, f: F) ->()
        where
            F: FnOnce()->() + Send + 'scope,
			;
    
}

impl<'scope,'env> ThreadScope<'scope> for thread::Scope<'scope,'env> {
    fn spawn<F>(&'scope self, f: F) -> ()
        where
            F: FnOnce()->() + std::marker::Send + 'scope,
    {
        self.spawn(f);
    }

}

pub trait ThreadScopeUser<'env>
{
    fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->()
        where 'env:'scope,
			TScope:ThreadScope<'scope>;
}

pub trait ThreadScopeCreator
{
    fn scope<'env,F>(&self,f:F ) -> ()
        where F:ThreadScopeUser<'env>,
            //'env:'scope
        ;
}

pub struct ThreadScopeCreatorStd;

impl ThreadScopeCreator for ThreadScopeCreatorStd {
    fn scope<'env,F>(&self,f:F ) -> ()
        where F:ThreadScopeUser<'env>
	{
		thread::scope(|s|f.use_scope(s));
	}
}


#[cfg(test)]
mod test{

    use super::*;
    #[test]
    fn test(){
        let mut a = vec![1, 2, 3];
            let mut x = 0;

            struct AScopeUser<'env>{
                a:&'env Vec<i32>,
                x:&'env mut i32
            }
            impl<'env> ThreadScopeUser<'env> for AScopeUser<'env> {

                fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->()
                    where TScope:'scope+ThreadScope<'scope>,
                        'env:'scope {
                    let a=self.a;
                    let x=self.x;
                    scope.spawn(move || {
                        println!("hello from the first scoped thread");
                        // We can borrow `a` here.
                        dbg!(a);
                    });
                    scope.spawn(|| {
                        println!("hello from the second scoped thread");
                        // We can even mutably borrow `x` here,
                        // because no other threads are using it.
                        *x += a[0] + a[2];
                    });
                    println!("hello from the main thread");
                }
            }
            for _ in 1..3 {
                let _spam=ThreadScopeCreatorStd::scope(&mut ThreadScopeCreatorStd, AScopeUser{a:&a,x:&mut x});
            }
            let _spam=ThreadScopeCreatorStd::scope(&mut ThreadScopeCreatorStd, AScopeUser{a:&a,x:&mut x});
            let _spam=ThreadScopeCreatorStd::scope(&mut ThreadScopeCreatorStd, AScopeUser{a:&a,x:&mut x});
            a.push(4);
            assert_eq!(x as usize, a.len());
        
    }
}