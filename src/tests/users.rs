#[allow(dead_code)]
//use std::{thread, time};
use assert_json_diff::assert_json_include;

use crate::tests::test_helpers::*;

#[async_std::test]
async fn get_users_count() {
    let mut server = test_setup().await;

    let (json, status, _) = get(&format!("/")).send(&mut server).await;

    assert_eq!(status, 200);

    assert_json_include!(
        actual: json,
        expected :{
            json!({
                "count" :0
            })
        }
    );
}
