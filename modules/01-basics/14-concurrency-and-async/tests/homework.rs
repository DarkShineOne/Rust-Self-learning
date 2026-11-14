use std::time::Duration;

use basics_14_async::{
    Feedback, ReceiveError, SharedStore, StoreError, collect_ratings, receive_with_timeout,
};
use tokio::sync::mpsc;

fn feedback(id: u64, rating: u8) -> Feedback {
    Feedback { id, rating }
}

#[tokio::test]
async fn shared_store_supports_put_get_and_duplicate() {
    let store = SharedStore::new();
    let clone = store.clone();
    store.put(feedback(1, 5)).await.unwrap();
    assert_eq!(clone.get(1).await, Some(feedback(1, 5)));
    assert_eq!(
        clone.put(feedback(1, 1)).await,
        Err(StoreError::DuplicateId)
    );
    assert_eq!(store.get(2).await, None);
}

#[tokio::test]
async fn receives_message_before_timeout() {
    let (sender, mut receiver) = mpsc::channel(1);
    sender.send(feedback(7, 4)).await.unwrap();
    let result = receive_with_timeout(&mut receiver, Duration::from_millis(100)).await;
    assert_eq!(result, Ok(feedback(7, 4)));
}

#[tokio::test]
async fn distinguishes_closed_channel_and_timeout() {
    let (sender, mut receiver) = mpsc::channel::<Feedback>(1);
    drop(sender);
    assert_eq!(
        receive_with_timeout(&mut receiver, Duration::from_millis(10)).await,
        Err(ReceiveError::Closed)
    );

    let (_sender, mut receiver) = mpsc::channel::<Feedback>(1);
    assert_eq!(
        receive_with_timeout(&mut receiver, Duration::from_millis(10)).await,
        Err(ReceiveError::Timeout)
    );
}

#[tokio::test]
async fn collects_sorted_ratings_from_multiple_stores() {
    let first = SharedStore::new();
    let second = SharedStore::new();
    first.put(feedback(1, 5)).await.unwrap();
    first.put(feedback(2, 1)).await.unwrap();
    second.put(feedback(3, 4)).await.unwrap();
    second.put(feedback(4, 3)).await.unwrap();
    assert_eq!(collect_ratings(vec![first, second]).await, [1, 3, 4, 5]);
    assert_eq!(collect_ratings(Vec::new()).await, Vec::<u8>::new());
}
