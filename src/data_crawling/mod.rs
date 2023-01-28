pub mod biz_day;
pub mod krx;
pub mod wics;

// #[cfg(test)]
// mod test_get_latest_wics {
//     use super::{wics::SECTOR_CODE_LIST, *};

//     #[tokio::test]
//     async fn each_sector_list_should_not_be_empty() {
//         // Arrange
//         let client = reqwest::Client::new();
//         // Act
//         let result = get_latest_wisc_data(&client).await.unwrap();
//         // Assert
//         SECTOR_CODE_LIST.iter().for_each(|sector_code| {
//             assert_ne!(result.select(col(sector_code)).is_empty());
//         })
//     }
// }
