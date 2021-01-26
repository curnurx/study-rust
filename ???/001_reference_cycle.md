

### Reference-cycle using Rc and RefCell.
An example-code of reference-cycle in the Guide book is too long. So I wrote a short one.

``` rust
use std::rc::Rc;
use std::cell::RefCell;
use Data::{Next, Nil};

#[derive(Debug)]
enum Data {
	Next(Rc<RefCell<Data>>),
	Nil,
}

fn main() {	
	let x = Rc::new(RefCell::new(Nil));
	*x.borrow_mut() = Next(Rc::clone(&x));
	println!("{:?}", x);
}

```

It works like the following cpp-code.

``` cpp
struct Data {
  Data* next;
}

int main() {
  Data* x;
  x->next = x;
}
```
