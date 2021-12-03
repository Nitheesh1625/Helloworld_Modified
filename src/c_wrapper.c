#include <linux/sched.h>
#include <linux/input.h>
#include <linux/serio.h>
#include <linux/libps2.h>
#include <linux/slab.h>
#include <linux/module.h>
#include <asm/hypervisor.h>

#include "psmouse.h"
#include "vmmouse.h"



void input_report_key_wrapper(struct input_dev *dev, unsigned int code, int value) {
    input_report_key(dev, code, value);
}

void psmouse_dbg_wrapper(struct psmouse *psmouse) {
    psmouse_dbg(psmouse, "Hypervisor is not supported");
}

void psmouse_reset_wrapper(struct psmouse *psmouse) {
    psmouse_reset(psmouse);
}

