use std::thread;

pub trait Scope<'scope>{
    fn spawn<F>(&'scope self, f: F) -> ()
        where
            F: FnOnce() + std::marker::Send + 'scope;
    
}

impl<'scope,'env> Scope<'scope> for thread::Scope<'scope,'env> {
    fn spawn<F>(&'scope self, f: F) -> ()
        where
            F: FnOnce() + std::marker::Send + 'scope {
        self.spawn(f);
    }
}

pub trait ScopeCreator {

    type Scope<'scope,'env:'scope>:Scope<'scope>+'scope;

    fn scope<'env,F,T>(&mut self,f:F )->T
        where F: for<'scope> FnOnce(&'scope Self::Scope<'scope,'env>) -> T,
            F:'env
            //'env:'scope
        ;
}

pub struct StdScopeCreator;

impl ScopeCreator for StdScopeCreator {
    type Scope<'scope,'env:'scope> = thread::Scope<'scope,'env>;
    
    fn scope<'env,F,T>(&mut self,f:F )->T
        where F: for<'scope> FnOnce(&'scope Self::Scope<'scope,'env>) -> T,
            F:'env
         {
        thread::scope(f)
    }
    

}

fn test_scope<'env,F,T>(f:F )->T
    where F: for<'scope> FnOnce(&'scope thread::Scope<'scope,'env>) -> T,
        //'env:'scope
        {
    thread::scope(f)
}

fn test(){
    use std::thread;

    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        
        s.spawn(|| {
            println!("hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}