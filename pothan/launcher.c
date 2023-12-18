#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <limits.h>

#include <windows.h>

typedef struct {
    char *str;
    int strlen;
} string;

void string_init(string *a) {
    a->str = (char*) malloc(sizeof(char));
    a->strlen = 0;
}

void string_push(string *a, const char *b)
{
    for (int i = 0;; i++)
    {
        if (b[i] != '\0')
        {
            a->str = (char *)realloc(a->str, sizeof(char) * (a->strlen + 1));
            a->str[a->strlen] = b[i];
            a->strlen++;
        }
        else
            break;
    }
}

char *to_str(string *a) {
    char *str;
    str = (char*) malloc(sizeof(char)*(a->strlen + 1));
    strcpy(str, a->str);
    str[a->strlen] = '\0';
    return str;
}

char string_deinit(string *a) {
    free(a->str);
}

int WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nShowCmd)
{
    char rootdir[PATH_MAX];
    getcwd(rootdir, sizeof(rootdir));

    string app_data;
    string_init(&app_data);
    string_push(&app_data, "APPDATA=");
    string_push(&app_data, rootdir);
    string_push(&app_data, "\\modules");

    putenv(to_str(&app_data));

    string_deinit(&app_data);

    STARTUPINFO si;
    PROCESS_INFORMATION pi;
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    ZeroMemory(&pi, sizeof(pi));

    CreateProcess(
        NULL,
        "xiphos\\bin\\xiphos.exe",
        NULL,
        NULL,
        FALSE,
        0x08000000,
        NULL,
        NULL,
        &si,
        &pi
    );

    return 0;
}