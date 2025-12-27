use std::thread;

pub trait ThreadScope<'scope>{
    fn spawn<F>(&'scope self, f: F) ->()
        where
            F: Send+'scope+FnOnce()->();
    
}

impl<'scope,'env> ThreadScope<'scope> for thread::Scope<'scope,'env> {
    fn spawn<F>(&'scope self, f: F) ->()
        where
            F: Send+'scope+FnOnce()->()
    {
        self.spawn(f);
    }
}

pub trait ThreadScopeUser<'scope,TScope>
	where TScope:'scope+ThreadScope<'scope>,
{
    fn use_scope(self, scope:&'scope TScope)->();
}


/*
pub trait ThreadScopeCreator<'env,F>
where F: for<'scope> ThreadScopeUser<'scope,Self::Scope<'scope>>,

{

    type Scope<'scope>:ThreadScope<'scope>+'scope;

    fn scope(&mut self,f:F ) -> ();
}

pub struct ThreadScopeCreatorStd;

impl<'env,F> ThreadScopeCreator<'env,F> for ThreadScopeCreatorStd 
where F: for<'scope> ThreadScopeUser<'scope,Self::Scope<'scope>>,

{
    type Scope<'scope> = thread::Scope<'scope,'env>;
    
    fn scope(&mut self,f:F ) -> ()
    {
        thread::scope(|scope: &thread::Scope<'_, '_>|f.use_scope(scope))
    }
    
    //type Output<'scope,'env:'scope,F:'scope + ScopeUser<'env> + 'env > = <F as ScopeUser<'env>>::Output;
}
*/

pub trait ThreadScopeCreator {

    type Scope<'scope,'env>:ThreadScope<'scope>
		where 'env:'scope;

    fn scope<'env,Func>(&'env self,f:Func ) -> ()
		// where Func:for<'scope> FnOnce(&'scope Self::Scope<'scope,'env>),
        where Func: for<'scope> ThreadScopeUser<'scope,Self::Scope<'scope,'env>>,

            //'env:'scope
        ;
}

pub struct ThreadScopeCreatorStd;

impl ThreadScopeCreator for ThreadScopeCreatorStd {
    type Scope<'scope,'env> = thread::Scope<'scope,'env>
		where 'env:'scope;
    
    fn scope<'env,Func>(&'env self,f:Func ) -> ()
		// where Func:for<'scope> FnOnce(&'scope thread::Scope<'scope,'env>),
        where Func: for<'scope> ThreadScopeUser<'scope,thread::Scope<'scope,'env>>,

            //'env:'scope
    {
        thread::scope(|s|f.use_scope(s));
    }
    
    //type Output<'scope,'env:'scope,F:'scope + ScopeUser<'env> + 'env > = <F as ScopeUser<'env>>::Output;
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
		impl<'scope,'env:'scope,TScope> ThreadScopeUser<'scope,TScope> for AScopeUser<'env> 
			where TScope:'scope+ThreadScope<'scope>,
		{
			
			fn use_scope(self, scope:&'scope TScope)->()
					{
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
		let _spam=ThreadScopeCreatorStd::scope(&ThreadScopeCreatorStd, AScopeUser{a:&a,x:&mut x});
		// thread::scope(|s|(AScopeUser{a:&a,x:&mut x}).use_scope(s));
		// let _spam=ThreadScopeCreatorStd::scope(&mut ThreadScopeCreatorStd, AScopeUser{a:&a,x:&mut x});
		a.push(4);
		assert_eq!(x as usize, a.len());
        
    }
}
