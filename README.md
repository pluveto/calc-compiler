# Calc - A simple calculator

## Pre-requisites

We will take Mac OS as an example, but you can use any other OS. 

- [Homebrew](https://brew.sh/)
- [LLVM](https://llvm.org/)

Install LLVM with Homebrew:

```sh
brew install llvm
```

Output:

```md
To use the bundled libc++ please add the following LDFLAGS:
  LDFLAGS="-L/opt/homebrew/opt/llvm/lib/c++ -Wl,-rpath,/opt/homebrew/opt/llvm/lib/c++"

llvm is keg-only, which means it was not symlinked into /opt/homebrew,
because macOS already provides this software and installing another version in
parallel can cause all kinds of trouble.

If you need to have llvm first in your PATH, run:
  echo 'export PATH="/opt/homebrew/opt/llvm/bin:$PATH"' >> ~/.zshrc

For compilers to find llvm you may need to set:
  export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
  export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"
```

You need to add the following lines to your `~/.zshrc` or `~/.bashrc`:

```sh
export PATH="/opt/homebrew/opt/llvm/bin:$PATH
```

## Run

```sh
cargo run -- -e "1;-mem*3-1;print(mem);print(mem+7);" | lli -
```

Output:

```sh
-4
3
```
