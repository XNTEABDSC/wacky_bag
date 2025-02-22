use crate::utils::grid_iter;
#[test]
fn test(){
    println!("{}",usize::MAX.wrapping_add(1 as usize));
    let mut iter=grid_iter::GridIter::new();
    let mut gridtest=[[0i32;16];16];

    for _i in 1..100{
        let p=iter.next().unwrap();
        println!("({}, {}), {}",p.point.0,p.point.1,p.distance);
        if p.point.0<16&&p.point.1<16 {
            gridtest[p.point.1 as usize][p.point.0 as usize]=1
        }
    }
    for y in 0..16 {
        for x in 0..16{
            print!("{}",gridtest[y as usize][x as usize])
        }
        println!()
    }
}
