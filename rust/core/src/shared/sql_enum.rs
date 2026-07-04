pub trait SqlEnum {
    fn to_sql(&self) -> &'static str;
    fn from_sql(s: &str) -> Option<Self> 
    where 
        Self: Sized;
}