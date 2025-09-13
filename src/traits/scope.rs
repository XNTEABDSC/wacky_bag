use std::thread;

pub trait Scope<'scope,ScopeFnOutput:Send+'scope>{

    fn spawn<F>(&'scope self, f: F) ->()
        where
            F: FnOnce()->ScopeFnOutput + std::marker::Send + 'scope;
    
}

impl<'scope,'env,FRet:Send+'scope> Scope<'scope,FRet> for thread::Scope<'scope,'env> {
    fn spawn<F>(&'scope self, f: F) -> ()
        where
            F: FnOnce()->FRet + std::marker::Send + 'scope 
    {
        self.spawn(f);
    }

}

pub trait ScopeUser<'env> {
    type ScopeFnOutput:Send+'env;
    type Output;
    fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->Self::Output
        where TScope:'scope+Scope<'scope,Self::ScopeFnOutput>,
            'env:'scope;
}


pub trait ScopeCreator {
    
    type Output <'env,F> where F: ScopeUser<'env> + 'env;

    //type Scope<'scope,'env:'scope>:Scope<'scope>+'scope;

    fn scope<'env,F>(&mut self,f:F ) -> Self::Output<'env,F>
        where F: ScopeUser<'env> + 'env,
            //'env:'scope
        ;
}

pub struct StdScopeCreator;

impl ScopeCreator for StdScopeCreator {
    //type Scope<'scope,'env:'scope> = thread::Scope<'scope,'env>;
    
    fn scope<'env,F>(&mut self,f:F )-> Self::Output<'env,F>
        where F: ScopeUser<'env> + 'env,
    {
        thread::scope(|scope: &thread::Scope<'_, '_>|f.use_scope(scope))
    }
    
    type Output <'env,F>  = <F as ScopeUser<'env>>::Output where F: ScopeUser<'env> + 'env;
    
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
            };
            impl<'env> ScopeUser<'env> for AScopeUser<'env> {
                type Output=();
                
                fn use_scope<'scope,TScope>(self, scope:&'scope TScope)->Self::Output
                    where TScope:'scope+Scope<'scope,()>,
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
            let spam=StdScopeCreator::scope(&mut StdScopeCreator, AScopeUser{a:&a,x:&mut x});
            a.push(4);
            assert_eq!(x as usize, a.len());
        
    }
}
