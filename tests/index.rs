#![allow(unused_imports)]
#![allow(unused_parens)]

use log::trace;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};

use crate::common::{collection, connection};
use arangors::{
    collection::{
        options::{ChecksumOptions, PropertiesOptions},
        response::Status,
        CollectionType,
    },
    document::options::InsertOptions,
    index::{Index, IndexSettings, VectorIndexParams},
    ClientError, Connection, Document,
};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};

pub mod common;

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_persistent_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_persistent_test";
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();

    let index = Index::builder()
        .name(index_name)
        .fields(vec!["password".to_string()])
        .settings(IndexSettings::Persistent {
            unique: true,
            sparse: false,
            deduplicate: false,
        })
        .build();

    let index = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    let delete_result = database.delete_index(&index.id).await.unwrap();

    assert!(index.id.len() > 0);
    assert_eq!(index.name, index_name.to_string());
    assert_eq!(delete_result.id, index.id);

    if let IndexSettings::Persistent {
        unique,
        sparse,
        deduplicate,
    } = index.settings
    {
        assert_eq!(unique, true);
        assert_eq!(sparse, false);
        assert_eq!(deduplicate, false);
    }
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_hash_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_hash_test";
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();

    let index = Index::builder()
        .name(index_name)
        .fields(vec!["password".to_string()])
        .settings(IndexSettings::Hash {
            unique: false,
            sparse: true,
            deduplicate: true,
        })
        .build();

    let index = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    let delete_result = database.delete_index(&index.id).await.unwrap();

    assert!(index.id.len() > 0);
    assert_eq!(index.name, index_name.to_string());
    assert_eq!(delete_result.id, index.id);

    if let IndexSettings::Hash {
        unique,
        sparse,
        deduplicate,
    } = index.settings
    {
        assert_eq!(unique, false);
        assert_eq!(sparse, true);
        assert_eq!(deduplicate, true);
    }
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_skiplist_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_skiplist_test";
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();

    let index = Index::builder()
        .name(index_name)
        .fields(vec!["password".to_string()])
        .settings(IndexSettings::Skiplist {
            unique: true,
            sparse: false,
            deduplicate: false,
        })
        .build();

    let index = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    let delete_result = database.delete_index(&index.id).await.unwrap();

    assert!(index.id.len() > 0);
    assert_eq!(index.name, index_name.to_string());
    assert_eq!(delete_result.id, index.id);

    if let IndexSettings::Skiplist {
        unique,
        sparse,
        deduplicate,
    } = index.settings
    {
        assert_eq!(unique, true);
        assert_eq!(sparse, false);
        assert_eq!(deduplicate, false);
    }
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_geo_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_geo_test";
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();

    let index = Index::builder()
        .name(index_name)
        .fields(vec!["password".to_string()])
        .settings(IndexSettings::Geo { geo_json: false })
        .build();

    let index = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    let delete_result = database.delete_index(&index.id).await.unwrap();

    assert!(index.id.len() > 0);
    assert_eq!(index.name, index_name.to_string());
    assert_eq!(delete_result.id, index.id);

    if let IndexSettings::Geo { geo_json } = index.settings {
        assert_eq!(geo_json, false);
    }
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_ttl_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_ttl_test";
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();

    let index = Index::builder()
        .name(index_name)
        .fields(vec!["password".to_string()])
        .settings(IndexSettings::Ttl { expire_after: 500 })
        .build();

    let index = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    let delete_result = database.delete_index(&index.id).await.unwrap();

    assert!(index.id.len() > 0);
    assert_eq!(index.name, index_name.to_string());
    assert_eq!(delete_result.id, index.id);

    if let IndexSettings::Ttl { expire_after } = index.settings {
        assert_eq!(expire_after, 500);
    }
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_fulltext_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_full_test";
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();

    let index = Index::builder()
        .name(index_name)
        .fields(vec!["password".to_string()])
        .settings(IndexSettings::Fulltext { min_length: 100 })
        .build();

    let index = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    let delete_result = database.delete_index(&index.id).await.unwrap();

    assert!(index.id.len() > 0);
    assert_eq!(index.name, index_name.to_string());
    assert_eq!(delete_result.id, index.id);

    if let IndexSettings::Fulltext { min_length } = index.settings {
        assert_eq!(min_length, 100);
    }
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_vector_index() {
    test_setup();
    let collection_name = "test_collection_vector";
    let index_name = "idx_vector_test";
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();

    database.drop_collection(collection_name).await.unwrap();
    let collection = database
        .create_collection(collection_name)
        .await
        .expect("Failed to create vector collection");

    // document required for index creation
    let vector_doc = json!({
        "embedding": vec![0.0; 256],
    });

    collection
        .create_document(vector_doc, InsertOptions::default())
        .await
        .unwrap();

    let index = Index::builder()
        .name(index_name)
        .fields(vec!["embedding".to_string()])
        .settings(IndexSettings::Vector {
            params: VectorIndexParams {
                metric: "cosine".to_string(),
                dimension: 256,
                n_lists: 1,
                ..Default::default()
            },
        })
        .build();

    println!("Creating index: {:?}", serde_json::to_string(&index));
    let index = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    assert!(index.id.len() > 0);
    assert_eq!(index.name, index_name.to_string());

    if let IndexSettings::Vector { params } = index.settings {
        assert_eq!(params.metric, "cosine");
        assert_eq!(params.dimension, 256);
        assert_eq!(params.n_lists, 1);
    }

    let list = database.indexes(collection_name).await.unwrap();

    println!("List of indexes: {:?}", list);
    assert!(list.indexes.len() > 0);

    let delete_result = database.delete_index(&index.id).await.unwrap();
    assert_eq!(delete_result.id, index.id);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_list_indexes() {
    test_setup();
    let collection_name = "test_collection";
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();
    let list = database.indexes(collection_name).await.unwrap();

    assert!(list.indexes.len() > 0);
}
