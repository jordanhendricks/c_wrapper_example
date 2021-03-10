#include "num_wrap.h"
#include <stdio.h>

/*
 * Change the definition of `raw_to_num` to try out different interfaces.
 */
//#define raw_to_num    raw_to_num_box
//#define raw_to_num    raw_to_num_no_box
#define raw_to_num    ref_to_num

// suggested convention:
// *mut as an arg: passing ownership from C to Rust
// &mut as an arg: Rust borrows arg, C can continue to use it

int main() {
    CNumWrapper *n1 = raw_numwrapper(1);
    CNumWrapper *n2 = raw_numwrapper(2);
    CNumWrapper *n3 = raw_numwrapper(1);

    printf("n1: raw_numwrapper=%p\n", n1);
    printf("n2: raw_numwrapper=%p\n", n2);
    printf("n3: raw_numwrapper=%p\n", n3);

    printf("n1: num=%d\n", raw_to_num(n1));
    printf("n2: num=%d\n", raw_to_num(n2));
    printf("n3: num=%d\n", raw_to_num(n3));
    printf("n3: num=%d\n", raw_to_num(n3));
}
