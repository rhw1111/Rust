use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Once;
use crate::modelbase::*;

static INIT: Once = Once::new();
pub static mut ENTITYAIMPSTATICFACTORY:Option<Arc<dyn Fn()->Box<dyn EntityAIMP>+Send+Sync>>=None;



pub struct EntityA<'a>{
    base:ModelBase<'a>,
    imp:Box<dyn EntityAIMP>

}


impl Deref for EntityA<'static>{
    type Target=ModelBase<'static>;
    fn deref<'a>(&'a self)->&'a ModelBase<'static>{
        &self.base
    }
}

impl DerefMut for EntityA<'static>{
    fn deref_mut<'a>(&'a mut self)->&'a mut ModelBase<'static>{
        &mut self.base
    }
}


pub trait EntityAIMP
{
     fn test(&self, entity:&EntityA)->&str;
} 



impl<'a> EntityA<'a>{
     pub fn new()->EntityA<'a>{
        
        let realimp:Box<dyn EntityAIMP>;
        unsafe {
            INIT.call_once(|| {
                if let None = ENTITYAIMPSTATICFACTORY{
                let x:Arc<dyn Fn()->Box<dyn EntityAIMP>+Send+Sync>=Arc::new(||->Box<dyn EntityAIMP>
                    {
            
                        Box::new(EntityARealIMP::new())
                       
                    });
                    ENTITYAIMPSTATICFACTORY = Some(x);
                }

                
            });


            if let Some(v) = &ENTITYAIMPSTATICFACTORY{
                realimp=v();
            }
            else
            {
                realimp=Box::new(EntityARealIMP::new());
            }

        }

        EntityA{
            base:ModelBase::new(),
            imp:realimp
        }
    
    }

    pub fn getid(&self)->&str{
        let value=self.base.getattribute::<String>("id");
        let mut result:&str =Default::default();
        if let Some(v) = value {
            result=v;
        } 
        
        result
    }

    pub fn setid(&mut self,value:String){
        self.base.setattribute::<String>("id",value);
    }

    pub fn getremark(&self)->Option<&str>{
        let value=self.base.getattribute::<Option<String>>("remark");
        let mut result:Option<&str> = Option::None;
        if let Some(v) = value {
            if let Some(iv)=v{
                result=Option::Some(iv);     
            }
            
        } 

        result
    }

    pub fn setremark(&mut self,value:Option<String>){
        self.base.setattribute::<Option<String>>("remark",value);
    }

    pub fn test(&self)->&str{
        self.imp.test(self)
    }

}

pub struct EntityARealIMP();
impl EntityARealIMP{
    pub fn new()->EntityARealIMP
    {
        EntityARealIMP{}
    }
}
impl EntityAIMP for EntityARealIMP 
{
    fn test(&self,entity:&EntityA)->&str{
        "eeee"
    }
}