#include "test_framework.h"
#include "../src/cli.h"

BEGIN_TESTING

// parseArgs
TEST("parseArgsNoFiles") {
    // Arrange
    int argc = 1;
    char* argv[] = { "./pandi-cc" };

    // Act
    CliArgs result = parseArgs(argc, argv);

    // Assert
    assert(result.fileCount == 0);
    assert(result.files == NULL);
}

END_TESTING
