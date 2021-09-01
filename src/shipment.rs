
use std::cmp::Reverse;
use std::collections::HashMap;
use std::io;
use std::path::{ PathBuf };
use crate::{ Storage };

pub struct Shipment(pub PathBuf, pub String);

pub fn ship<S: Storage>(storage: &S, shipments: Vec<Shipment>) -> io::Result<()> {
    shipments
        .iter()
        .filter_map(|Shipment(path, dst)| {
            let file_name = path.file_name()?;
            let mut dst_dir = path.clone();
            dst_dir.pop();
            dst_dir.pop();
            dst_dir.push(dst);
            Some((path, dst_dir, file_name))
        })
        .map(|(path, mut dst_dir, file_name)| {
            if !storage.exists(&dst_dir) {
                storage.create_dir_all(&dst_dir)?;
            }
            dst_dir.push(file_name);

            let new_path = dst_dir;
            println!("rename {:} to {:}", path.display(), new_path.display());
            storage.rename(path, new_path)
        })
        .collect()
}

pub fn sorted_shipments<I, W, F>(items: I, weights: &HashMap<&str, W>, condition: F) -> Vec<Shipment>
where
    I: Iterator<Item=PathBuf>,
    W: Ord,
    F: Fn((usize, (PathBuf, &W))) -> Shipment
{
    let mut v = items
        .filter_map(|item| {
            let f = item.file_name()?.to_str()?;
            let weight = weights.get(f)?;
            Some((item, weight))
        })
        .collect::<Vec<(PathBuf, &W)>>();
    v.sort_by_key(|&(_, weight)| Reverse(weight));

    v.into_iter()
     .enumerate()
     .map(condition)
     .collect()
}