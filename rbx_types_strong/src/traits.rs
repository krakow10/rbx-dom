use rbx_types::Variant;

pub trait Properties{
    fn clone_property(&self,prop_name:&str)->Variant;
    fn set_property(&mut self,prop_name:&str,value:Variant);
}

impl Properties for crate::instances::BasePart{
    fn clone_property(&self,prop_name:&str)->Variant {
        match prop_name{
            "CFrame"=>Variant::CFrame(self.CFrame),
            // other=>self.superclass.clone_property(other),
            other=>panic!("unknown property {other}"),
        }
    }
    fn set_property(&mut self,prop_name:&str,value:Variant) {
        match (prop_name,value){
           ("CFrame",Variant::CFrame(value))=>self.CFrame=value,
           // (other,variant)=>self.superclass.set_property(other,variant),
           (other,variant)=>panic!("unknown property '{other}' or wrong variant type '{variant:?}'"),
        }
    }
}
