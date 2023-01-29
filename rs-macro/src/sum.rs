#[macro_export]
macro_rules! add {
    // first arm match add!(1,2), add!(2,3) etc
    ($a:expr,$b:expr)=>{
        {
            $a+$b
        }
    };
    // Second arm macth add!(1), add!(2) etc
    ($a:expr)=>{
        {
            $a
        }
    };
    // add the number and the result of remaining arguments
    ($a:expr,$($b:tt)*)=>{
       {
           $a+add!($($b)*)
       }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_two_numbers() {
        let result = add!(1, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn set_number() {
        let result = add!(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn sum_list_of_numbers() {
        let result = add!(1, 2, 3, 4);
        assert_eq!(result, 10);
    }
}
