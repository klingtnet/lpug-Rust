# inspired by https://gist.github.com/seanjensengrey/f5d73bbdf22cfa1ad463

try:
    from cffi import FFI
except ImportError:
    print("pip install cffi")


def main():
    ffi = FFI()
    lib = ffi.dlopen("./target/debug/libsimple_cffi.so")

    print(lib)

    ffi.cdef('long square(int);')

    x = 50
    print("{}*{} = {}".format(x, x, lib.square(x)))

if __name__ == '__main__':
    main()
