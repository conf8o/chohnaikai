# chohnaikai

A system for items that is interesting to closed community.

For example, it notify game clips to Discord and aggregate reactions on them, and sort them.

Also, users can implement particular ways to do them.

This system is in develop yet.

# Features

## Storage

Users can extend to use their particular storage service by implementing `Storage` protocol.

So far, this project plan to support local file system and Google Drive by default.

## Rules for Sorting

Users can set their rules for sorting weighted items. (Sorted items that is going to store into a paticular place is called "Shipment")

```rust
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
```

This example means following.

1. Up to the second items will be moved to "A" folder.
2. items with 5 or more weights will be moved to "B" folder.
3. The other items will be moved to "C" folder.

## Notice

Users can extend to use their particular communication service by implementing `Notice` protocol.

So far, this project plan to support Discord by default.

## Aggregate

Users can extend to use their particular way to aggregate reactions that are attached on items by implementing `Aggregate` protocol.
