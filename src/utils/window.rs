#[macro_export]
macro_rules! trait_field {
    ($name:ident: $t:ty) => {
        paste::paste! {
            fn [<get_ $name>](&self) -> $t;
            fn [<set_ $name>](&self, $name: $t);
        }
    };
}

#[macro_export]
macro_rules! trait_field_ref {
    ($name:ident: $t:ty) => {
        paste::paste! {
            fn [<get_ $name>](&self) -> &$t;
            fn [<get_ $name _mut>](&mut self) -> &mut $t;
            fn [<set_ $name>](&self, $name: $t);
        }
    };
}