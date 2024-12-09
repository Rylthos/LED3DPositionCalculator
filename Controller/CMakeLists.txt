cmake_minimum_required(VERSION 3.18)

project(LEDController)

set(CMAKE_CXX_STANDARD 20)

# Change Output Directory
set(CMAKE_BINARY_DIR "/bin")
set(OUTPUT_DIRECTORY "${PROJECT_SOURCE_DIR}${CMAKE_BINARY_DIR}")
if(CMAKE_BUILD_TYPE MATCHES Debug)
  message("Building Debug")
  set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${OUTPUT_DIRECTORY}/debug")
  set(linker "${linker} -g")
  add_compile_definitions(DEBUG_MODE)
else()
  message("Building Release")
  set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${OUTPUT_DIRECTORY}/release")
endif()

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wno-unknown-pragmas")

add_subdirectory(vendor)

file(GLOB_RECURSE cppFiles CONFIGURE_DEPENDS "src/*.cpp")
add_executable(${PROJECT_NAME} ${cppFiles})

# Add libraries and include directories
target_include_directories(${PROJECT_NAME} PUBLIC)

target_link_libraries(
  ${PROJECT_NAME}
  PRIVATE ${linker}
  PUBLIC glm)
