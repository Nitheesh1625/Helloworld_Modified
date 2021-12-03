/*#include <linux/input.h>
#include <linux/serio.h>
#include <linux/libps2.h>
#include <linux/slab.h>
#include <linux/module.h>
#include <asm/hypervisor.h>
*/
#define _VMMOUSE_H

#define VMMOUSE_PSNAME  "VirtualPS/2"

extern void input_report_key_wrapper(struct input_dev *dev, unsigned int code, int value);

extern void psmouse_dbg_wrapper(struct psmouse *psmouse);

extern void psmouse_reset_wrapper(struct psmouse *psmouse);

int vmmouse_detect(struct psmouse *psmouse, bool set_properties);

int vmmouse_init(struct psmouse *psmouse);
