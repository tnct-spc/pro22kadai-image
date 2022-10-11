require 'rutie'

Rutie.new(:pro22kadai_image, lib_path: 'target/release').init 'init_ffitest', __dir__

put ffi_test.ffi_test('Hello, world!')
