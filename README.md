# rs-car-sync

This is a fork of [rs-car](https://github.com/dapplion/rs-car) that only uses standard sync rust api.

The main motivation is to get rid of useless contaminating async api when reading a car file from memory.

Rust implementation of the [CAR specifications](https://ipld.io/specs/transport/car/), both [CARv1](https://ipld.io/specs/transport/car/carv1/) and [CARv2](https://ipld.io/specs/transport/car/carv2/).

## Usage

```rs
let mut file = std::fs::File::open(car_filepath).unwrap();
let block_iterator = decode_car_stream(&mut file, true).unwrap();

while let Some(item) = block_iterator.next() {
    let (cid, block) = item.unwrap();
    // Do something with CAR block
}
```
