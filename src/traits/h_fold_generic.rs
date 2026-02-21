use std::process::Output;

use frunk::{HCons, HNil};

pub trait GenericFolder<Acc,X>:Clone{
	type Output;
	fn fold_apply(self,acc:Acc,x:X)->Self::Output;
}

pub trait HFoldableGeneric<Init,GFolder>
{
	type Output;
	fn fold(self,init:Init,folder:GFolder)->Self::Output;
}

impl<Init,GFolder> HFoldableGeneric<Init,GFolder> for HNil  {
	type Output=Init;

	fn fold(self,init:Init,_folder:GFolder)->Self::Output {
		init
	}
}

impl<Init,GFolder,H,T,GFOutput,TFOutput> HFoldableGeneric<Init,GFolder> for HCons<H,T>
	where GFolder:GenericFolder<Init,H,Output = GFOutput>+Clone,
		T:HFoldableGeneric<GFOutput,GFolder,Output = TFOutput>
{
	type Output=TFOutput;

	fn fold(self,init:Init,folder:GFolder)->Self::Output {
		
		self.tail.fold(folder.clone().fold_apply(init, self.head), folder)
	}
}

pub struct HListRightFold<T>(pub T);

impl<Init,GFolder> HFoldableGeneric<Init,GFolder> for HListRightFold<HNil> {
	type Output=Init;

	fn fold(self,init:Init,_folder:GFolder)->Self::Output {
		init
	}
}

impl<Init,GFolder,H,T,GFOutput,TFOutput> HFoldableGeneric<Init,GFolder> for HListRightFold<HCons<H,T>>
	where 
		HListRightFold<T>:HFoldableGeneric<Init,GFolder,Output = TFOutput>,
		GFolder:GenericFolder<TFOutput,H,Output = GFOutput>+Clone,
{
	type Output=GFOutput;

	fn fold(self,init:Init,folder:GFolder)->Self::Output {
		folder.clone().fold_apply(HListRightFold(self.0.tail).fold(init, folder), self.0.head)
	}
}

pub trait GenericMapper<Input> {
	type Output;
	fn map_apply(self,input:Input)->Self::Output;
}

pub struct HListMapper<GMapper>(pub GMapper);

impl<Acc,X,Mapper> GenericFolder<Acc,X> for HListMapper<Mapper>
	where Mapper:GenericMapper<X>+Clone
{
	
}