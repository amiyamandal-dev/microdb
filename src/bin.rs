extern crate sled;

use sled::{Config, Result};

fn basic() -> Result<()> {
    let config = Config::new().temporary(true);

    let db = config.open()?;

    let k = b"k".to_vec();
    let v1 = b"v1".to_vec();
    let v2 = b"v2".to_vec();

    // set and get
    db.insert(k.clone(), v1.clone())?;
    assert_eq!(db.get(&k).unwrap().unwrap(), (v1.clone()));

    // compare and swap
    match db.compare_and_swap(k.clone(), Some(&v1.clone()), Some(v2.clone()))? {
        Ok(()) => println!("it worked!"),
        Err(sled::CompareAndSwapError { current: cur, proposed: _ }) => {
            println!("the actual current value is {:?}", cur)
        }
    }

    // scan forward
    let mut iter = db.range(k.as_slice()..);
    let (k1, v1) = iter.next().unwrap().unwrap();
    assert_eq!(v1, v2.clone());
    assert_eq!(k1, k.clone());
    assert_eq!(iter.next(), None);

    // deletion
    db.remove(&k)?;

    Ok(())
}

fn main(){
    basic().unwrap();
    
}