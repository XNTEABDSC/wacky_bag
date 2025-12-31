use std::ops::ControlFlow;

pub trait SteppingIteratorFunction<Item,Data>{
	fn step(&mut self,item:Item,data:Data,dim:usize)->ControlFlow<(),()>;
}

impl<F,Item,Data> SteppingIteratorFunction<Item,Data> for F
	where F: FnMut(Item,Data,usize)->ControlFlow<(),()>
{
	fn step(&mut self,item:Item,data:Data,dim:usize)->ControlFlow<(),()> {
		(self)(item,data,dim)
	}
}

pub struct SteppingIterator<const N:usize, Items, Datas, StepFn>
{
	items:Items,
	datas:Datas,
	step_fn:StepFn,
	ended:bool
}

impl<const N:usize, Items, Datas, StepFn, Item, Data> SteppingIterator<N, Items, Datas, StepFn>
where 
	Items:Clone,
	for<'a>&'a mut Items: IntoIterator<Item = Item>,
	for<'a>&'a Datas: IntoIterator<Item = Data>,
	StepFn: SteppingIteratorFunction<Item,Data>,
{
	pub fn new(items:Items,datas:Datas,f_next:StepFn)->Self{
		Self{
			items,
			datas,
			step_fn: f_next,
			ended:false
		}
	}
}

impl<const N:usize, Items, Datas, StepFn,Item,Data> Iterator for SteppingIterator<N, Items, Datas, StepFn>
where 
	Items:Clone,
	for<'a>&'a mut Items: IntoIterator<Item = Item>,
	for<'a>&'a Datas: IntoIterator<Item = Data>,
	StepFn: SteppingIteratorFunction<Item,Data>,
{
	type Item = Items;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ended {
			return None;
		}
		let ret=self.items.clone();
		let loop_res=(&mut self.items).into_iter().zip(
			(&self.datas).into_iter()
		).enumerate().try_for_each(|(k,(i,d))|self.step_fn.step(i,d,k));
		if let ControlFlow::Continue(())=loop_res {
			self.ended=true;
		}
		return Some(ret);
	}
}

pub struct SteppingIteratorRev<const N:usize, Items, Datas, StepFn>
{
	items:Items,
	datas:Datas,
	step_fn:StepFn,
	ended:bool
}


impl<const N:usize, Items, Datas, StepFn, Item, Data> SteppingIteratorRev<N, Items, Datas, StepFn>
where 
	Items:Clone,
	for<'a>&'a mut Items: IntoIterator<Item = Item,IntoIter : DoubleEndedIterator+ExactSizeIterator>,
	for<'a>&'a Datas: IntoIterator<Item = Data,IntoIter : DoubleEndedIterator+ExactSizeIterator>,
	StepFn: SteppingIteratorFunction<Item,Data>,
{
	pub fn new(items:Items,datas:Datas,f_next:StepFn)->Self{
		Self{
			items,
			datas,
			step_fn: f_next,
			ended:false
		}
	}
}


impl<const N:usize, Items, Datas, StepFn,Item,Data> Iterator for SteppingIteratorRev<N, Items, Datas, StepFn>
where 
	Items:Clone,
	for<'a>&'a mut Items: IntoIterator<Item = Item,IntoIter : DoubleEndedIterator+ExactSizeIterator>,
	for<'a>&'a Datas: IntoIterator<Item = Data,IntoIter : DoubleEndedIterator+ExactSizeIterator>,
	StepFn: SteppingIteratorFunction<Item,Data>,
{
	type Item = Items;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ended {
			return None;
		}
		let ret=self.items.clone();
		let loop_res=(&mut self.items).into_iter().zip(
			(&self.datas).into_iter()
		).enumerate().rev().try_for_each(|(k,(i,d))|self.step_fn.step(i,d,k));
		if let ControlFlow::Continue(())=loop_res {
			self.ended=true;
		}
		return Some(ret);
	}
}