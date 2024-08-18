

pub struct FIdUIdList{

    fid_to_uid_list:Vec<usize>,
    uid_to_fid_list:Vec<usize>,

}

impl FIdUIdList {
    #[inline]
    pub const fn new()->Self{
        FIdUIdList{
            fid_to_uid_list:Vec::new(),
            uid_to_fid_list:Vec::new(),
        }
    }

    #[inline]
    pub fn add(&mut self,fid:usize)->usize{
        let uid=self.uid_to_fid_list.len();
        for _ in self.fid_to_uid_list.len() .. fid {
            self.fid_to_uid_list.push(usize::MAX)
        }
        self.fid_to_uid_list.push(uid);
        //add_to_and_set(&mut self.fid_to_uid, fid, uid);
        
        return uid;
    }

    #[inline]
    pub fn remove(&mut self,fid:usize,uid:usize){
        //self.fid_to_uid.
        self.fid_to_uid_list[fid]=usize::MAX;
        self.uid_to_fid_list.swap_remove(uid);
    }

    #[inline]
    pub fn remove_by_fid(&mut self,fid:usize) {
        self.remove(fid, self.fid_to_uid_list[fid]);
    }

    #[inline]
    pub fn remove_by_uid(&mut self,uid:usize) {
        self.remove(self.uid_to_fid_list[uid], uid);
    }

    #[inline]
    pub fn fid_to_uid(&self,fid:usize)->usize{
        self.fid_to_uid_list[fid]
    }

    #[inline]
    pub fn uid_to_fid(&self,uid:usize)->usize{
        self.uid_to_fid_list[uid]
    }
}