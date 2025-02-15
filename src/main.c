#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "cli.h"

void changeFileNameExtension(char *dest, size_t dest_size, char *fileName,
                             char ext) {
  strcpy(dest, fileName);
  dest[dest_size - 1] = ext;
}

void removeFileNameExtension(char *dest, size_t dest_size, char *fileName) {
  strcpy(dest, fileName);
  dest[dest_size - 2] = '\0';
}

void compilerDriver(char *fileName) {
  size_t fileNameLength = strlen(fileName);

  char processedFileName[fileNameLength];
  changeFileNameExtension(processedFileName, fileNameLength, fileName, 'i');

  char assemblyFileName[fileNameLength];
  changeFileNameExtension(assemblyFileName, fileNameLength, fileName, 's');

  char outputFileName[fileNameLength];
  removeFileNameExtension(outputFileName, fileNameLength, fileName);

  char cmd[512];
  sprintf(cmd, "gcc -E -P %s -o %s", fileName, processedFileName);
  printf("Executing the preprocessor: '%s'\n", cmd);
  system(cmd);

  // TODO: Replace with my own compilation logic
  sprintf(cmd, "gcc -S %s -o %s", processedFileName, assemblyFileName);
  printf("Executing the compiler: '%s'\n", cmd);
  system(cmd);

  sprintf(cmd, "gcc %s -o %s", assemblyFileName, outputFileName);
  printf("Executing the linker: '%s'\n", cmd);
  system(cmd);
}

int main(int argc, char *argv[]) {
    CliArgs cliArgs = parseArgs(argc, argv);

  compilerDriver(cliArgs.files[0]);

  return 0;
}
