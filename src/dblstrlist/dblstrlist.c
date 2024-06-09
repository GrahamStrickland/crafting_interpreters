#include <string.h>

#include "dblstrlist.h"

struct snode {
    char *key;
    struct snode *prev;
    struct snode *next;
};

struct dblstrlist {
    struct snode *head;
};

/* List-Search(L, k) finds the first element with key k in list l,
    returning a pointer to this element */
struct snode *list_search(struct dblstrlist *l, char *k) {
    struct snode *x = l->head;

    while (x != NULL && !strcmp(x->key, k))
        x = x->next;
    
    return x;
}
