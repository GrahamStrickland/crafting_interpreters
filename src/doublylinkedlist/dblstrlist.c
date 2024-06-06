#include <string.h>

#include "dblstrlist.h"

// List-Search(L, k) finds
struct snode *list_search(struct dblstrlist *l, char *k) {
    struct snode *x = l->head;

    while (x != NULL && !strcmp(x->key, k))
        x = x->next;
    
    return x;
}

