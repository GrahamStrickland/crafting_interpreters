cmake_minimum_required(VERSION 3.16)
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g3 -std=c17")
project(clox)

set(HEADER FILES
    common.h
    compiler.h
    chunk.h
    debug.h
    memory.h
    scanner.h
    value.h
    vm.h
)
set(SOURCES 
    chunk.c
    compiler.c
    debug.c
    memory.c
    main.c
    scanner.c
    value.c
    vm.c
)

add_executable(clox ${SOURCES})
