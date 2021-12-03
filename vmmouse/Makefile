
obj-m := psmouse.o

psmouse-objs := src/psmouse-base.o src/rust.o



src/rust.o: Cargo.toml src/psmouse-base.c src/lib.rs 
	cargo rustc --release -Z build-std=core,alloc --target=x86_64-unknown-none-linuxkernel	
	mkdir -p rust_objs
	cd rust_objs && ar x ../target/x86_64-unknown-none-linuxkernel/release/libpsmouse.a
	ld -r -o src/rust.o rust_objs/*.o
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules
clean:
	cargo clean
	rm -rf rust_objs
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
