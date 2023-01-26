#[cfg(test)]
mod test {
    #[test]
    fn find_out_common_columns() {
        // Arrange
        let left_input = "krx_sector_kospi.parquet";
        let right_input = "krx_individual_kospi.parquet";
        // Act
        let result = get_common_columns(left_input, right_input).unwrap();
        // Assert
        assert_eq!(result, []);
    }
}
