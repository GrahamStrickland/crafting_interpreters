cmake_minimum_required(VERSION 3.16)
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -std=c17")
project(clox)

set(HEADER FILES
    common.h
    chunk.h
    memory.h
)
set(SOURCES 
    chunk.c
    memory.c
    main.c
)

add_executable(clox ${SOURCES})
