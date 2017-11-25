from cffi import FFI

ffi = FFI() #create FFI object

#C function signature matching Rust "snapping_mech"
ffi.cdef( """
    double snapping_mechanism(double, double, double);
""")

#open SO file
#this will be a DLL file if on windows
C = ffi.dlopen("/home/osboxes/Desktop/DP/target/debug/libsnapping_mech.so")

#make a call to the snapping mechanism in rust file
print(C.snapping_mechanism(12.0, 2.0, 1000.0))