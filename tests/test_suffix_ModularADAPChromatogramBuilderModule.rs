use mzbatch_generator::modular_adap_chromatogram_builder_module_parameters::Suffix;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let suffix_obj = Suffix::new();
        assert_eq!(suffix_obj.get_name(), "Suffix");
        assert_eq!(suffix_obj.get_value(), "");
    }

    #[test]
    fn test_suffix_set_get_value(){
        let mut suffix_obj = Suffix::new();
        assert_eq!(suffix_obj.get_value(), "");
        suffix_obj.set_value("New value");
        assert_eq!(suffix_obj.get_value(), "New value");
    }
}