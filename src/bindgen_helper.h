#include <linux/sched.h>
#include <linux/module.h>
#include <linux/input.h>
#include <linux/serio.h>
#include <linux/libps2.h>
#include <linux/slab.h>
#include <asm/hypervisor.h>

#include "psmouse.h"
#include "vmmouse.h"

