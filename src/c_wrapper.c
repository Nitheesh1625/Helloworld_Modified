#include <linux/mutex.h>
#include <linux/spinlock.h>
#include <linux/uaccess.h>
#include <linux/netdevice.h>
#include <linux/mii.h>
#include <linux/usb.h>
#include <linux/usb/usbnet.h>
#include <linux/of_net.h>
#include <linux/skbuff.h>
#include <linux/sched.h>
#include <linux/input.h>
#include <linux/serio.h>
#include <linux/libps2.h>
#include <linux/slab.h>
#include <linux/module.h>
#include <asm/hypervisor.h>

#include "psmouse.h"
#include "vmmouse.h"


long copy_to_user_wrapper(void __user *to, const void *from, unsigned long n) {
    return _copy_to_user(to, from, n);
}

void spin_lock_init_wrapper(spinlock_t *lock) { spin_lock_init(lock); }
void spin_lock_wrapper(spinlock_t *lock) { spin_lock(lock); }
void spin_unlock_wrapper(spinlock_t *lock) { spin_unlock(lock); }

void mutex_init_wrapper(struct mutex *lock) { mutex_init(lock); }
void mutex_lock_wrapper(struct mutex *lock) { mutex_lock(lock); }
void mutex_unlock_wrapper(struct mutex *lock) { mutex_unlock(lock); }

void skb_set_tail_pointer_wrapper(struct sk_buff *skb, const int offset) {
    skb_set_tail_pointer(skb, offset);
}

int usbnet_read_cmd_wrapper(struct usbnet *dev, u8 cmd, u8 reqtype, u16 value,
                    u16 index, void *data, u16 size) {
    return usbnet_read_cmd(dev, cmd, reqtype, value, index, data, size);
}

void bug_helper(void) {
    schedule();
}

void input_report_key_wrapper(struct input_dev *dev, unsigned int code, int value) {
    input_report_key(dev, code, value);
}

void psmouse_dbg_wrapper(struct psmouse *psmouse, char string[100]) {
    psmouse_dbg(psmouse, string);
}
