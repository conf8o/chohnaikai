pub mod storage;
pub use storage::{ Storage, PatternError, ToPathBuf, ToPathBufError, search };

pub mod config;

pub mod shipment;
pub use shipment::{ Shipment, ship, sorted_shipments };

pub mod notice;
pub use notice::{ Notice, NoticeError };

pub mod aggregate;
pub use aggregate:: { Aggregate, AggregateError };

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::iter::FromIterator;
    use std::path::Path;
    use std::vec::IntoIter;
    use super::*;

    #[test]
    fn test_config() {
        let config = &config::CONFIG;
        let shared = &config.shared_dir;
        assert_eq!("0.shared", shared);
    }

    #[test]
    fn test_shipment() {
        let storage = storage::LocalStorage{};
        let items = Vec::from_iter(search(&storage, "tests/resources/*").unwrap());
        let original_len = items.len();
        assert!(original_len > 0);

        let weights: HashMap<&str, i32> = [("0.txt", 4), ("1.txt", 6), ("2.txt", 3), ("3.txt", 5)].iter().cloned().collect();
        
        let shipments = sorted_shipments(items.into_iter(), &weights,
            |(i, (item, &weight))| {
                if i < 2 {
                    Shipment(item, "A".to_string())
                } else if weight > 3 {
                    Shipment(item, "B".to_string())
                } else {
                    Shipment(item, "C".to_string())
                }
            }
        );
        ship(&storage, shipments).unwrap();

        let items = Vec::from_iter(search(&storage, "tests/resources/*").unwrap());
        let a = Vec::from_iter(search(&storage, "tests/A/*").unwrap());
        let b = Vec::from_iter(search(&storage, "tests/B/*").unwrap());
        let c = Vec::from_iter(search(&storage, "tests/C/*").unwrap());

        assert_eq!(items.len(), 0);
        assert_eq!(a.len(), 2);
        assert_eq!(b.len(), 1);
        assert_eq!(c.len(), 1);
        let back = sorted_shipments(a.into_iter().chain(b).chain(c), &weights,
            |(_, (item, &_))| {
                Shipment(item, "resources".to_string())
            }
        );
        ship(&storage, back).unwrap();
        let items = Vec::from_iter(search(&storage, "tests/resources/*").unwrap());

        assert_eq!(original_len, items.len());
    }

    #[test]
    fn test_local_storage() {
        let storage = storage::LocalStorage{};
        let clips = search(&storage, "tests/resources/*").unwrap();
        for (i, clip) in clips.enumerate() {
            assert_eq!(
                Path::new(&format!("tests/resources/{}.txt", i)),
                clip
            );
        }
    }

    // #[test]
    // fn test_google_drive() {
    //     let storage = storage::GoogleDrive{};
    //     let clips = storage::search(&storage, "tests/resources/*").unwrap();
    //     for (i, clip) in clips.enumerate() {
    //         assert_eq!(
    //             Path::new(&format!("tests/resources/{}.txt", i)),
    //             clip
    //         );
    //     }
    // }
    
    struct Standard {}

    impl Notice for Standard {
        type Content = String;

        fn send(&self, content: String) -> Result<(), NoticeError> {
            println!("{:?}", content);
            Ok(())
        }
    }

    #[test]
    fn test_notice() {
        let standard = Standard{};
        standard.send("Hello".to_string()).unwrap();
    }

    impl Aggregate for Standard {
        type Item = String;
        type Items = IntoIter<Self::Item>;
        fn aggregate(&self) -> Result<Self::Items, AggregateError> {
            use std::num::ParseIntError;
            let x: Result<Vec<String>, ParseIntError> = (0..3)
                .map(|i| {
                    let s = i.to_string();
                    let i = s.parse::<i32>()?;
                    Ok(i.to_string())
                })
                .collect();
            match x {
                Ok(v) => Ok(v.into_iter()),
                Err(_) => Err(AggregateError { msg: "aggregate error"})
            }
        }
    }

    #[test]
    fn test_aggregate() {
        let standard = Standard{};
        for line in standard.aggregate().unwrap() {
            println!("{:}", line);
        }
    }
}
