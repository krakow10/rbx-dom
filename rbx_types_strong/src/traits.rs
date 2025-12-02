use rbx_types::Variant;

pub trait Properties{
    fn clone_property(&self,prop_name:&str)->Variant;
    fn set_property(&mut self,prop_name:&str,value:Variant);
}

