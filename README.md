# Grid-Cell

Library for Japanese traditional geometry standard aka "地域メッシュ" or "標準地域メッシュ".

## Usage

```rust
let spot = Spot::new(Coordinate {
    longitude: 139.733231,
    latitude: 35.680916,
});

// 1次メッシュの地域メッシュコード
let actual = spot.as_first_cell().code;
let expected = 5339;
assert_eq!(actual, expected);

// 2次メッシュの地域メッシュコード
let actual = spot.as_second_cell().code;
let expected = 533945;
assert_eq!(actual, expected);

// 3次メッシュの地域メッシュコード
let actual = spot().as_third_cell().code;
let expected = 53394518;
assert_eq!(actual, expected);

// 4次メッシュ (2分の1地域メッシュ) の地域メッシュコード
let actual = spot.as_fourth_cell().code;
let expected = 533945184;
assert_eq!(actual, expected);
```
