#include <stdlib.h>
#include <stdio.h>

#include "dblstrlist.h"


int main(int argc, char* argv[]) {
    struct dblstrlist *l = NULL;
    struct snode *x = NULL;
    int i;
    
    l = dblstrlistalloc(); 
    for (i = 0; i < 5; ++i) {
        x = smalloc();

        if (x != NULL) {
            switch(i) {
            case 0:
                sassign(x, "you?");
                break;
            case 1:
                sassign(x, "are");
                break;
            case 2:
                sassign(x, "how");
                break;
            case 3:
                sassign(x, "world");
                break;
            case 4:
                sassign(x, "Hello");
                break;
            default:
                break;
            }

            if (x != NULL)
                list_insert(l, x);
        }
    }

    while (1) {
        if ((x = list_search(l, "Hello")) != NULL)
            list_delete(l, x);
        else if ((x = list_search(l, "world")) != NULL)
            list_delete(l, x);
        else if ((x = list_search(l, "how")) != NULL)
            list_delete(l, x);
        else if ((x = list_search(l, "are ")) != NULL)
            list_delete(l, x);
        else if ((x = list_search(l, "you?")) != NULL)
            list_delete(l, x);
        else
            break;

        printf("%s ", get_key(x));

        if (x != NULL)
            free(x);
    }

    printf("\n");

    free(l);

    return 0;
}
