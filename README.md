## Running on Windows

some of the modules use openblas-src.

To get this working you need [vcpkg](https://github.com/Microsoft/vcpkg#quick-start-windows) from microsoft repo.

Install it according to the instructions and move it to your path, e.g. ~/.local/bin . 

Additionally create an environment variable *VCPKG_ROOT* which points to the repo you checked out.

After that you can 

```
> vcpkg integrate install
> vcpkg install openblas --triplet x64-windows
```

The cargo build process should now find the openblas library for linking.

