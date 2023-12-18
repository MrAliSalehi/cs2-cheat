
pub type Res = eyre::Result<()>;

#[macro_export]
macro_rules! continue_if {
    ($cond:expr) => {
        if $cond { continue; }
    };
}