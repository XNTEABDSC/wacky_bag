use std::{collections::HashMap, sync::{LazyLock, Mutex}};



struct DimRootOfXUsize{
    values:Vec<HashMap<usize,(usize,usize)>>
}
impl DimRootOfXUsize {
    pub fn new()->Self {
        Self { values: Vec::new() }
    }
    pub fn get(&mut self,x:usize,dim:usize)->(usize,usize) {
        while self.values.len()<dim {
            self.values.push(HashMap::new());
        }
        let map=self.values.get_mut(dim).unwrap();
        let entry=map.entry(x);

        let res= entry.or_insert_with(
            ||{
                let root=f64::powf(x as f64, 1.0/(dim as f64)).floor() as usize;
                let root_pow=root.pow(dim as u32);
                (root,root_pow)
            }
        );
        {
            let mut check1_changed=false;
            loop {
                if res.1>x{
                    res.0-=1;
                    res.1=res.0.pow(dim as u32);
                    check1_changed=true;
                }else {
                    break;
                }
                
            }
            if check1_changed{
                return *res;
            }
            let mut res_plus_pow=(res.0+1).pow(dim as u32);
            loop {
                if res_plus_pow<=x{
                    res.0+=1;
                    res.1=res_plus_pow;
                    res_plus_pow=(res.0+1).pow(dim as u32);
                }else {
                    break;
                }
            }
            return *res;
        }
    }
}


impl Default for DimRootOfXUsize {
    fn default() -> Self {
        Self { values: Default::default() }
    }
}
//static mut DIM_ROOT_OF_X_USIZE:LazyLock<Arc<Mutex<DimRootOfXUsize>>>=LazyLock::new(||Default::default());

static DIM_ROOT_OF_X_USIZE:LazyLock< Mutex<DimRootOfXUsize> > = LazyLock::new(|| Default::default());
pub fn get_dim_root_of_a_usize(x:usize,dim:usize)->(usize,usize) {
    DIM_ROOT_OF_X_USIZE.lock().unwrap().get(dim, x)
}