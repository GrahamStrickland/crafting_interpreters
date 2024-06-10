#include <stdlib.h>

#include "dblstrlist.h"


int main(int argc, char* argv[]) {
    struct dblstrlist *l = dblstrlistalloc(); 
    struct snode *x = NULL;
    int i;
    
    for (i = 1; i <= 5; ++i) {
        x = smalloc();

        if (x != NULL) {
            switch(i) {
            case 0:
                sassign(x, "Hello");
                break;
            case 1:
                sassign(x, "world");
                break;
            case 2:
                sassign(x, "how");
                break;
            case 3:
                sassign(x, "are");
                break;
            case 4:
                sassign(x, "you?");
                break;
            default:
                break;
            }
            list_insert(l, x);
        }
    }

    return 0;
}
