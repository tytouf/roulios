define debug
    target remote localhost:1234
    symbol-file target/cortex-m3/release/nk.elf
    break rust_begin_unwind
end

debug

