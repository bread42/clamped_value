## Usage

Add the following to your Cargo.toml

```toml
[dependencies]
clamped_value = "0.1.0"
```

## Guide

#### Creation of a `ClampedValue`

Create a new `ClampedValue` using the function below, where : 
- `min` is the minimum value
- `value` is the current value
- `max` is the max value

```rust
ClampedValue::new(min, value, max)
```

#### Setting the current value

To set the current value of a `ClampedValue`, use the following function :
```rust 
ClampedValue::set(&mut self, new_value)
```

If `new_value` is outside of the minimum or maximum, it will be set to the 
relevant bound.

#### Mathmatical operations on the current value

Use any of the following operators on a `ClampedValue` to perform mathmatical 
operations on its current value : 

- AddAssign (+=)
- SubAssign (-=)
- MulAssign (*=)
- DivAssign (/=)

All of these operations are saturated, meaning that if the result is outside
the minimum or maximum of the `ClampedValue`, the current value will be set 
to the relevant bound.