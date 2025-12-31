use std::thread::{self, ScopedJoinHandle};

pub trait ThreadScope<'scope,ScopeFnOutput:'scope+Send>{
	type Output;
    fn spawn<F>(&'scope self, f: F) ->Self::Output
        where
            F: FnOnce()->ScopeFnOutput + Send + 'scope,
			;
    
}

impl<'scope,'env,ScopeFnOutput:'scope+Send> ThreadScope<'scope,ScopeFnOutput> for thread::Scope<'scope,'env> {
	type Output = ScopedJoinHandle<'scope, ScopeFnOutput>;
    fn spawn<F>(&'scope self, f: F) -> ScopedJoinHandle<'scope, ScopeFnOutput>
        where
            F: FnOnce()->ScopeFnOutput + std::marker::Send + 'scope,
    {
        self.spawn(f)
    }

}

pub trait ThreadScopeUserOutput<'env> {
    type Output;
    type ScopeFnOutput:'env+Send;
}

pub trait ThreadScopeUser<'env,TScope>:ThreadScopeUserOutput<'env>
	where TScope: for<'scope> ThreadScope<'scope,Self::ScopeFnOutput>,
{
    fn use_scope<'scope>(self, scope:&'scope TScope)->Self::Output
        where 'env:'scope;
}



pub trait ThreadScopeCreator
{
    type TScope<'env,'scope,ScopeFnOutput>: ThreadScope<'scope,ScopeFnOutput>
		where ScopeFnOutput:Send+'scope,'env:'scope;

    type Output <'env,F> 
		where F:'env + ThreadScopeUserOutput<'env>;

    //type Scope<'scope,'env:'scope>:Scope<'scope>+'scope;

    fn scope<'env,F>(&self,f:F ) -> Self::Output<'env,F>
        where F:'env+ for<'scope> ThreadScopeUser<'env,Self::TScope<'env,'scope,<F as ThreadScopeUserOutput<'env>>::ScopeFnOutput>>,
        ;
}

pub struct ThreadScopeCreatorStd;

impl ThreadScopeCreator for ThreadScopeCreatorStd {
    type TScope<'env,'scope,ScopeFnOutput> = thread::Scope<'scope,'env>
			where ScopeFnOutput:Send+'scope,'env:'scope;
	
	type Output <'env,F> = <F as ThreadScopeUserOutput<'env>>::Output
		where F:'env + ThreadScopeUserOutput<'env>;
		// where F: for<'scope> ThreadScopeUser<'env,Self::TScope<'env,'scope>,Output = Output,ScopeFnOutput = ScopeFnOutput>;
	
	fn scope<'env,F>(&self,f:F ) -> Self::Output<'env,F>
		where F:'env+for<'scope> ThreadScopeUser<'env,Self::TScope<'env,'scope,<F as ThreadScopeUserOutput<'env>>::ScopeFnOutput>>,
	{
		thread::scope(|s|f.use_scope(s))
	}
	//type Scope<'scope,'env:'scope> = thread::Scope<'scope,'env>;
    // 
    
    // type Output <'env,F> = <F as ThreadScopeUser<'env>>::Output
    //     where F: ThreadScopeUser<'env,Output = Output,ScopeFnOutput = ScopeFnOutput>;
    
    
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
            impl<'env> ThreadScopeUser<'env> for AScopeUser<'env> {
                type Output=();
                
                fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->Self::Output
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
                
                type ScopeFnOutput=();
            
                
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