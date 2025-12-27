use std::thread;

pub trait ThreadScope<'scope,ScopeFnOutput:'scope>{

    fn spawn<F>(&'scope self, f: F) ->()
        where
            F: FnOnce()->ScopeFnOutput + std::marker::Send + 'scope;
    
}

impl<'scope,'env,ScopeFnOutput:Send+'scope> ThreadScope<'scope,ScopeFnOutput> for thread::Scope<'scope,'env> {
    fn spawn<F>(&'scope self, f: F) -> ()
        where
            F: FnOnce()->ScopeFnOutput + std::marker::Send + 'scope 
    {
        self.spawn(f);
    }

}

pub trait ThreadScopeUser<'env> {
    type ScopeFnOutput:'env;
    type Output;
    fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->Self::Output
        where TScope:'scope+ThreadScope<'scope,Self::ScopeFnOutput>,
            'env:'scope;
}


pub trait ThreadScopeCreator<Output,ScopeFnOutput> {
    
    type Output <'env,F> 
        where F: ThreadScopeUser<'env,Output = Output,ScopeFnOutput = ScopeFnOutput>;

    //type Scope<'scope,'env:'scope>:Scope<'scope>+'scope;

    fn scope<'env,F>(&mut self,f:F ) -> Self::Output<'env,F>
        where F: ThreadScopeUser<'env,Output = Output,ScopeFnOutput = ScopeFnOutput>,
            //'env:'scope
        ;
}

pub struct ThreadScopeCreatorStd;

impl<Output,ScopeFnOutput:Send> ThreadScopeCreator<Output,ScopeFnOutput> for ThreadScopeCreatorStd {
    //type Scope<'scope,'env:'scope> = thread::Scope<'scope,'env>;
    
    fn scope<'env,F>(&mut self,f:F )-> Self::Output<'env,F>
        where F: ThreadScopeUser<'env,Output = Output,ScopeFnOutput = ScopeFnOutput>,
    {
        thread::scope(|scope: &thread::Scope<'_, '_>|f.use_scope(scope))
    }
    
    type Output <'env,F> = <F as ThreadScopeUser<'env>>::Output
        where F: ThreadScopeUser<'env,Output = Output,ScopeFnOutput = ScopeFnOutput>;
    
    
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
                    where TScope:'scope+ThreadScope<'scope,()>,
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