pub trait SqlEnum {
    fn to_sql(&self) -> String;
    fn from_sql(s: &str) -> Option<Self> 
    where 
        Self: Sized;
}