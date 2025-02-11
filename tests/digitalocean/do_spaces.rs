use crate::helpers::utilities::{context, generate_id, FuncTestsSecrets};
use qovery_engine::models::digital_ocean::DoRegion;
use qovery_engine::object_storage::spaces::{BucketDeleteStrategy, Spaces};
use qovery_engine::object_storage::ObjectStorage;
use tempfile::NamedTempFile;

#[allow(dead_code)]
const TEST_REGION: DoRegion = DoRegion::Amsterdam3;

#[cfg(feature = "test-do-infra")]
#[test]
fn test_delete_bucket_hard_delete_strategy() {
    // setup:
    let context = context("fake_orga_id", "fake_cluster_id");
    let secrets = FuncTestsSecrets::new();

    let spaces = Spaces::new(
        context,
        "test-fake".to_string(),
        "test-fake".to_string(),
        secrets.DIGITAL_OCEAN_SPACES_ACCESS_ID.unwrap(),
        secrets.DIGITAL_OCEAN_SPACES_SECRET_ID.unwrap(),
        TEST_REGION,
        BucketDeleteStrategy::HardDelete,
    );

    let bucket_name = format!("qovery-test-bucket-{}", generate_id());

    spaces
        .create_bucket(bucket_name.as_str())
        .expect("error while creating object-storage bucket");

    // compute:
    let result = spaces.delete_bucket(bucket_name.as_str());

    // validate:
    assert!(result.is_ok());
    assert!(!spaces.bucket_exists(bucket_name.as_str()))
}

#[cfg(feature = "test-do-infra")]
#[test]
fn test_delete_bucket_empty_strategy() {
    // setup:
    let context = context("fake_orga_id", "fake_cluster_id");
    let secrets = FuncTestsSecrets::new();

    let spaces = Spaces::new(
        context,
        "test-fake".to_string(),
        "test-fake".to_string(),
        secrets.DIGITAL_OCEAN_SPACES_ACCESS_ID.unwrap(),
        secrets.DIGITAL_OCEAN_SPACES_SECRET_ID.unwrap(),
        TEST_REGION,
        BucketDeleteStrategy::Empty,
    );

    let bucket_name = format!("qovery-test-bucket-{}", generate_id());

    spaces
        .create_bucket(bucket_name.as_str())
        .expect("error while creating object-storage bucket");

    // compute:
    let result = spaces.delete_bucket(bucket_name.as_str());

    // validate:
    assert!(result.is_ok());
    assert!(spaces.bucket_exists(bucket_name.as_str()));

    // clean-up:
    spaces
        .delete_bucket(bucket_name.as_str())
        .unwrap_or_else(|_| panic!("error deleting object storage bucket {}", bucket_name));
}

#[cfg(feature = "test-do-infra")]
#[test]
fn test_create_bucket() {
    // setup:
    let context = context("fake_orga_id", "fake_cluster_id");
    let secrets = FuncTestsSecrets::new();

    let spaces = Spaces::new(
        context,
        "test-fake".to_string(),
        "test-fake".to_string(),
        secrets.DIGITAL_OCEAN_SPACES_ACCESS_ID.unwrap(),
        secrets.DIGITAL_OCEAN_SPACES_SECRET_ID.unwrap(),
        TEST_REGION,
        BucketDeleteStrategy::HardDelete,
    );

    let bucket_name = format!("qovery-test-bucket-{}", generate_id());

    // compute:
    let result = spaces.create_bucket(bucket_name.as_str());

    // validate:
    assert!(result.is_ok());

    // clean-up:
    spaces
        .delete_bucket(bucket_name.as_str())
        .unwrap_or_else(|_| panic!("error deleting object storage bucket {}", bucket_name));
}

#[cfg(feature = "test-do-infra")]
#[test]
fn test_recreate_bucket() {
    // setup:
    let context = context("fake_orga_id", "fake_cluster_id");
    let secrets = FuncTestsSecrets::new();

    let spaces = Spaces::new(
        context,
        "test-fake".to_string(),
        "test-fake".to_string(),
        secrets.DIGITAL_OCEAN_SPACES_ACCESS_ID.unwrap(),
        secrets.DIGITAL_OCEAN_SPACES_SECRET_ID.unwrap(),
        TEST_REGION,
        BucketDeleteStrategy::HardDelete,
    );

    let bucket_name = format!("qovery-test-bucket-{}", generate_id());

    // compute & validate:
    let create_result = spaces.create_bucket(bucket_name.as_str());
    assert!(create_result.is_ok());
    assert!(spaces.bucket_exists(bucket_name.as_str()));

    let delete_result = spaces.delete_bucket(bucket_name.as_str());
    assert!(delete_result.is_ok());
    assert!(!spaces.bucket_exists(bucket_name.as_str()));

    let recreate_result = spaces.create_bucket(bucket_name.as_str());
    assert!(recreate_result.is_ok());
    assert!(spaces.bucket_exists(bucket_name.as_str()));

    // clean-up:
    spaces
        .delete_bucket(bucket_name.as_str())
        .unwrap_or_else(|_| panic!("error deleting object storage bucket {}", bucket_name));
}

#[cfg(feature = "test-do-infra")]
#[test]
fn test_put_file() {
    // setup:
    let context = context("fake_orga_id", "fake_cluster_id");
    let secrets = FuncTestsSecrets::new();

    let spaces = Spaces::new(
        context,
        "test-fake".to_string(),
        "test-fake".to_string(),
        secrets.DIGITAL_OCEAN_SPACES_ACCESS_ID.unwrap(),
        secrets.DIGITAL_OCEAN_SPACES_SECRET_ID.unwrap(),
        TEST_REGION,
        BucketDeleteStrategy::HardDelete,
    );

    let bucket_name = format!("qovery-test-bucket-{}", generate_id());
    let object_key = format!("test-object-{}", generate_id());

    spaces
        .create_bucket(bucket_name.as_str())
        .expect("error while creating object-storage bucket");

    let temp_file = NamedTempFile::new().expect("error while creating tempfile");

    // compute:
    let result = spaces.put(
        bucket_name.as_str(),
        object_key.as_str(),
        temp_file.into_temp_path().to_str().unwrap(),
    );

    // validate:
    assert!(result.is_ok());
    assert!(spaces.get(bucket_name.as_str(), object_key.as_str(), false).is_ok());

    // clean-up:
    spaces
        .delete_bucket(bucket_name.as_str())
        .unwrap_or_else(|_| panic!("error deleting object storage bucket {}", bucket_name));
}

#[cfg(feature = "test-do-infra")]
#[test]
fn test_get_file() {
    // setup:
    let context = context("fake_orga_id", "fake_cluster_id");
    let secrets = FuncTestsSecrets::new();

    let spaces = Spaces::new(
        context,
        "test-fake".to_string(),
        "test-fake".to_string(),
        secrets.DIGITAL_OCEAN_SPACES_ACCESS_ID.unwrap(),
        secrets.DIGITAL_OCEAN_SPACES_SECRET_ID.unwrap(),
        TEST_REGION,
        BucketDeleteStrategy::HardDelete,
    );

    let bucket_name = format!("qovery-test-bucket-{}", generate_id());
    let object_key = format!("test-object-{}", generate_id());

    spaces
        .create_bucket(bucket_name.as_str())
        .expect("error while creating object-storage bucket");

    let temp_file = NamedTempFile::new().expect("error while creating tempfile");
    let tempfile_path = temp_file.into_temp_path();
    let tempfile_path = tempfile_path.to_str().unwrap();

    spaces
        .put(bucket_name.as_str(), object_key.as_str(), tempfile_path)
        .unwrap_or_else(|_| panic!("error while putting file {} into bucket {}", tempfile_path, bucket_name));

    // compute:
    let result = spaces.get(bucket_name.as_str(), object_key.as_str(), false);

    // validate:
    assert!(result.is_ok());
    assert!(spaces.get(bucket_name.as_str(), object_key.as_str(), false).is_ok());

    // clean-up:
    spaces
        .delete_bucket(bucket_name.as_str())
        .unwrap_or_else(|_| panic!("error deleting object storage bucket {}", bucket_name));
}
