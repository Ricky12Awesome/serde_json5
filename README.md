# serde_json5

A [Serde](https://serde.rs/) serializer / deserializer for [Json5](https://json5.org/)

## TODO

- [ ] **Implement working serializer**
- [ ] **Implement working deserializer**
- [ ] **Good Documentation**
- [ ] **Good Error handling**
- [ ] **Attributes** *(Example: `#[json5(comment = "")]`)*
- [ ] **Format Settings** *(Example: `unquated = true`)*

## Goal
```rust
#[derive(Default, Serialize, Deserialize)]
#[json5(format = array)]
struct Color {
  r: u8,
  g: u8,
  b: u8,
}

#[derive(Default, Serialize, Deserialize)]
struct Image {
  #[json5(comment = "Width")] 
  width: u32,
  #[json5(comment = "Height")] 
  height: u32,
  #[json5(comment = "1D Array of all the pixels")]
  #[json5(comment = "Format: [R, G, B]")]
  pixels: Vec<Color>
}
```

Would result in

```json5
{
  // Width
  "width": 2,
  // Height,
  "height": 2,
  // 1D Array of all the pixels
  // Format: [R, G, B]
  "pixels": [
    [0, 0, 255],
    [0, 255, 0],
    [255, 0, 0],
    [255, 0, 255]
  ]
}
```