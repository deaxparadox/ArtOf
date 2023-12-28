#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

#define MAX_STRING_LIST_LEN = 200
#define MAX_STRING_LEN 200

char *longestCommonPrefix(char **strs, int strsSize);

int main()
{
    int size = 3;
    char *strs[] = {"flower", "flow", "flight"};

    char *lcp = longestCommonPrefix(strs, size);
    printf("%s\n", lcp);

    if (lcp)
    {
        printf("freeing lcp\n");
    }
    free(lcp);


    return 0;
}

char *longestCommonPrefix(char **strs, int strsSize)
{
    char *pref = calloc(200, sizeof(char));
    memset(pref, 0, 200 * sizeof(char));
    char t;
    for (int i=0; i<strlen(strs[0]); i++) {
        t = strs[0][i];
        for (int j=1; j<strsSize; j++) {
            if (t != strs[j][i])
            {
                return pref;
            }
        }
        pref[i] = t;
    }
    return pref;
}
