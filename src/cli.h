#pragma once
#include <__stddef_size_t.h>
#include <stdbool.h>

typedef struct {
    bool lex;
    bool parse;
    bool codegen;
    bool assembly;
    size_t fileCount;
    char** files;
} CliArgs;

CliArgs parseArgs(int argc, char* argv[]);
