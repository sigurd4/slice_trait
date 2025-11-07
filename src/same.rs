mod private
{
    pub const trait Same<T>: Sized
    {
        fn same(self) -> Result<T, Self>;
    }
    impl<T, U> const Same<T> for U
    {
        default fn same(self) -> Result<T, Self>
        {
            Err(self)
        }
    }
    impl<T> const Same<T> for T
    {
        fn same(self) -> Result<T, Self>
        {
            Ok(self)
        }
    }
}

pub const trait Same: Sized
{
    fn same<T>(self) -> Result<T, Self>;
}
impl<U> const Same for U
{
    fn same<T>(self) -> Result<T, Self>
    {
        private::Same::<T>::same(self)
    }
}