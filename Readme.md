This repository is a learning repository and  
contains the code I wrote while learning to   
use/write `macros` in Rust.  
  
  

I will be including some learning resources  
along the code while writing it. If somebody  
wants to go down the same path.     
  
  
  
# Resources:

- The Little Book of Rust macros: `https://lukaswirth.dev/tlborm/`


# Some important points: 

- There is a special metavariable `$crate` which can be used to  
refer to the current crate.  
- A `const fn` in rust can be called at compile time and can also  
be used inside a macro though it cannot read/write files or do things  
that might depend on `IO`. However, it can be used for pattern matching.  
