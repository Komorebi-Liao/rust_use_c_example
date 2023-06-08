# rust_use_c_example
## Example1 
see in https://doc.rust-lang.org/nomicon/ffi.html

Note that when using this method with dynamic libraries(.so), you may encounter issues where the system cannot find the dynamic library.You can find a similar description at this link: https://stackoverflow.com/questions/60236701/how-to-use-a-compiled-c-so-file-in-rust .You can resolve this by following either of the two methods below:

- move the .so file to /usr/local/lib

or
- Specify the library search path: You can use the LD_LIBRARY_PATH environment variable to specify the directories where the dynamic libraries are located.  eg:  
```LD_LIBRARY_PATH=/your/lib/path cargo run``` 

  Set the variable to include the directory containing the required dynamic library before running the program. 
Update the system's library cache: Alternatively, you can update the system's library cache using the ldconfig command. This command updates the cache and allows the system to locate the dynamic library correctly. Run the command with root privileges: ``` sudo ldconfig ```

## Example2 
This example use ```cc::Build``` to compile the c file. This approach doesn't require pre-compiling the C file in advance and doesn't have the aforementioned dynamic library linking issues.

## Example3
This example use ```libloading``` to load .so file. 