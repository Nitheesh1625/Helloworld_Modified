
obj-m			+= test.o

psmouse-objs := src/psmouse-base.o 

psmouse-m    += src/vmmouse.o



helloworld-objs := target/x86_64-linux-kernel-module/$(BUILD)/libhello_world.a

KDIR = /lib/modules/$(shell uname -r)/build

src/vmmouse.o: src/lib.rs
	$(MAKE) $(CMD_LINE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
