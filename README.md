# chohnaikai

A system for items that is interesting to closed community.

For example, it notify game clips to Discord and survey reactions on them, and sort them.

Also, users can implement particular ways to do them.

This system is in develop yet.

# Features

## Storage

Users can extend to use their particular storage service by implementing `Storage` trait.

So far, this project plan to support local file system and Google Drive by default.

## Rules for Shipment

Users can set their rules to sort weighted items.

The sorted items is going to store into a paticular storage as shipments.

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

Users can extend to use their particular communication service by implementing `Notice` trait.

So far, this project plan to support Discord by default.

## Survey reactions

Users can extend to use their particular way to survey reactions that are attached on items by implementing `Survey` protocol.
