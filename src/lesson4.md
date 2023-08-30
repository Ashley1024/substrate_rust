# lesson 4 

## rust enum and pattern match

### special case if let 
### when you only care about 1 branch
### It lose the exhaustiveness check
```
let some_u8_value = Some(3);
match some_u8_value{
    Some()=> println!("three"),
    _=>(),
}

// equal 

if let Some(3) = some_u8_value{
    println!("three");
}
```

## ownership


## generic,trait and lifecycle
    

## proj management