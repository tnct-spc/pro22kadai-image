import ctypes
import ffi
import base64


def ffi_test(message):
    lib = ctypes.cdll.LoadLibrary('target/release/libpro22kadai_image.so')
    lib.ffi_test.argtype = ctypes.POINTER(ctypes.c_uint8)
    lib.ffi_test.restype = ctypes.POINTER(ctypes.c_uint8)

    return lib.ffi_test(message)


def get_points(encoded_img):
    lib = ctypes.cdll.LoadLibrary('target/release/libpro22kadai_image.so')
    lib.get_points.argtype = ctypes.pointer
    lib.get_points.argtype = ctypes.pointer

    return lib.get_points(encoded_img)


def main():
    ret = ffi_test("Hello, world!")
    print(ret)


if __name__ == "__main__":
    main()
