#pragma once

#include <stdbool.h>
#include <string.h>
#include <assert.h>

#define BEGIN_TESTING int main(int argc, char* argv[]) {
#define END_TESTING return 0;}
#define TEST(TEST_NAME) if(runTest(TEST_NAME, argc, argv))

bool runTest(const char* testName, int argc, char* argv[]) {
    // No specific tests are mentioned, run all tests
    if(argc == 1) {
        return true;
    }
    
    // If the test is specified in the args, run it
    for(int i = 1; i < argc; ++i) {
        if(strcmp(testName, argv[i]) == 0) {
            return true;
        }
    }

    // Test isn't specified in the args, not running it
    return false;
}
