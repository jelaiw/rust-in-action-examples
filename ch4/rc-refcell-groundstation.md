```sh
$ cargo run
   Compiling rc-refcell-groundstation v0.1.0 (/workspaces/rust-in-action-examples/ch4/rc-refcell-groundstation)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/rc-refcell-groundstation`
base: RefCell { value: GroundStation { radio_freq: 87.65 } }
base_2: GroundStation { radio_freq: 75.31 }
base: RefCell { value: GroundStation { radio_freq: 75.31 } }
base: RefCell { value: <borrowed> }
base_3: GroundStation { radio_freq: 118.52000000000001 }
```
