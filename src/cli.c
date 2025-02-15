#include "cli.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

CliArgs parseArgs(int argc, char* argv[]) {
    CliArgs parsedArgs;
    parsedArgs.lex = false;
    parsedArgs.parse = false;
    parsedArgs.codegen = false;
    parsedArgs.assembly = false;
    parsedArgs.fileCount = 0;
    parsedArgs.files = NULL;


    size_t i;
    for(i = 1; i < argc && argv[i][0] == '-'; ++i) {
        if(strcmp(argv[i], "--lex") == 0) {
            parsedArgs.lex = true;
        } else if(strcmp(argv[i], "--parse") == 0) {
            parsedArgs.parse = true;
        } else if(strcmp(argv[i], "--codegen") == 0) {
            parsedArgs.codegen = true;
        } else if(strcmp(argv[i], "-S") == 0) {
            parsedArgs.assembly = true;
        } else {
            fprintf(stderr, "ERROR: Unrecognised cli flag '%s'\nUsage: %s [--lex|--parse|--codegen|-S] <input.c>", argv[i], argv[0]);
            exit(1);
        }
    }
    if(i < argc) {
        parsedArgs.fileCount = argc - i;
        parsedArgs.files = argv + i;

        if(parsedArgs.fileCount > 1) {
            fprintf(stderr, "ERROR: Too many input file provided: %zu, when only 1 is supported.\nUsage: %s [--lex|--parse|--codegen|-S] <input.c>", parsedArgs.fileCount, argv[0]);
            exit(1);
        }
    } else {
        fprintf(stderr, "ERROR: No input file provided.\nUsage: %s [--lex|--parse|--codegen|-S] <input.c>", argv[0]);
        exit(1);
    }

    return parsedArgs;
}
