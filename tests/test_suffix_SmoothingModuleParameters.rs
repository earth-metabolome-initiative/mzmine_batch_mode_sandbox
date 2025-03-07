use mzbatch_generator::smoothing_module_parameters::SmoothSuffix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let suffix_obj = SmoothSuffix::new();
        assert_eq!{suffix_obj.get_name(), "Suffix"};
        assert_eq!{suffix_obj.get_value(), ""};
    }

    #[test]
    fn test_suffix_set_get_value(){
        let mut suffix_obj = SmoothSuffix::new();
        assert_eq!(suffix_obj.get_value(), "");
        suffix_obj.set_value("sm");
        assert_eq!(suffix_obj.get_value(), "sm");
    }
}
