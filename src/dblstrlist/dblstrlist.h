struct snode;
struct dblstrlist;

struct snode *list_search(struct dblstrlist *l, char *k);
struct snode *list_insert(struct dblstrlist *l, char *k);
