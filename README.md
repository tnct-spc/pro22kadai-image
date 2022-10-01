# pro22kadai-image
## Build environment to run Rust
In Windows 10/11:
```PowerShell
winget install rustup
```

In Arch Linux:
```Bash
$ sudo pacman -S rustup
$ rustup default stable
```

Other Unix/Linux
```Bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Clone this project
```Bash
$ git clone https://github.com/tnct-spc/pro22kadai-image -b main
$ cd pro22kadai-image/
```

To build and run
```Bash
$ cargo run
```

Build without run
```Bash
$ cargo build
```