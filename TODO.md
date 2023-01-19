
* BUILD_OS -> CRYPTEIA_BUILD_OS
* BUILD_TARGET -> CRYPTEIA_BUILD_TARGET
* Can some BUILD_TARGET be removed?
* Docs around using containers to pull binaries.

## x86_64-unknown-linux-gnu

> ls ./target/x86_64-unknown-linux-gnu/release/
total 12M
drwxrwxrwx+  25 vscode 4.0K Jan 20 00:43 build/
-rw-rw-rw-    1 vscode    0 Jan 20 00:43 .cargo-lock
-rwxrwxrwx    2 vscode  11M Jan 20 00:45 crypteia
-rw-rw-rw-    1 vscode  168 Jan 20 00:45 crypteia.d
drwxrwxrwx+   2 vscode  28K Jan 20 00:45 deps/
drwxrwxrwx+   2 vscode 4.0K Jan 20 00:43 examples/
drwxrwxrwx+ 150 vscode  12K Jan 20 00:43 .fingerprint/
drwxrwxrwx+   2 vscode 4.0K Jan 20 00:43 incremental/
-rw-rw-rw-    1 vscode  141 Jan 20 00:45 libcrypteia.d
-rwxrwxrwx    2 vscode 435K Jan 20 00:45 libcrypteia.so

> ldd ./target/x86_64-unknown-linux-gnu/release/libcrypteia.so 
        linux-vdso.so.1 (0x00007fff6ddf7000)
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007f037e86a000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f037e850000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f037e82e000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f037e659000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f037e8eb000)

## aarch64-unknown-linux-gnu

> ls ./target/aarch64-unknown-linux-gnu/release/
total 11M
drwxrwxrwx+  25 vscode 4.0K Jan 20 13:12 build/
-rw-rw-rw-    1 vscode    0 Jan 20 13:12 .cargo-lock
-rwxrwxrwx    2 vscode 9.7M Jan 20 13:15 crypteia
-rw-rw-rw-    1 vscode  113 Jan 20 13:15 crypteia.d
drwxrwxrwx+   2 vscode  36K Jan 20 13:15 deps/
drwxrwxrwx+   2 vscode 4.0K Jan 20 13:12 examples/
drwxrwxrwx+ 150 vscode  12K Jan 20 13:12 .fingerprint/
drwxrwxrwx+   2 vscode 4.0K Jan 20 13:12 incremental/
-rw-rw-rw-    1 vscode   98 Jan 20 13:15 libcrypteia.d
-rwxrwxrwx    2 vscode 383K Jan 20 13:14 libcrypteia.so

> ldd ./target/aarch64-unknown-linux-gnu/release/libcrypteia.so 
        not a dynamic executable

