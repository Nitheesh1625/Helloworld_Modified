obj-m := vmmouse.o
sync_example-objs := target/x86_64-linux-kernel-module/debug/libsync_example.a src/c_wrapper.o
EXTRA_LDFLAGS += --entry=init_module

KDIR = /lib/modules/$(shell uname -r)/build
CMD_LINE =

all:
	$(MAKE) $(CMD_LINE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
