cmake_minimum_required(VERSION 4.0)
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g3 -std=c23")
project(clox)

set(HEADER FILES
    common.h
    compiler.h
    chunk.h
    debug.h
    memory.h
    object.h
    scanner.h
    table.h
    value.h
    vm.h
)
set(SOURCES 
    chunk.c
    compiler.c
    debug.c
    memory.c
    main.c
    object.c
    scanner.c
    table.c
    value.c
    vm.c
)

add_executable(clox ${SOURCES})
